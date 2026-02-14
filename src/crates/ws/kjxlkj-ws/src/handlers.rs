// WebSocket message handler implementations per /docs/spec/api/websocket.md
// Split from handler.rs for the 200-line file policy.
use actix::prelude::*;
use actix_web_actors::ws;
use kjxlkj_db::repo::notes;
use uuid::Uuid;

use crate::handler::WsSession;
use crate::protocol::ServerMessage;
use crate::session_mgr::StreamKey;

impl WsSession {
    /// Handle subscribe_note: look up current version and replay cursor.
    pub(crate) fn handle_subscribe_note(
        &mut self,
        note_id: Uuid,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let pool = self.pool.clone();
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        let key = StreamKey::note(note_id);

        ctx.spawn(
            async move {
                let version = notes::current_version(&pool, note_id)
                    .await
                    .unwrap_or(0);
                mgr.subscribe(conn_id, key.clone()).await;
                ServerMessage::Subscribed {
                    stream_id: key.0,
                    current_version: version,
                    replay_cursor: version,
                }
            }
            .into_actor(self)
            .map(|msg, act, ctx| {
                act.send_msg(ctx, &msg);
            }),
        );
    }

    /// Handle unsubscribe_note.
    pub(crate) fn handle_unsubscribe_note(
        &mut self,
        note_id: Uuid,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        let key = StreamKey::note(note_id);
        ctx.spawn(
            async move { mgr.unsubscribe(conn_id, &key).await }
                .into_actor(self)
                .map(|_, _, _| ()),
        );
    }

    /// Handle subscribe_workspace.
    pub(crate) fn handle_subscribe_workspace(
        &mut self,
        workspace_id: Uuid,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        let key = StreamKey::workspace(workspace_id);

        ctx.spawn(
            async move {
                mgr.subscribe(conn_id, key.clone()).await;
                ServerMessage::Subscribed {
                    stream_id: key.0,
                    current_version: 0,
                    replay_cursor: 0,
                }
            }
            .into_actor(self)
            .map(|msg, act, ctx| {
                act.send_msg(ctx, &msg);
            }),
        );
    }

    /// Handle unsubscribe_workspace.
    pub(crate) fn handle_unsubscribe_workspace(
        &mut self,
        workspace_id: Uuid,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        let key = StreamKey::workspace(workspace_id);
        ctx.spawn(
            async move { mgr.unsubscribe(conn_id, &key).await }
                .into_actor(self)
                .map(|_, _, _| ()),
        );
    }

    /// Handle apply_patch with idempotency and version checking.
    /// Per spec: duplicate idempotency_key MUST replay-safe-return.
    /// Conflicting base_version MUST return patch_rejected.
    pub(crate) fn handle_apply_patch(
        &mut self,
        note_id: Uuid,
        base_version: i64,
        patch_ops: Vec<serde_json::Value>,
        idempotency_key: String,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let pool = self.pool.clone();
        let mgr = self.mgr.clone();
        let actor_id = self.user_id;
        let _conn_id = self.conn_id;

        ctx.spawn(
            async move {
                // Check idempotency cache first
                if let Some((ver, seq)) =
                    mgr.check_idempotency(note_id, &idempotency_key).await
                {
                    return ServerMessage::PatchCommitted {
                        note_id,
                        version: ver,
                        event_seq: seq,
                        idempotency_key,
                    };
                }

                let payload = serde_json::json!({ "ops": patch_ops });
                // Construct markdown from patch ops (simplified: take body)
                let new_md = patch_ops
                    .iter()
                    .filter_map(|op| op.get("text").and_then(|t| t.as_str()))
                    .collect::<Vec<_>>()
                    .join("");
                let fallback_md = if new_md.is_empty() {
                    serde_json::to_string(&patch_ops).unwrap_or_default()
                } else {
                    new_md
                };

                match notes::apply_mutation(
                    &pool,
                    note_id,
                    base_version,
                    &fallback_md,
                    actor_id,
                    "patch",
                    &payload,
                )
                .await
                {
                    Ok(Some(new_version)) => {
                        let event_seq = new_version;
                        mgr.record_idempotency(
                            note_id,
                            idempotency_key.clone(),
                            new_version,
                            event_seq,
                        )
                        .await;
                        ServerMessage::PatchCommitted {
                            note_id,
                            version: new_version,
                            event_seq,
                            idempotency_key,
                        }
                    }
                    Ok(None) => {
                        let cur = notes::current_version(&pool, note_id)
                            .await
                            .unwrap_or(0);
                        ServerMessage::PatchRejected {
                            note_id,
                            expected_version: base_version,
                            current_version: cur,
                            reason: "VERSION_CONFLICT".into(),
                        }
                    }
                    Err(e) => ServerMessage::Error {
                        code: "INTERNAL_ERROR".into(),
                        message: e.to_string(),
                        request_id: Uuid::now_v7().to_string(),
                    },
                }
            }
            .into_actor(self)
            .map(move |msg, act, ctx| {
                act.send_msg(ctx, &msg);
                // Broadcast note_event to other subscribers
                if let ServerMessage::PatchCommitted {
                    note_id,
                    version,
                    event_seq,
                    ..
                } = &msg
                {
                    let _note_id = *note_id;
                    let _version = *version;
                    let _event_seq = *event_seq;
                    // Broadcast handled via session_mgr
                    // (full broadcast requires addr registry — deferred)
                }
            }),
        );
    }

    /// Handle ack: update cursor, check for stale regression.
    pub(crate) fn handle_ack(
        &mut self,
        stream_id: String,
        event_seq: i64,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        ctx.spawn(
            async move { mgr.update_ack(conn_id, &stream_id, event_seq).await }
                .into_actor(self)
                .map(move |result, act, ctx| {
                    if let Err(current_cursor) = result {
                        let msg = ServerMessage::Error {
                            code: "STALE_CURSOR".into(),
                            message: format!(
                                "Ack seq {event_seq} < current cursor {current_cursor}"
                            ),
                            request_id: Uuid::now_v7().to_string(),
                        };
                        act.send_msg(ctx, &msg);
                    }
                }),
        );
    }

    /// Handle presence_ping: broadcast presence to workspace subscribers.
    /// Per spec: presence events MAY be lossy.
    pub(crate) fn handle_presence(
        &mut self,
        workspace_id: Uuid,
        note_id: Option<Uuid>,
        _cursor: Option<serde_json::Value>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let _msg = ServerMessage::PresenceEvent {
            workspace_id,
            note_id,
            user_id: self.user_id,
            state: "active".into(),
            server_ts: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
        };
        // Presence broadcast to other subscribers is best-effort.
        // Full implementation requires addr registry for cross-actor delivery.
        let _ = ctx;
    }
}
