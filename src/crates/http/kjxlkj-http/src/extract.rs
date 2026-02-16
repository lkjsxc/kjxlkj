use actix_web::{HttpRequest, HttpResponse};
use kjxlkj_auth::middleware::AuthIdentity;
use kjxlkj_auth::session;
use sqlx::PgPool;

/// Cookie name for session token.
pub const SESSION_COOKIE: &str = "kjxlkj_session";

/// Extract authenticated identity from request.
/// Returns None if not authenticated.
pub async fn extract_identity(
    req: &HttpRequest,
    pool: &PgPool,
) -> Option<AuthIdentity> {
    let cookie = req.cookie(SESSION_COOKIE)?;
    let token = cookie.value();
    let sess = session::validate(pool, token).await.ok()??;
    Some(AuthIdentity {
        session_id: sess.session_id,
        user_id: sess.user_id,
        username: sess.username,
        is_owner: sess.is_owner,
        csrf_token: sess.csrf_token,
    })
}

/// Require authentication. Returns 401 if not authenticated.
pub async fn require_auth(
    req: &HttpRequest,
    pool: &PgPool,
) -> Result<AuthIdentity, HttpResponse> {
    extract_identity(req, pool).await.ok_or_else(|| {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "code": "AUTH_REQUIRED",
            "message": "authentication required",
            "details": null,
            "request_id": uuid::Uuid::now_v7().to_string(),
        }))
    })
}

/// Validate CSRF token for mutating requests.
pub fn validate_csrf(req: &HttpRequest, identity: &AuthIdentity) -> Result<(), HttpResponse> {
    let header_value = req
        .headers()
        .get("X-CSRF-Token")
        .and_then(|v| v.to_str().ok());

    match header_value {
        Some(token) if token == identity.csrf_token => Ok(()),
        _ => Err(HttpResponse::Forbidden().json(serde_json::json!({
            "code": "CSRF_INVALID",
            "message": "invalid or missing CSRF token",
            "details": null,
            "request_id": uuid::Uuid::now_v7().to_string(),
        }))),
    }
}

/// Generate a request ID for error responses.
pub fn request_id() -> String {
    uuid::Uuid::now_v7().to_string()
}
