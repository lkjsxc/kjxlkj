/// WebSocket handler per /docs/spec/api/websocket.md
///
/// Endpoint: GET /ws
/// Session must be verified before upgrade.
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use crate::protocol::{ClientMessage, ServerMessage};

/// WebSocket upgrade handler
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Per /docs/spec/api/websocket.md: verify authenticated session
    // before processing messages. This is a stub implementation.
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    let response = process_client_message(client_msg);
                    let json = serde_json::to_string(&response).unwrap_or_default();
                    if socket.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}

fn process_client_message(msg: ClientMessage) -> ServerMessage {
    match msg {
        ClientMessage::SubscribeNote { note_id } => ServerMessage::Subscribed {
            stream_id: note_id,
            current_version: 0,
            replay_cursor: 0,
        },
        ClientMessage::SubscribeWorkspace { workspace_id } => ServerMessage::Subscribed {
            stream_id: workspace_id,
            current_version: 0,
            replay_cursor: 0,
        },
        ClientMessage::ApplyPatch {
            note_id,
            base_version,
            idempotency_key,
            ..
        } => ServerMessage::PatchCommitted {
            note_id,
            version: base_version + 1,
            event_seq: 1,
            idempotency_key,
        },
        _ => ServerMessage::Error {
            code: "NOT_IMPLEMENTED".into(),
            message: "not yet implemented".into(),
            details: None,
            request_id: uuid::Uuid::new_v4().to_string(),
        },
    }
}
