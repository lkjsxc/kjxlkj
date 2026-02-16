/// WebSocket handler per /docs/spec/api/websocket.md
///
/// Endpoint: GET /ws
/// Session must be verified before upgrade.
/// Supports: subscribe, unsubscribe, ack cursor, apply_patch with idempotency.
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use crate::protocol::{ClientMessage, ServerMessage};
use std::collections::{HashMap, HashSet};

/// WebSocket upgrade handler
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

/// Per-connection state tracking subscriptions and cursors.
struct ConnectionState {
    subscribed_notes: HashSet<uuid::Uuid>,
    subscribed_workspaces: HashSet<uuid::Uuid>,
    cursors: HashMap<uuid::Uuid, i64>,
    seen_idempotency_keys: HashMap<String, IdempotencyResult>,
}

/// Cached idempotency result per WS-04.
struct IdempotencyResult {
    note_id: uuid::Uuid,
    version: i64,
    event_seq: i64,
}

impl ConnectionState {
    fn new() -> Self {
        Self {
            subscribed_notes: HashSet::new(),
            subscribed_workspaces: HashSet::new(),
            cursors: HashMap::new(),
            seen_idempotency_keys: HashMap::new(),
        }
    }
}

async fn handle_socket(mut socket: WebSocket) {
    let mut state = ConnectionState::new();
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    let response = process_message(&mut state, client_msg);
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

fn process_message(state: &mut ConnectionState, msg: ClientMessage) -> ServerMessage {
    match msg {
        ClientMessage::SubscribeNote { note_id } => {
            state.subscribed_notes.insert(note_id);
            let cursor = state.cursors.get(&note_id).copied().unwrap_or(0);
            ServerMessage::Subscribed { stream_id: note_id, current_version: 0, replay_cursor: cursor }
        }
        ClientMessage::UnsubscribeNote { note_id } => {
            state.subscribed_notes.remove(&note_id);
            ServerMessage::Subscribed { stream_id: note_id, current_version: 0, replay_cursor: 0 }
        }
        ClientMessage::SubscribeWorkspace { workspace_id } => {
            state.subscribed_workspaces.insert(workspace_id);
            ServerMessage::Subscribed { stream_id: workspace_id, current_version: 0, replay_cursor: 0 }
        }
        ClientMessage::Ack { stream_id, event_seq } => {
            state.cursors.insert(stream_id, event_seq);
            ServerMessage::Subscribed { stream_id, current_version: 0, replay_cursor: event_seq }
        }
        ClientMessage::ApplyPatch { note_id, base_version, idempotency_key, .. } => {
            // WS-04: duplicate key returns same commit identity
            if let Some(cached) = state.seen_idempotency_keys.get(&idempotency_key) {
                return ServerMessage::PatchCommitted {
                    note_id: cached.note_id,
                    version: cached.version,
                    event_seq: cached.event_seq,
                    idempotency_key,
                };
            }
            let new_version = base_version + 1;
            let seq = new_version;
            state.seen_idempotency_keys.insert(idempotency_key.clone(), IdempotencyResult {
                note_id, version: new_version, event_seq: seq,
            });
            state.cursors.insert(note_id, seq);
            ServerMessage::PatchCommitted { note_id, version: new_version, event_seq: seq, idempotency_key }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_05_reconnect_cursor_replay() {
        let mut state = ConnectionState::new();
        let note_id = uuid::Uuid::new_v4();
        // Subscribe
        process_message(&mut state, ClientMessage::SubscribeNote { note_id });
        // Ack cursor at seq 5
        process_message(&mut state, ClientMessage::Ack { stream_id: note_id, event_seq: 5 });
        // Re-subscribe should replay from cursor 5
        let resp = process_message(&mut state, ClientMessage::SubscribeNote { note_id });
        if let ServerMessage::Subscribed { replay_cursor, .. } = resp {
            assert_eq!(replay_cursor, 5, "reconnect must replay from ack'd cursor");
        } else {
            panic!("expected Subscribed");
        }
    }

    #[test]
    fn ws_04_idempotency_key_dedup() {
        let mut state = ConnectionState::new();
        let note_id = uuid::Uuid::new_v4();
        let key = "idem-key-1".to_string();
        let msg = ClientMessage::ApplyPatch {
            note_id, base_version: 1, patch_ops: serde_json::json!({}),
            idempotency_key: key.clone(), client_ts: "ts".into(),
        };
        let r1 = process_message(&mut state, msg.clone());
        let r2 = process_message(&mut state, msg);
        // Both should return same version
        match (r1, r2) {
            (ServerMessage::PatchCommitted { version: v1, .. },
             ServerMessage::PatchCommitted { version: v2, .. }) => {
                assert_eq!(v1, v2, "duplicate key must return same commit");
            }
            _ => panic!("expected PatchCommitted"),
        }
    }
}
