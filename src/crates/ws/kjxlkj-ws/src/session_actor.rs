use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// WebSocket session actor per /docs/spec/api/websocket.md.
pub struct WsSession {
    /// Last heartbeat timestamp
    pub hb: Instant,
    /// Session user ID
    pub user_id: uuid::Uuid,
    /// Heartbeat interval from config
    pub hb_interval: Duration,
    /// Client timeout from config
    pub client_timeout: Duration,
}

impl WsSession {
    pub fn new(
        user_id: uuid::Uuid,
        hb_interval_ms: u64,
        client_timeout_ms: u64,
    ) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            hb_interval: Duration::from_millis(hb_interval_ms),
            client_timeout: Duration::from_millis(client_timeout_ms),
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
                debug!(user_id = %self.user_id, "ws text: {}", text);
                // Protocol message handling per /docs/spec/api/websocket.md
                // will be expanded in Stage 02 Wave 021
                ctx.text(text);
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
