// WebSocket handler per /docs/spec/api/websocket.md
// Full implementation with Actix WebSocket actor.
use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use kjxlkj_auth::middleware::AuthSession;
use sqlx::PgPool;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::protocol::{ClientMessage, ServerMessage};
use crate::session_mgr::{ConnId, SessionManager};

/// Heartbeat interval per /docs/spec/api/websocket.md
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
/// Client timeout before disconnect
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// WebSocket upgrade endpoint: GET /ws
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    auth: AuthSession,
    pool: web::Data<PgPool>,
    mgr: web::Data<SessionManager>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn_id = Uuid::now_v7();
    let actor = WsSession {
        conn_id,
        user_id: auth.user.id,
        pool: pool.get_ref().clone(),
        mgr: mgr.get_ref().clone(),
        last_hb: Instant::now(),
    };
    ws::start(actor, &req, stream)
}

/// Per-connection actor managing subscriptions and message routing.
pub struct WsSession {
    pub conn_id: ConnId,
    pub user_id: Uuid,
    pub pool: PgPool,
    pub mgr: SessionManager,
    pub last_hb: Instant,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        let user_id = self.user_id;
        ctx.spawn(
            async move { mgr.register(conn_id, user_id).await }
                .into_actor(self)
                .map(|_, _, _| ()),
        );
        self.start_heartbeat(ctx);
        tracing::info!(conn_id = %self.conn_id, "ws session started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let mgr = self.mgr.clone();
        let conn_id = self.conn_id;
        actix::spawn(async move { mgr.unregister(conn_id).await });
        tracing::info!(conn_id = %self.conn_id, "ws session stopped");
    }
}

impl WsSession {
    pub(crate) fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_hb) > CLIENT_TIMEOUT {
                tracing::warn!(conn_id = %act.conn_id, "ws client timeout");
                ctx.stop();
                return;
            }
            let msg = ServerMessage::Heartbeat {
                server_ts: time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap_or_default(),
            };
            if let Ok(json) = serde_json::to_string(&msg) {
                ctx.text(json);
            }
            ctx.ping(b"");
        });
    }

    pub(crate) fn send_msg(&self, ctx: &mut ws::WebsocketContext<Self>, msg: &ServerMessage) {
        if let Ok(json) = serde_json::to_string(msg) {
            ctx.text(json);
        }
    }

    pub(crate) fn send_error(
        &self,
        ctx: &mut ws::WebsocketContext<Self>,
        code: &str,
        message: &str,
    ) {
        let msg = ServerMessage::Error {
            code: code.into(),
            message: message.into(),
            request_id: Uuid::now_v7().to_string(),
        };
        self.send_msg(ctx, &msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                tracing::error!(error = %e, "ws protocol error");
                ctx.stop();
                return;
            }
        };
        match msg {
            ws::Message::Ping(data) => {
                self.last_hb = Instant::now();
                ctx.pong(&data);
            }
            ws::Message::Pong(_) => {
                self.last_hb = Instant::now();
            }
            ws::Message::Text(text) => {
                self.handle_text(text.to_string(), ctx);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl WsSession {
    pub(crate) fn handle_text(
        &mut self,
        text: String,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let client_msg: ClientMessage = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(e) => {
                self.send_error(ctx, "INVALID_MESSAGE", &e.to_string());
                return;
            }
        };
        match client_msg {
            ClientMessage::SubscribeNote { note_id } => {
                self.handle_subscribe_note(note_id, ctx);
            }
            ClientMessage::UnsubscribeNote { note_id } => {
                self.handle_unsubscribe_note(note_id, ctx);
            }
            ClientMessage::SubscribeWorkspace { workspace_id } => {
                self.handle_subscribe_workspace(workspace_id, ctx);
            }
            ClientMessage::UnsubscribeWorkspace { workspace_id } => {
                self.handle_unsubscribe_workspace(workspace_id, ctx);
            }
            ClientMessage::ApplyPatch {
                note_id,
                base_version,
                patch_ops,
                idempotency_key,
                client_ts: _,
            } => {
                self.handle_apply_patch(
                    note_id,
                    base_version,
                    patch_ops,
                    idempotency_key,
                    ctx,
                );
            }
            ClientMessage::Ack {
                stream_id,
                event_seq,
            } => {
                self.handle_ack(stream_id, event_seq, ctx);
            }
            ClientMessage::PresencePing {
                workspace_id,
                note_id,
                cursor,
            } => {
                self.handle_presence(workspace_id, note_id, cursor, ctx);
            }
        }
    }
}
