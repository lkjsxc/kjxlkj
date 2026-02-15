use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use kjxlkj_auth::session;
use kjxlkj_domain::ids::SessionId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::session_actor::WsSession;

/// WebSocket upgrade handler.
/// Per /docs/spec/security/csrf.md: WS handshake MUST verify
/// authenticated session before upgrade.
pub async fn ws_connect(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<PgPool>,
    ws_config: web::Data<WsConfig>,
) -> Result<HttpResponse, actix_web::Error> {
    let cookie = req
        .cookie("kjxlkj_session")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("session required"))?;

    let session_id = cookie
        .value()
        .parse::<Uuid>()
        .map(SessionId)
        .map_err(|_| actix_web::error::ErrorUnauthorized("invalid session"))?;

    let (user_id, _csrf) = session::validate_session(pool.get_ref(), session_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("expired session"))?;

    let actor = WsSession::new(
        user_id.0,
        ws_config.heartbeat_interval_ms,
        ws_config.client_timeout_ms,
        pool.get_ref().clone(),
        ws_config.replay_batch_size,
    );
    ws::start(actor, &req, stream)
}

/// WebSocket runtime configuration.
#[derive(Debug, Clone)]
pub struct WsConfig {
    pub heartbeat_interval_ms: u64,
    pub client_timeout_ms: u64,
    pub replay_batch_size: u32,
}
