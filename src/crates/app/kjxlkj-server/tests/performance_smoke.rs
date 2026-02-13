use actix_web::{web, App, HttpResponse, HttpServer};
use futures_util::{SinkExt, StreamExt};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;
use std::time::Instant;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn perf_01_and_perf_02_smoke_baseline() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(6)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");
    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-perf-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-perf-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let address = listener.local_addr().expect("read bound addr");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");
    let base_url = format!("http://{}", address);

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Perf note",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "seed"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let create_body: serde_json::Value = create_note.json().await.expect("parse create note body");
    let note_id = create_body["note_id"].as_str().expect("note id string").to_owned();
    let mut version = create_body["version"].as_i64().expect("version as i64") as i32;

    let mut read_latencies_ms: Vec<u128> = Vec::new();
    let mut write_latencies_ms: Vec<u128> = Vec::new();

    for _ in 0..30 {
        let start = Instant::now();
        let list = client
            .get(format!("{base_url}/api/notes"))
            .header("Cookie", format!("kjxlkj_session={session_id}"))
            .query(&[("workspace_id", workspace.id.to_string())])
            .send()
            .await
            .expect("list notes request");
        assert_eq!(list.status(), StatusCode::OK);
        read_latencies_ms.push(start.elapsed().as_millis());

        let start = Instant::now();
        let patch = client
            .patch(format!("{base_url}/api/notes/{note_id}"))
            .header("Cookie", format!("kjxlkj_session={session_id}"))
            .header("x-csrf-token", &csrf_token)
            .json(&json!({
                "base_version": version,
                "patch_ops": [{"delete": 4}, {"insert": "seed"}],
                "idempotency_key": format!("perf-http-{version}")
            }))
            .send()
            .await
            .expect("patch notes request");
        assert_eq!(patch.status(), StatusCode::OK);
        let patch_body: serde_json::Value = patch.json().await.expect("parse patch body");
        version = patch_body["version"].as_i64().expect("patch version as i64") as i32;
        write_latencies_ms.push(start.elapsed().as_millis());
    }

    let read_p95 = percentile_95(&read_latencies_ms);
    let write_p95 = percentile_95(&write_latencies_ms);
    assert!(read_p95 < 2000, "expected read p95 under 2000ms, got {read_p95}");
    assert!(write_p95 < 2000, "expected write p95 under 2000ms, got {write_p95}");

    let ws_url = format!("ws://{}/ws", address);
    let mut request = ws_url
        .clone()
        .into_client_request()
        .expect("build ws request");
    request.headers_mut().insert(
        "Cookie",
        HeaderValue::from_str(&format!("kjxlkj_session={session_id}"))
            .expect("valid cookie header"),
    );
    let (mut socket, _) = connect_async(request).await.expect("connect websocket");

    socket
        .send(Message::Text(
            json!({
                "type": "subscribe_note",
                "note_id": note_id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscribe note stream");

    let _ = wait_for_type(&mut socket, "subscribed").await;

    let mut last_seq = 0_i64;
    for idx in 0..20 {
        socket
            .send(Message::Text(
                json!({
                    "type": "apply_patch",
                    "note_id": note_id,
                    "base_version": version,
                    "patch_ops": [{"delete": 4}, {"insert": "seed"}],
                    "idempotency_key": format!("perf-ws-{idx}"),
                    "client_ts": null
                })
                .to_string()
                .into(),
            ))
            .await
            .expect("send apply_patch over websocket");

        let committed = wait_for_type(&mut socket, "patch_committed").await;
        let seq = committed["event_seq"].as_i64().expect("event_seq as i64");
        let new_version = committed["version"].as_i64().expect("version as i64") as i32;
        assert!(seq > last_seq, "expected increasing WS event sequence");
        last_seq = seq;
        version = new_version;
    }

    socket.close(None).await.expect("close websocket");
    server_handle.stop(true).await;
}

#[tokio::test]
async fn perf_03_librarian_throughput_and_retry_envelope() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");
    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-perf03-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-perf03-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let session_id = new_session_id();
    let csrf_token = new_csrf_token();
    repos::auth::create_session(&pool, session_id, owner.id, &csrf_token, session_expiry(7))
        .await
        .expect("create owner session");

    let provider_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind provider server");
    let provider_address = provider_listener
        .local_addr()
        .expect("read provider address");

    let provider_server = HttpServer::new(|| {
        App::new().route(
            "/v1/chat/completions",
            web::post().to(|| async {
                HttpResponse::Ok().json(json!({
                    "choices": [{
                        "message": {
                            "content": "<librarian_response><request_id>perf03</request_id><status>ok</status><summary>batch</summary><operations></operations><warnings></warnings></librarian_response>"
                        }
                    }]
                }))
            }),
        )
    })
    .listen(provider_listener)
    .expect("listen provider")
    .run();

    let provider_handle = provider_server.handle();
    let _provider_task = tokio::spawn(provider_server);

    let state = AppState::new(pool.clone(), false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind app server");
    let address = listener.local_addr().expect("read app address");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .listen(listener)
    .expect("listen app")
    .run();

    let server_handle = server.handle();
    let _server_task = tokio::spawn(server);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("build reqwest client");

    let base_url = format!("http://{}", address);

    let create_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {
                "kind": "librarian_structure",
                "protocol": "xml_attrless",
                "provider": {
                    "provider_kind": "openrouter",
                    "model": "openrouter/perf03-model",
                    "base_url": format!("http://{provider_address}/v1/chat/completions"),
                    "timeout_ms": 400,
                    "retry_limit": 1
                },
                "plan": {
                    "goal": "Perf batch",
                    "scope": "workspace",
                    "taxonomy_json": {"topics": ["perf"]},
                    "style_profile": "concise",
                    "strict_mode": false,
                    "max_operations": 8,
                    "allow_delete": false
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .expect("create perf03 rule request");
    assert_eq!(create_rule.status(), StatusCode::CREATED);

    let create_rule_body: serde_json::Value = create_rule
        .json()
        .await
        .expect("parse perf03 rule response");
    let rule_id = uuid::Uuid::parse_str(
        create_rule_body["rule"]["id"]
            .as_str()
            .expect("perf03 rule id"),
    )
    .expect("parse perf03 rule id");

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Perf03 note",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "seed"
        }))
        .send()
        .await
        .expect("create perf03 note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let create_note_body: serde_json::Value = create_note
        .json()
        .await
        .expect("parse perf03 note response");
    let note_id = create_note_body["note_id"]
        .as_str()
        .expect("perf03 note id")
        .to_owned();
    let mut version = create_note_body["version"].as_i64().expect("perf03 version") as i32;

    let start = Instant::now();
    for idx in 0..50 {
        let patch = client
            .patch(format!("{base_url}/api/notes/{note_id}"))
            .header("Cookie", format!("kjxlkj_session={session_id}"))
            .header("x-csrf-token", &csrf_token)
            .json(&json!({
                "base_version": version,
                "patch_ops": [{"delete": 4}, {"insert": "seed"}],
                "idempotency_key": format!("perf03-{idx}")
            }))
            .send()
            .await
            .expect("perf03 patch request");
        assert_eq!(patch.status(), StatusCode::OK);

        let patch_body: serde_json::Value = patch.json().await.expect("parse perf03 patch response");
        version = patch_body["version"].as_i64().expect("perf03 updated version") as i32;
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 90,
        "expected PERF-03 batch under 90s, got {}s",
        elapsed.as_secs()
    );

    let metrics: (i64, Option<i64>, Option<f64>) = sqlx::query_as(
        "SELECT
            COUNT(*) AS total_runs,
            SUM(CASE WHEN status = 'failed' THEN 1 ELSE 0 END) AS failed_runs,
            AVG(COALESCE((result_json->>'attempts')::float, 0)) AS avg_attempts
         FROM automation_runs
         WHERE rule_id = $1",
    )
    .bind(rule_id)
    .fetch_one(&pool)
    .await
    .expect("query perf03 run metrics");

    assert_eq!(metrics.0, 50, "expected one run per patched source event");
    assert_eq!(metrics.1.unwrap_or(0), 0, "expected zero parse/provider failures in perf batch");

    let average_attempts = metrics.2.unwrap_or(0.0);
    assert!(
        average_attempts <= 2.0,
        "expected retry envelope avg attempts <= 2, got {average_attempts}"
    );

    server_handle.stop(true).await;
    provider_handle.stop(true).await;
}

fn percentile_95(values: &[u128]) -> u128 {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let index = ((sorted.len() as f64) * 0.95).ceil() as usize;
    let idx = index.saturating_sub(1).min(sorted.len().saturating_sub(1));
    sorted[idx]
}

async fn wait_for_type(
    socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    message_type: &str,
) -> serde_json::Value {
    for _ in 0..48 {
        let next = timeout(Duration::from_secs(5), socket.next())
            .await
            .expect("websocket receive timeout")
            .expect("websocket closed unexpectedly")
            .expect("websocket receive error");

        if let Message::Text(text) = next {
            let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid websocket json");
            if parsed["type"] == message_type {
                return parsed;
            }
        }
    }

    panic!("expected websocket message type '{message_type}' not observed in time")
}
