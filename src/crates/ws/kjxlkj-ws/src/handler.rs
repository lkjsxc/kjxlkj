// WebSocket handler per /docs/spec/api/websocket.md
use actix_web::{web, HttpRequest, HttpResponse};

/// WebSocket upgrade endpoint: GET /ws
pub async fn ws_handler(
    _req: HttpRequest,
    _stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: verify authenticated session before upgrade
    // TODO: create actor with subscription management
    let resp = HttpResponse::NotImplemented().json(serde_json::json!({
        "code": "NOT_IMPLEMENTED",
        "message": "WebSocket handler pending full implementation"
    }));
    Ok(resp)
}
