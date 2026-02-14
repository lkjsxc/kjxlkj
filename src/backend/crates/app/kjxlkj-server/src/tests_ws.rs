use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    model::{NoteRecord, Role, SessionRecord, UserRecord},
    routes::build_router,
    state::AppState,
};

async fn recv_type(stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, expected: &str) -> Value {
    for _ in 0..20 {
        if let Some(Ok(Message::Text(text))) = stream.next().await {
            let value: Value = serde_json::from_str(text.as_str()).expect("ws json");
            if value.get("type").and_then(Value::as_str) == Some(expected) {
                return value;
            }
        }
    }
    panic!("did not receive expected websocket message type");
}

#[tokio::test]
async fn ws_patch_conflict_and_idempotency_replay() {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite pool");
    let state = AppState::new(db_pool);

    let session_id = "ws-session".to_string();
    {
        let mut store = state.store.write().await;
        let user = UserRecord {
            id: "user-1".to_string(),
            email: "owner@example.com".to_string(),
            display_name: "Owner".to_string(),
            role: Role::Owner,
            status: "active".to_string(),
            created_at: "2026-02-14T00:00:00Z".to_string(),
            password_hash: "noop".to_string(),
        };
        store.users.insert(user.id.clone(), user);
        store.sessions.insert(
            session_id.clone(),
            SessionRecord {
                id: session_id.clone(),
                user_id: "user-1".to_string(),
                csrf_token: "csrf".to_string(),
            },
        );
        store.notes.insert(
            "note-1".to_string(),
            NoteRecord {
                id: "note-1".to_string(),
                workspace_id: "ws-1".to_string(),
                project_id: None,
                title: "Title".to_string(),
                note_kind: "markdown".to_string(),
                access_scope: "workspace".to_string(),
                markdown: "abc".to_string(),
                current_version: 1,
                deleted: false,
                metadata_json: Default::default(),
                tags: vec![],
                history: vec![],
                idempotency: Default::default(),
            },
        );
    }

    let app = build_router(state);
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("listener");
    let addr = listener.local_addr().expect("local addr");
    let server = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let mut request = format!("ws://{addr}/ws")
        .into_client_request()
        .expect("ws request");
    request.headers_mut().insert(
        "Cookie",
        format!("kjxlkj_sid={session_id}")
            .parse()
            .expect("cookie header"),
    );
    let (mut stream, _) = connect_async(request).await.expect("ws connect");

    stream
        .send(Message::Text(
            json!({"type":"subscribe_note","note_id":"note-1"})
                .to_string()
                .into(),
        ))
        .await
        .expect("subscribe send");
    let _ = recv_type(&mut stream, "subscribed").await;

    stream
        .send(Message::Text(
            json!({"type":"apply_patch","note_id":"note-1","base_version":0,"idempotency_key":"dup"})
                .to_string()
                .into(),
        ))
        .await
        .expect("patch conflict send");
    let rejected = recv_type(&mut stream, "patch_rejected").await;
    assert_eq!(rejected.get("reason"), Some(&json!("VERSION_CONFLICT")));

    stream
        .send(Message::Text(
            json!({"type":"apply_patch","note_id":"note-1","base_version":1,"idempotency_key":"dup"})
                .to_string()
                .into(),
        ))
        .await
        .expect("patch send");
    let committed = recv_type(&mut stream, "patch_committed").await;
    let first_seq = committed
        .get("event_seq")
        .and_then(Value::as_u64)
        .expect("event seq");

    stream
        .send(Message::Text(
            json!({"type":"apply_patch","note_id":"note-1","base_version":1,"idempotency_key":"dup"})
                .to_string()
                .into(),
        ))
        .await
        .expect("patch replay send");
    let replay = recv_type(&mut stream, "patch_committed").await;
    let replay_seq = replay
        .get("event_seq")
        .and_then(Value::as_u64)
        .expect("replay seq");
    assert_eq!(first_seq, replay_seq);

    let _ = stream.close(None).await;
    server.abort();
}
