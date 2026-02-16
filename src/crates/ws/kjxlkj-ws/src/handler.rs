use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use sqlx::PgPool;
use kjxlkj_auth::session as auth_session;
use crate::session::WsSession;

/// GET /ws - WebSocket upgrade handler.
/// Authenticates via session cookie before upgrade.
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    // Authenticate before upgrade
    let cookie = req.cookie("kjxlkj_session");
    let token = match cookie {
        Some(c) => c.value().to_string(),
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "code": "AUTH_REQUIRED",
                "message": "session required for websocket",
            })));
        }
    };

    let sess = auth_session::validate(&pool, &token).await;
    let user = match sess {
        Ok(Some(u)) => u,
        _ => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "code": "AUTH_REQUIRED",
                "message": "invalid session",
            })));
        }
    };

    let ws_session = WsSession::new(user.user_id, pool.get_ref().clone());
    ws::start(ws_session, &req, stream)
}
