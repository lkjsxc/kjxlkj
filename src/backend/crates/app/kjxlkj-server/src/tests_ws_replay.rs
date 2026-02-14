use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::{net::TcpListener, time::{timeout, Duration}};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    model::{NoteRecord, Role, SessionRecord, UserRecord},
    routes::build_router,
    state::AppState,
};

async fn recv_type(
    stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    expected: &str,
) -> Value {
    for _ in 0..30 {
        if let Some(Ok(Message::Text(text))) = stream.next().await {
            let value: Value = serde_json::from_str(text.as_str()).expect("ws json");
            if value.get("type").and_then(Value::as_str) == Some(expected) {
                return value;
            }
        }
    }
    panic!("missing expected websocket message");
}

async fn connect_with_session(
    addr: std::net::SocketAddr,
    session_id: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let mut request = format!("ws://{addr}/ws")
        .into_client_request()
        .expect("ws request");
    request.headers_mut().insert(
        "Cookie",
        format!("kjxlkj_sid={session_id}")
            .parse()
            .expect("cookie header"),
    );
    let (stream, _) = connect_async(request).await.expect("ws connect");
    stream
}

#[tokio::test]
async fn ws_reconnect_ack_cursor_replay_is_deterministic() {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("sqlite pool");
    let state = AppState::new(db_pool);

    let session_id = "ws-replay-session".to_string();
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

    let mut first = connect_with_session(addr, &session_id).await;
    first
        .send(Message::Text(
            json!({"type":"subscribe_note","note_id":"note-1"})
                .to_string()
                .into(),
        ))
        .await
        .expect("subscribe send");
    let _ = recv_type(&mut first, "subscribed").await;

    first
        .send(Message::Text(
            json!({"type":"apply_patch","note_id":"note-1","base_version":1,"idempotency_key":"k1"})
                .to_string()
                .into(),
        ))
        .await
        .expect("patch send");
    let committed = recv_type(&mut first, "patch_committed").await;
    let event_seq = committed
        .get("event_seq")
        .and_then(Value::as_u64)
        .expect("event seq");

    first
        .send(Message::Text(
            json!({"type":"ack","stream_id":"note:note-1","event_seq":event_seq})
                .to_string()
                .into(),
        ))
        .await
        .expect("ack send");
    let _ = first.close(None).await;

    let mut second = connect_with_session(addr, &session_id).await;
    second
        .send(Message::Text(
            json!({"type":"subscribe_note","note_id":"note-1","ack_cursor":event_seq})
                .to_string()
                .into(),
        ))
        .await
        .expect("resubscribe send");
    let subscribed = recv_type(&mut second, "subscribed").await;
    assert_eq!(subscribed.get("replay_cursor"), Some(&json!(event_seq)));

    let maybe_event = timeout(Duration::from_millis(150), second.next()).await;
    if let Ok(Some(Ok(Message::Text(text)))) = maybe_event {
        let value: Value = serde_json::from_str(text.as_str()).expect("ws json");
        assert_ne!(value.get("type"), Some(&json!("note_event")));
    }

    let _ = second.close(None).await;
    server.abort();
}
