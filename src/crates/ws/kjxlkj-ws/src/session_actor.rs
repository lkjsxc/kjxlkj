use actix::ActorFutureExt;
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use sqlx::PgPool;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

use crate::messages::{ClientMessage, ServerMessage};
use crate::protocol;
use crate::subscriptions::SubscriptionState;

/// WebSocket session actor per /docs/spec/api/websocket.md.
pub struct WsSession {
    pub hb: Instant,
    pub user_id: uuid::Uuid,
    pub hb_interval: Duration,
    pub client_timeout: Duration,
    pub pool: PgPool,
    pub subs: SubscriptionState,
    pub replay_batch: i64,
}

impl WsSession {
    pub fn new(
        user_id: uuid::Uuid,
        hb_interval_ms: u64,
        client_timeout_ms: u64,
        pool: PgPool,
        replay_batch: u32,
    ) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            hb_interval: Duration::from_millis(hb_interval_ms),
            client_timeout: Duration::from_millis(client_timeout_ms),
            pool,
            subs: SubscriptionState::new(),
            replay_batch: replay_batch as i64,
        }
    }

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let timeout = self.client_timeout;
        ctx.run_interval(self.hb_interval, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > timeout {
                warn!("ws client timeout, disconnecting");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
            // Send server heartbeat per protocol
            let hb = ServerMessage::Heartbeat {
                server_ts: time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap_or_default(),
            };
            ctx.text(hb.to_json());
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);
        debug!(user_id = %self.user_id, "ws session started");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(data)) => {
                self.hb = Instant::now();
                ctx.pong(&data);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();
                debug!(user_id = %self.user_id, "ws text: {}", text);

                // Parse client message
                let client_msg: ClientMessage = match serde_json::from_str(&text) {
                    Ok(m) => m,
                    Err(e) => {
                        let err = ServerMessage::Error {
                            code: "BAD_REQUEST".into(),
                            message: format!("invalid message: {e}"),
                            request_id: uuid::Uuid::now_v7().to_string(),
                        };
                        ctx.text(err.to_json());
                        return;
                    }
                };

                // Dispatch via protocol handler (async)
                let pool = self.pool.clone();
                let user_id = self.user_id;
                let replay_batch = self.replay_batch;

                // We need to use a sync approach since we
                // can't easily hold &mut self across await.
                // Use actix spawn_local via `ctx.wait`.
                let subs_ptr = &mut self.subs as *mut SubscriptionState;
                // SAFETY: we process one message at a time in
                // the single-threaded actor context.
                let fut = async move {
                    let subs = unsafe { &mut *subs_ptr };
                    protocol::handle_message(
                        client_msg, user_id, &pool, subs, replay_batch,
                    ).await
                };
                let fut = actix::fut::wrap_future::<_, Self>(fut);
                ctx.wait(fut.map(|responses, _act, ctx: &mut ws::WebsocketContext<Self>| {
                    for resp in responses {
                        ctx.text(resp.to_json());
                    }
                }));
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
