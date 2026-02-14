use actix_web::{web, App, HttpServer};
use futures_util::{SinkExt, StreamExt};
use kjxlkj_auth::{hash_password, new_csrf_token, new_session_id, session_expiry};
use kjxlkj_db::repos;
use kjxlkj_db::repos::automation::CreateAutomationRuleInput;
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use serde_json::json;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn ws_subscribe_patch_replay_and_conflict_flow() {
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
        &format!("owner-ws-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner and workspace");

    let (stream, _) = repos::notes::create_note(
        &pool,
        owner.id,
        repos::notes::CreateNoteInput {
            workspace_id: workspace.id,
            project_id: None,
            title: "First note".to_owned(),
            note_kind: "markdown".to_owned(),
            access_scope: "workspace".to_owned(),
            markdown: "Hello world".to_owned(),
        },
    )
    .await
    .expect("create note");

    repos::automation::create_rule(
        &pool,
        CreateAutomationRuleInput {
            workspace_id: workspace.id,
            trigger: "note_patched".to_owned(),
            condition_json: json!({}),
            action_json: json!({ "kind": "noop" }),
            enabled: true,
            actor_id: owner.id,
        },
    )
    .await
    .expect("create automation rule");

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
        .expect("send subscribe_workspace");

    let workspace_subscribed = wait_for_type(&mut socket, "subscribed").await;
    assert_eq!(workspace_subscribed["stream_id"], format!("workspace:{}", workspace.id));

    socket
        .send(Message::Text(
            json!({
                "type": "subscribe_note",
                "note_id": stream.id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send subscribe_note");

    let subscribed = wait_for_type(&mut socket, "subscribed").await;
    assert_eq!(subscribed["stream_id"], format!("note:{}", stream.id));

    socket
        .send(Message::Text(
            json!({
                "type": "apply_patch",
                "note_id": stream.id,
                "base_version": 1,
                "patch_ops": [
                    { "retain": 6 },
                    { "delete": 5 },
                    { "insert": "notes" }
                ],
                "idempotency_key": "ws-idem-01",
                "client_ts": null
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send apply_patch");

    let committed = wait_for_type(&mut socket, "patch_committed").await;
    let committed_event_seq = committed["event_seq"].as_i64().expect("event_seq as i64");
    let committed_version = committed["version"].as_i64().expect("version as i64");

    let automation_event = wait_for_type(&mut socket, "automation_event").await;
    let automation_event_seq = automation_event["event_seq"]
        .as_i64()
        .expect("automation event_seq as i64");
    assert_eq!(automation_event["workspace_id"], workspace.id.to_string());
    assert_eq!(automation_event["status"], "succeeded");
    assert!(automation_event["run_id"].as_str().is_some());
    assert!(automation_event["payload"]["operation_preview"].is_array());
    assert!(automation_event["payload"]["operation_applied"].is_array());
    assert!(automation_event["payload"]["operation_rejected"].is_array());

    let mut ordering_request = ws_url
        .clone()
        .into_client_request()
        .expect("build ordering request");
    ordering_request.headers_mut().insert(
        "Cookie",
        HeaderValue::from_str(&format!("kjxlkj_session={session_id}"))
            .expect("valid ordering cookie header"),
    );
    let (mut ordering_socket, _) = connect_async(ordering_request)
        .await
        .expect("connect ordering websocket");

    ordering_socket
        .send(Message::Text(
            json!({
                "type": "subscribe_workspace",
                "workspace_id": workspace.id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send ordering subscribe_workspace");

    let ordering_subscribed = wait_for_type(&mut ordering_socket, "subscribed").await;
    assert_eq!(ordering_subscribed["replay_cursor"].as_i64(), Some(0));

    let mut note_patched_seq: Option<i64> = None;
    let mut automation_queued_seq: Option<i64> = None;
    let mut automation_running_seq: Option<i64> = None;
    let mut automation_succeeded_seq: Option<i64> = None;

    for _ in 0..32 {
        if note_patched_seq.is_some()
            && automation_queued_seq.is_some()
            && automation_running_seq.is_some()
            && automation_succeeded_seq.is_some()
        {
            break;
        }

        let event = wait_for_stream_event(&mut ordering_socket).await;
        let Some(event_type) = event["event_type"].as_str() else {
            continue;
        };
        let Some(event_seq) = event["event_seq"].as_i64() else {
            continue;
        };

        match event_type {
            "note_patched" => note_patched_seq = Some(event_seq),
            "automation_run_queued" => automation_queued_seq = Some(event_seq),
            "automation_run_running" => automation_running_seq = Some(event_seq),
            "automation_run_succeeded" => automation_succeeded_seq = Some(event_seq),
            _ => {}
        }
    }

    let note_patched_seq = note_patched_seq.expect("note_patched event present");
    let automation_queued_seq = automation_queued_seq.expect("automation_run_queued event present");
    let automation_running_seq = automation_running_seq.expect("automation_run_running event present");
    let automation_succeeded_seq =
        automation_succeeded_seq.expect("automation_run_succeeded event present");

    assert!(note_patched_seq < automation_queued_seq);
    assert!(automation_queued_seq < automation_running_seq);
    assert!(automation_running_seq < automation_succeeded_seq);

    ordering_socket
        .close(None)
        .await
        .expect("close ordering websocket");

    socket
        .send(Message::Text(
            json!({
                "type": "ack",
                "stream_id": format!("workspace:{}", workspace.id),
                "event_seq": automation_event_seq,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("ack workspace cursor");

    socket
        .send(Message::Text(
            json!({
                "type": "ack",
                "stream_id": format!("workspace:{}", workspace.id),
                "event_seq": automation_event_seq.saturating_sub(1),
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send stale workspace ack");

    let stale_ack_error = wait_for_type(&mut socket, "error").await;
    assert_eq!(stale_ack_error["code"], "STALE_CURSOR");
    assert_eq!(
        stale_ack_error["stream_id"],
        format!("workspace:{}", workspace.id)
    );

    socket
        .send(Message::Text(
            json!({
                "type": "apply_patch",
                "note_id": stream.id,
                "base_version": 1,
                "patch_ops": [ { "retain": 1 } ],
                "idempotency_key": "ws-idem-01",
                "client_ts": null
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send idempotent retransmit");

    let replayed = wait_for_type(&mut socket, "patch_committed").await;
    assert_eq!(replayed["event_seq"].as_i64(), Some(committed_event_seq));
    assert_eq!(replayed["version"].as_i64(), Some(committed_version));

    socket
        .send(Message::Text(
            json!({
                "type": "apply_patch",
                "note_id": stream.id,
                "base_version": 1,
                "patch_ops": [ { "retain": 1 } ],
                "idempotency_key": "ws-idem-conflict",
                "client_ts": null
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send stale patch");

    let rejected = wait_for_type(&mut socket, "patch_rejected").await;
    assert_eq!(rejected["expected_version"].as_i64(), Some(1));
    assert!(rejected["current_version"].as_i64().unwrap_or_default() >= 2);

    socket
        .send(Message::Text(
            json!({
                "type": "ack",
                "stream_id": format!("note:{}", stream.id),
                "event_seq": committed_event_seq,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("ack cursor");

    socket
        .send(Message::Text(
            json!({
                "type": "ack",
                "stream_id": format!("note:{}", stream.id),
                "event_seq": committed_event_seq.saturating_sub(1),
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send stale note ack");

    let stale_note_ack_error = wait_for_type(&mut socket, "error").await;
    assert_eq!(stale_note_ack_error["code"], "STALE_CURSOR");
    assert_eq!(stale_note_ack_error["stream_id"], format!("note:{}", stream.id));

    let future_workspace_seq: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(seq), 0) + 1 FROM workspace_events WHERE workspace_id = $1",
    )
    .bind(workspace.id)
    .fetch_one(&pool)
    .await
    .expect("compute next workspace seq");

    sqlx::query(
        "INSERT INTO workspace_events (workspace_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(workspace.id)
    .bind(future_workspace_seq)
    .bind("future_workspace_event")
    .bind(json!({ "compat": true }))
    .bind(owner.id)
    .execute(&pool)
    .await
    .expect("insert unknown workspace event");

    socket.close(None).await.expect("close first websocket");

    let mut request_reconnect = ws_url
        .clone()
        .into_client_request()
        .expect("build reconnect request");
    request_reconnect.headers_mut().insert(
        "Cookie",
        HeaderValue::from_str(&format!("kjxlkj_session={session_id}"))
            .expect("valid reconnect cookie header"),
    );
    let (mut reconnect, _) = connect_async(request_reconnect)
        .await
        .expect("reconnect websocket");

    reconnect
        .send(Message::Text(
            json!({
                "type": "subscribe_note",
                "note_id": stream.id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send subscribe_note on reconnect");

    let replay_subscribed = wait_for_type(&mut reconnect, "subscribed").await;
    assert_eq!(
        replay_subscribed["replay_cursor"].as_i64(),
        Some(committed_event_seq)
    );

    reconnect
        .send(Message::Text(
            json!({
                "type": "subscribe_workspace",
                "workspace_id": workspace.id,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send subscribe_workspace on reconnect");

    let workspace_replay_subscribed = wait_for_type(&mut reconnect, "subscribed").await;
    assert_eq!(
        workspace_replay_subscribed["stream_id"],
        format!("workspace:{}", workspace.id)
    );
    assert_eq!(
        workspace_replay_subscribed["replay_cursor"].as_i64(),
        Some(automation_event_seq)
    );

    let unknown_event = wait_for_type(&mut reconnect, "workspace_event").await;
    assert_eq!(unknown_event["event_type"], "future_workspace_event");
    assert_eq!(unknown_event["payload"]["compat"], true);
    assert_eq!(
        unknown_event["event_seq"].as_i64(),
        Some(future_workspace_seq as i64)
    );

    reconnect.close(None).await.expect("close reconnect websocket");
    server_handle.stop(true).await;
}

async fn wait_for_type(
    socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    message_type: &str,
) -> serde_json::Value {
    for _ in 0..16 {
        let next = timeout(Duration::from_secs(5), socket.next())
            .await
            .expect("websocket receive timeout")
            .expect("websocket closed unexpectedly")
            .expect("websocket receive error");

        if let Message::Text(text) = next {
            let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid json ws message");
            if parsed["type"] == message_type {
                return parsed;
            }
        }
    }
    panic!("expected websocket message type '{message_type}' not observed in time")
}

async fn wait_for_stream_event(
    socket: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
) -> serde_json::Value {
    for _ in 0..16 {
        let next = timeout(Duration::from_secs(5), socket.next())
            .await
            .expect("websocket receive timeout")
            .expect("websocket closed unexpectedly")
            .expect("websocket receive error");

        if let Message::Text(text) = next {
            let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid json ws message");
            if parsed["type"] == "workspace_event" || parsed["type"] == "automation_event" {
                return parsed;
            }
        }
    }
    panic!("expected websocket workspace/automation stream event not observed in time")
}
