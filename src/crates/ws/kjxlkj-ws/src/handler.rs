use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::hub::WsHub;
use crate::protocol::{ClientMessage, ServerMessage};
use kjxlkj_auth::session;
use kjxlkj_db::repos;

/// GET /ws — WebSocket upgrade handler per websocket.md.
/// Session MUST be verified before upgrade (csrf.md exception rule).
pub async fn ws_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    hub: web::Data<Arc<WsHub>>,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify session from cookie before upgrade.
    let cookie = req.cookie("session").ok_or_else(|| {
        actix_web::error::ErrorUnauthorized("session required")
    })?;
    let (_, user_id) = session::validate_session(pool.get_ref(), cookie.value())
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("invalid session"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("expired session"))?;

    let (resp, mut ws_session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let pool = pool.into_inner();
    let hub = hub.into_inner();

    actix_rt::spawn(async move {
        // Send initial heartbeat.
        let _ = ws_session
            .text(serde_json::to_string(&ServerMessage::Heartbeat {
                server_ts: chrono_now(),
            }).unwrap_or_default())
            .await;

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    let result = handle_client_message(
                        &text, user_id, &pool, &hub, &mut ws_session,
                    ).await;
                    if let Err(e) = result {
                        let err_msg = ServerMessage::Error {
                            code: "PROTOCOL_ERROR".into(),
                            message: e,
                            request_id: Uuid::new_v4().to_string(),
                        };
                        let _ = ws_session
                            .text(serde_json::to_string(&err_msg).unwrap_or_default())
                            .await;
                    }
                }
                Message::Ping(bytes) => {
                    let _ = ws_session.pong(&bytes).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    Ok(resp)
}

async fn handle_client_message(
    text: &str,
    user_id: Uuid,
    pool: &PgPool,
    hub: &Arc<WsHub>,
    ws: &mut actix_ws::Session,
) -> Result<(), String> {
    let msg: ClientMessage =
        serde_json::from_str(text).map_err(|e| format!("parse error: {e}"))?;

    match msg {
        ClientMessage::SubscribeNote { note_id } => {
            let version = repos::notes::get_version(pool, note_id)
                .await
                .map_err(|e| e.to_string())?
                .unwrap_or(0);
            let resp = ServerMessage::Subscribed {
                stream_id: format!("note:{note_id}"),
                current_version: version,
                replay_cursor: 0,
            };
            let _ = ws
                .text(serde_json::to_string(&resp).unwrap_or_default())
                .await;
        }
        ClientMessage::SubscribeWorkspace { workspace_id } => {
            let resp = ServerMessage::Subscribed {
                stream_id: format!("workspace:{workspace_id}"),
                current_version: 0,
                replay_cursor: 0,
            };
            let _ = ws
                .text(serde_json::to_string(&resp).unwrap_or_default())
                .await;
        }
        ClientMessage::ApplyPatch {
            note_id,
            base_version,
            patch_ops,
            idempotency_key,
            ..
        } => {
            let body = patch_ops
                .get("body")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match repos::notes::update_body(pool, note_id, body, base_version).await {
                Ok(Some(note)) => {
                    let event = repos::events::append_note_event(
                        pool, Uuid::new_v4(), note_id, note.version,
                        "note.patched",
                        &serde_json::json!({"version": note.version}), user_id,
                    ).await;
                    let seq = event.map(|e| e.seq).unwrap_or(0);
                    let committed = ServerMessage::PatchCommitted {
                        note_id,
                        version: note.version,
                        event_seq: seq,
                        idempotency_key,
                    };
                    let _ = ws
                        .text(serde_json::to_string(&committed).unwrap_or_default())
                        .await;
                    hub.broadcast_note_event(note_id, ServerMessage::NoteEvent {
                        note_id,
                        event_seq: seq,
                        version: note.version,
                        event_type: "note.patched".into(),
                        payload: serde_json::json!({"version": note.version}),
                    }).await;
                }
                _ => {
                    let current = repos::notes::get_version(pool, note_id)
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or(0);
                    let rejected = ServerMessage::PatchRejected {
                        note_id,
                        expected_version: base_version,
                        current_version: current,
                        reason: "version conflict".into(),
                    };
                    let _ = ws
                        .text(serde_json::to_string(&rejected).unwrap_or_default())
                        .await;
                }
            }
        }
        ClientMessage::PresencePing {
            workspace_id,
            note_id,
            ..
        } => {
            hub.broadcast_workspace_event(
                workspace_id,
                ServerMessage::PresenceEvent {
                    workspace_id,
                    note_id,
                    user_id,
                    state: "active".into(),
                    server_ts: chrono_now(),
                },
            ).await;
        }
        ClientMessage::Ack { .. } | ClientMessage::UnsubscribeNote { .. }
        | ClientMessage::UnsubscribeWorkspace { .. } => {
            // Acknowledged; no response needed.
        }
    }
    Ok(())
}

fn chrono_now() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_default()
}
