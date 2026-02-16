use actix::prelude::*;
use actix_web_actors::ws;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn};
use crate::protocol::{ClientMessage, ServerMessage};

/// Heartbeat interval.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
/// Client timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

/// WebSocket session actor.
pub struct WsSession {
    pub user_id: Uuid,
    pub pool: PgPool,
    pub hb: Instant,
    /// Subscription cursors: stream_id -> last_acked_seq
    pub subscriptions: HashMap<String, i64>,
    /// Idempotency cache: key -> (note_id, version, seq)
    pub idempotency: HashMap<String, (Uuid, i64, i64)>,
}

impl WsSession {
    pub fn new(user_id: Uuid, pool: PgPool) -> Self {
        Self {
            user_id,
            pool,
            hb: Instant::now(),
            subscriptions: HashMap::new(),
            idempotency: HashMap::new(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                warn!("ws client timeout, disconnecting");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn send_msg(&self, msg: &ServerMessage, ctx: &mut ws::WebsocketContext<Self>) {
        if let Ok(json) = serde_json::to_string(msg) {
            ctx.text(json);
        }
    }

    fn handle_client_msg(&mut self, msg: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            ClientMessage::SubscribeNote { note_id } => {
                let stream_id = format!("note:{note_id}");
                self.subscriptions.insert(stream_id.clone(), 0);
                self.send_msg(
                    &ServerMessage::Subscribed {
                        stream_id,
                        current_version: 0,
                        replay_cursor: 0,
                    },
                    ctx,
                );
            }
            ClientMessage::UnsubscribeNote { note_id } => {
                let stream_id = format!("note:{note_id}");
                self.subscriptions.remove(&stream_id);
            }
            ClientMessage::SubscribeWorkspace { workspace_id } => {
                let stream_id = format!("ws:{workspace_id}");
                self.subscriptions.insert(stream_id.clone(), 0);
                self.send_msg(
                    &ServerMessage::Subscribed {
                        stream_id,
                        current_version: 0,
                        replay_cursor: 0,
                    },
                    ctx,
                );
            }
            ClientMessage::Ack { stream_id, event_seq } => {
                if let Some(cursor) = self.subscriptions.get_mut(&stream_id) {
                    *cursor = event_seq;
                }
            }
            ClientMessage::ApplyPatch {
                note_id,
                base_version,
                patch_ops,
                idempotency_key,
                client_ts: _,
            } => {
                // Idempotency check
                if let Some((nid, ver, seq)) = self.idempotency.get(&idempotency_key) {
                    self.send_msg(
                        &ServerMessage::PatchCommitted {
                            note_id: *nid,
                            version: *ver,
                            event_seq: *seq,
                            idempotency_key,
                        },
                        ctx,
                    );
                    return;
                }

                // Apply patch via DB (simplified)
                let pool = self.pool.clone();
                let user_id = self.user_id;
                let key = idempotency_key.clone();

                let fut = async move {
                    let markdown = patch_ops
                        .get("markdown")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    kjxlkj_db::repo_note::patch_note(
                        &pool,
                        note_id,
                        base_version,
                        markdown,
                        user_id,
                        "user",
                    )
                    .await
                };

                let key2 = key.clone();
                ctx.spawn(
                    actix::fut::wrap_future(fut).map(move |result, act: &mut Self, ctx| {
                        match result {
                            Ok(new_version) => {
                                let seq = new_version; // Simplified: version = seq
                                act.idempotency
                                    .insert(key2.clone(), (note_id, new_version, seq));
                                act.send_msg(
                                    &ServerMessage::PatchCommitted {
                                        note_id,
                                        version: new_version,
                                        event_seq: seq,
                                        idempotency_key: key2,
                                    },
                                    ctx,
                                );
                            }
                            Err(kjxlkj_db::repo_note::PatchError::Conflict {
                                expected,
                                actual,
                            }) => {
                                act.send_msg(
                                    &ServerMessage::PatchRejected {
                                        note_id,
                                        expected_version: expected,
                                        current_version: actual,
                                        reason: "VERSION_CONFLICT".into(),
                                    },
                                    ctx,
                                );
                            }
                            Err(_) => {
                                act.send_msg(
                                    &ServerMessage::Error {
                                        code: "INTERNAL_ERROR".into(),
                                        message: "patch failed".into(),
                                        details: None,
                                        request_id: Uuid::now_v7().to_string(),
                                    },
                                    ctx,
                                );
                            }
                        }
                    }),
                );
            }
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        info!("ws session started for user {}", self.user_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("ws session stopped for user {}", self.user_id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => self.handle_client_msg(client_msg, ctx),
                    Err(e) => {
                        self.send_msg(
                            &ServerMessage::Error {
                                code: "BAD_REQUEST".into(),
                                message: format!("invalid message: {e}"),
                                details: None,
                                request_id: Uuid::now_v7().to_string(),
                            },
                            ctx,
                        );
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
