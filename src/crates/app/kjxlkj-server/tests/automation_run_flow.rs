use actix_web::{web, App, HttpServer};
use futures_util::{SinkExt, StreamExt};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use reqwest::StatusCode;
use serde_json::json;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn automation_run_idempotency_status_and_ws_event_replay() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
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
        &format!("owner-auto-run-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-auto-run-{token}"),
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

    let create_rule = client
        .post(format!("{base_url}/api/automation/rules"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "trigger": "note_patched",
            "condition_json": {"always": true},
            "action_json": {"kind": "noop"},
            "enabled": true
        }))
        .send()
        .await
        .expect("create automation rule request");
    assert_eq!(create_rule.status(), StatusCode::CREATED);

    let create_rule_body: serde_json::Value = create_rule
        .json()
        .await
        .expect("parse create rule response");
    let rule_id = create_rule_body["rule"]["id"]
        .as_str()
        .expect("rule id string")
        .to_owned();

    let create_note = client
        .post(format!("{base_url}/api/notes"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "project_id": null,
            "title": "Automation target",
            "note_kind": "markdown",
            "access_scope": "workspace",
            "markdown": "hello"
        }))
        .send()
        .await
        .expect("create note request");
    assert_eq!(create_note.status(), StatusCode::CREATED);

    let note_body: serde_json::Value = create_note.json().await.expect("parse create note response");
    let note_id = note_body["note_id"]
        .as_str()
        .expect("note id string")
        .to_owned();

    let patch_once = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"delete": 5}, {"insert": "hello automation"}],
            "idempotency_key": "auto-run-idempotent-key"
        }))
        .send()
        .await
        .expect("first patch request");
    assert_eq!(patch_once.status(), StatusCode::OK);

    let patch_twice = client
        .patch(format!("{base_url}/api/notes/{note_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "base_version": 1,
            "patch_ops": [{"retain": 1}],
            "idempotency_key": "auto-run-idempotent-key"
        }))
        .send()
        .await
        .expect("duplicate patch request");
    assert_eq!(patch_twice.status(), StatusCode::OK);

    let rule_uuid = uuid::Uuid::parse_str(&rule_id).expect("parse rule id");
    let runs: Vec<(uuid::Uuid, String, String)> = sqlx::query_as(
        "SELECT id, status, triggering_event_id
         FROM automation_runs
         WHERE rule_id = $1
         ORDER BY created_at ASC",
    )
    .bind(rule_uuid)
    .fetch_all(&pool)
    .await
    .expect("query automation runs");

    assert_eq!(runs.len(), 1, "idempotent triggering must produce one run");
    assert_eq!(runs[0].1, "succeeded");

    let run_id = runs[0].0;
    let run_status = client
        .get(format!("{base_url}/api/automation/runs/{run_id}"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .query(&[("workspace_id", workspace.id.to_string())])
        .send()
        .await
        .expect("get automation run status request");
    assert_eq!(run_status.status(), StatusCode::OK);

    let run_status_body: serde_json::Value = run_status
        .json()
        .await
        .expect("parse run status body");
    assert_eq!(run_status_body["run"]["status"], json!("succeeded"));

    let list_runs = client
        .get(format!("{base_url}/api/automation/runs"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .query(&[("workspace_id", workspace.id.to_string()), ("limit", "10".to_owned())])
        .send()
        .await
        .expect("list automation runs request");
    assert_eq!(list_runs.status(), StatusCode::OK);

    let listed_runs: serde_json::Value = list_runs
        .json()
        .await
        .expect("parse listed runs response");
    assert!(listed_runs["runs"]
        .as_array()
        .map(|runs| runs.iter().any(|run| run["id"] == json!(run_id.to_string())))
        .unwrap_or(false));

    let launch_run = client
        .post(format!("{base_url}/api/automation/rules/{rule_id}/launch"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "workspace_id": workspace.id,
            "note_id": note_id,
        }))
        .send()
        .await
        .expect("launch automation run request");
    assert_eq!(launch_run.status(), StatusCode::OK);

    let launch_run_body: serde_json::Value = launch_run
        .json()
        .await
        .expect("parse launch run response");
    let launched_run_id = launch_run_body["run"]["id"]
        .as_str()
        .expect("launched run id string")
        .to_owned();
    assert_eq!(launch_run_body["run"]["status"], json!("succeeded"));

    let review_run = client
        .post(format!("{base_url}/api/automation/runs/{launched_run_id}/review"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "apply": false,
            "decisions": [],
        }))
        .send()
        .await
        .expect("review automation run request");
    assert_eq!(review_run.status(), StatusCode::OK);

    let review_run_body: serde_json::Value = review_run
        .json()
        .await
        .expect("parse review run response");
    assert_eq!(review_run_body["run"]["result_json"]["review"]["apply"], json!(false));

    let invalid_review = client
        .post(format!("{base_url}/api/automation/runs/{launched_run_id}/review"))
        .header("Cookie", format!("kjxlkj_session={session_id}"))
        .header("x-csrf-token", &csrf_token)
        .json(&json!({
            "apply": false,
            "decisions": [{
                "operation_id": "op-invalid",
                "decision": "maybe"
            }],
        }))
        .send()
        .await
        .expect("invalid review decision request");
    assert_eq!(invalid_review.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let reviewed_events: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM workspace_events WHERE workspace_id = $1 AND event_type = 'automation_run_reviewed'",
    )
    .bind(workspace.id)
    .fetch_one(&pool)
    .await
    .expect("count automation_run_reviewed events");
    assert!(reviewed_events >= 1);

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
                "type": "subscribe_workspace",
                "workspace_id": workspace.id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscribe workspace stream");

    let mut found_automation_event = false;
    for _ in 0..32 {
        let next = timeout(Duration::from_secs(5), socket.next())
            .await
            .expect("websocket receive timeout")
            .expect("websocket closed unexpectedly")
            .expect("websocket receive error");

        if let Message::Text(text) = next {
            let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid websocket json");
            let is_workspace_automation = parsed["type"] == "workspace_event"
                && parsed["event_type"]
                    .as_str()
                    .map(|value| value.starts_with("automation_run_"))
                    .unwrap_or(false);

            let is_typed_automation = parsed["type"] == "automation_event"
                && parsed["event_type"]
                    .as_str()
                    .map(|value| value.starts_with("automation_run_"))
                    .unwrap_or(false);

            if is_workspace_automation || is_typed_automation {
                found_automation_event = true;
                break;
            }
        }
    }

    assert!(
        found_automation_event,
        "expected replayed workspace automation event after subscribe"
    );

    socket.close(None).await.expect("close websocket");
    server_handle.stop(true).await;
}
