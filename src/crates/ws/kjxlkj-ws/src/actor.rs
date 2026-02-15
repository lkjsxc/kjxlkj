//! WebSocket actor per /docs/spec/api/websocket.md.

use crate::protocol::{ClientMsg, ServerMsg};
use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

/// Per-connection WS actor.
pub struct WsSession {
    pub user_id: Uuid,
    pub pool: sqlx::PgPool,
    pub hb: Instant,
    pub note_subs: HashSet<Uuid>,
    pub ws_subs: HashSet<Uuid>,
    pub ack_cursors: std::collections::HashMap<String, i64>,
}

impl WsSession {
    pub fn new(user_id: Uuid, pool: sqlx::PgPool) -> Self {
        Self {
            user_id,
            pool,
            hb: Instant::now(),
            note_subs: HashSet::new(),
            ws_subs: HashSet::new(),
            ack_cursors: std::collections::HashMap::new(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            let ts = time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            let msg = ServerMsg::Heartbeat { server_ts: ts };
            if let Ok(json) = serde_json::to_string(&msg) {
                ctx.text(json);
            }
        });
    }

    fn handle_message(
        &mut self,
        msg: ClientMsg,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        match msg {
            ClientMsg::SubscribeNote { note_id } => {
                self.note_subs.insert(note_id);
                let stream_id = format!("note:{note_id}");
                let resp = ServerMsg::Subscribed {
                    stream_id,
                    current_version: 0,
                    replay_cursor: 0,
                };
                self.send_json(&resp, ctx);
            }
            ClientMsg::UnsubscribeNote { note_id } => {
                self.note_subs.remove(&note_id);
            }
            ClientMsg::SubscribeWorkspace { workspace_id } => {
                self.ws_subs.insert(workspace_id);
                let stream_id = format!("workspace:{workspace_id}");
                let resp = ServerMsg::Subscribed {
                    stream_id,
                    current_version: 0,
                    replay_cursor: 0,
                };
                self.send_json(&resp, ctx);
            }
            ClientMsg::UnsubscribeWorkspace { workspace_id } => {
                self.ws_subs.remove(&workspace_id);
            }
            ClientMsg::ApplyPatch {
                note_id,
                base_version,
                patch_ops,
                idempotency_key,
                ..
            } => {
                let pool = self.pool.clone();
                let user_id = self.user_id;
                let key = idempotency_key.clone();
                let addr = ctx.address();
                actix::spawn(async move {
                    // Check idempotency
                    if let Ok(Some((_, ver))) =
                        kjxlkj_db::repo::note::check_idempotency(
                            &pool, note_id, &key,
                        )
                        .await
                    {
                        let _ = addr.try_send(WsSend(ServerMsg::PatchCommitted {
                            note_id,
                            version: ver,
                            event_seq: ver,
                            idempotency_key: key,
                        }));
                        return;
                    }
                    // Get current projection
                    let proj =
                        kjxlkj_db::repo::note::get_projection(&pool, note_id)
                            .await;
                    let md = proj
                        .ok()
                        .flatten()
                        .map(|p| p.markdown)
                        .unwrap_or_default();
                    // Apply patch ops
                    let ops: Vec<kjxlkj_domain::patch::PatchOp> =
                        serde_json::from_value(patch_ops.clone())
                            .unwrap_or_default();
                    let new_md = match kjxlkj_domain::patch::apply_patch(&md, &ops) {
                        Ok(m) => m,
                        Err(e) => {
                            let _ = addr.try_send(WsSend(ServerMsg::PatchRejected {
                                note_id,
                                expected_version: base_version,
                                current_version: base_version,
                                reason: e,
                            }));
                            return;
                        }
                    };
                    let payload = serde_json::json!({"patch_ops": patch_ops});
                    match kjxlkj_db::repo::note::apply_mutation(
                        &pool,
                        note_id,
                        base_version,
                        &new_md,
                        None,
                        "patch",
                        &payload,
                        user_id,
                    )
                    .await
                    {
                        Ok(Some(ver)) => {
                            // Record idempotency
                            let eid = kjxlkj_domain::types::new_id();
                            let _ = kjxlkj_db::repo::note::record_idempotency(
                                &pool, note_id, &key, eid, ver,
                            )
                            .await;
                            // Update backlinks
                            let links =
                                kjxlkj_domain::patch::extract_wiki_links(&new_md);
                            let _ = kjxlkj_db::repo::note::update_backlinks(
                                &pool, note_id, &links,
                            )
                            .await;
                            let _ = addr.try_send(WsSend(ServerMsg::PatchCommitted {
                                note_id,
                                version: ver,
                                event_seq: ver,
                                idempotency_key: key,
                            }));
                        }
                        Ok(None) => {
                            // Get current version for rejection
                            let cur = kjxlkj_db::repo::note::get_note(
                                &pool, note_id,
                            )
                            .await
                            .ok()
                            .flatten()
                            .map(|n| n.current_version)
                            .unwrap_or(0);
                            let _ = addr.try_send(WsSend(ServerMsg::PatchRejected {
                                note_id,
                                expected_version: base_version,
                                current_version: cur,
                                reason: "VERSION_CONFLICT".into(),
                            }));
                        }
                        Err(e) => {
                            let _ = addr.try_send(WsSend(ServerMsg::Error {
                                code: "INTERNAL_ERROR".into(),
                                message: e.to_string(),
                                request_id: Uuid::now_v7().to_string(),
                            }));
                        }
                    }
                });
            }
            ClientMsg::Ack {
                stream_id,
                event_seq,
            } => {
                if let Some(cur) = self.ack_cursors.get(&stream_id) {
                    if event_seq < *cur {
                        let resp = ServerMsg::Error {
                            code: "STALE_CURSOR".into(),
                            message: format!(
                                "stale ack: {event_seq} < current {cur}"
                            ),
                            request_id: Uuid::now_v7().to_string(),
                        };
                        self.send_json(&resp, ctx);
                        return;
                    }
                }
                self.ack_cursors.insert(stream_id, event_seq);
            }
            ClientMsg::PresencePing { .. } => {
                // Presence events are lossy per spec; broadcast not implemented
                // in this baseline
            }
        }
    }

    fn send_json(
        &self,
        msg: &ServerMsg,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        if let Ok(json) = serde_json::to_string(msg) {
            ctx.text(json);
        }
    }
}

/// Internal message for sending from async tasks.
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsSend(pub ServerMsg);

impl Handler<WsSend> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: WsSend, ctx: &mut Self::Context) {
        self.send_json(&msg.0, ctx);
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();
                match serde_json::from_str::<ClientMsg>(&text) {
                    Ok(client_msg) => self.handle_message(client_msg, ctx),
                    Err(e) => {
                        let resp = ServerMsg::Error {
                            code: "BAD_REQUEST".into(),
                            message: e.to_string(),
                            request_id: Uuid::now_v7().to_string(),
                        };
                        self.send_json(&resp, ctx);
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
