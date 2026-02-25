//! WebSocket handler

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, State},
    response::IntoResponse,
    Extension,
};
use futures::{sink::SinkExt, stream::StreamExt};
use uuid::Uuid;

use kjxlkj_domain::Actor;
use kjxlkj_db::{NoteRepo, IdempotencyRepo};
use crate::protocol::WsMessage;
use crate::session::{WsSession, SessionRegistry, BroadcastRegistry};

/// Upgrade to WebSocket
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
    Extension(user_id): Extension<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(socket: WebSocket, state: WsState, user_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    
    let session = WsSession::new(user_id);
    state.session_registry.add(session).await;

    let mut recv = receiver;
    let state_clone = state.clone();
    let session_id = Uuid::new_v4();
    
    let handle_task = tokio::spawn(async move {
        while let Some(msg) = recv.next().await {
            let msg = match msg {
                Ok(m) => m,
                Err(_) => break,
            };

            match msg {
                Message::Text(text) => {
                    if let Ok(response) = handle_message(&text, &state_clone, session_id).await {
                        let _ = sender.send(Message::Text(response)).await;
                    }
                }
                Message::Ping(data) => {
                    let _ = sender.send(Message::Pong(data)).await;
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    let _ = handle_task.await;
    state.session_registry.remove(session_id).await;
}

async fn handle_message(
    text: &str,
    state: &WsState,
    session_id: Uuid,
) -> Result<String, serde_json::Error> {
    let msg: WsMessage = serde_json::from_str(text)?;

    let response = match msg {
        WsMessage::Ack { event_seq, .. } => {
            state.session_registry.update_cursor(session_id, event_seq).await;
            WsMessage::Pong
        }
        WsMessage::ApplyPatch { note_id, base_version, idempotency_key, .. } => {
            if state.idempotency_repo.exists(&idempotency_key).await {
                WsMessage::PatchCommitted {
                    note_id,
                    version: base_version + 1,
                    event_seq: 0,
                    idempotency_key,
                }
            } else {
                let note_repo = NoteRepo::new();
                match note_repo.get(note_id).await {
                    Ok(Some(mut note)) => {
                        if note.version == base_version {
                            note.version += 1;
                            state.idempotency_repo.set(&idempotency_key, "committed").await;
                            WsMessage::PatchCommitted {
                                note_id,
                                version: note.version,
                                event_seq: 0,
                                idempotency_key,
                            }
                        } else {
                            WsMessage::PatchRejected {
                                note_id,
                                expected_version: base_version,
                                current_version: note.version,
                                reason: "VERSION_MISMATCH".into(),
                            }
                        }
                    }
                    _ => WsMessage::Error {
                        code: "NOTE_NOT_FOUND".into(),
                        message: "Note not found".into(),
                    }
                }
            }
        }
        _ => WsMessage::Error {
            code: "UNKNOWN_MESSAGE".into(),
            message: "Unknown message type".into(),
        }
    };

    serde_json::to_string(&response)
}

/// WebSocket router state
#[derive(Debug, Clone)]
pub struct WsState {
    pub session_registry: SessionRegistry,
    pub broadcast_registry: BroadcastRegistry,
    pub idempotency_repo: IdempotencyRepo,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            session_registry: SessionRegistry::new(),
            broadcast_registry: BroadcastRegistry::new(),
            idempotency_repo: IdempotencyRepo::default(),
        }
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}
