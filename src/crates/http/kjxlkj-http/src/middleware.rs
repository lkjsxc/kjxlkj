//! Session extraction middleware per /docs/spec/security/sessions.md and csrf.md.

use crate::dto::ApiError;
use actix_web::{HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

/// Extracted session context for authenticated requests.
pub struct SessionCtx {
    pub user_id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub csrf_token: String,
    pub session_id: Uuid,
}

const SESSION_COOKIE: &str = "kjxlkj_session";

/// Extract session from cookie. Returns None if unauthenticated.
pub async fn extract_session(
    req: &HttpRequest,
    pool: &PgPool,
) -> Option<SessionCtx> {
    let cookie = req.cookie(SESSION_COOKIE)?;
    let sid: Uuid = cookie.value().parse().ok()?;
    let sess = kjxlkj_db::repo::session::find_session(pool, sid)
        .await
        .ok()??;
    let user = kjxlkj_db::repo::user::find_by_id(pool, sess.user_id)
        .await
        .ok()??;
    if user.status == "disabled" {
        return None;
    }
    Some(SessionCtx {
        user_id: user.id,
        email: user.email,
        display_name: user.display_name,
        role: user.role,
        csrf_token: sess.csrf_token,
        session_id: sess.id,
    })
}

/// Require authenticated session or return 401.
pub async fn require_session(
    req: &HttpRequest,
    pool: &PgPool,
) -> Result<SessionCtx, HttpResponse> {
    extract_session(req, pool).await.ok_or_else(|| {
        HttpResponse::Unauthorized()
            .json(ApiError::new("AUTH_REQUIRED", "authentication required"))
    })
}

/// Validate CSRF token for mutating requests per /docs/spec/security/csrf.md.
pub fn validate_csrf(
    req: &HttpRequest,
    ctx: &SessionCtx,
    config: &kjxlkj_db::config::AppConfig,
) -> Result<(), HttpResponse> {
    let header_name = &config.security.csrf_header;
    let token = req
        .headers()
        .get(header_name.as_str())
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if token != ctx.csrf_token {
        return Err(HttpResponse::Forbidden()
            .json(ApiError::new("CSRF_INVALID", "invalid CSRF token")));
    }
    Ok(())
}

/// Build a session cookie value.
pub fn make_session_cookie(
    session_id: Uuid,
    config: &kjxlkj_db::config::AppConfig,
) -> actix_web::cookie::Cookie<'static> {
    let mut builder = actix_web::cookie::Cookie::build(
        SESSION_COOKIE,
        session_id.to_string(),
    )
    .path("/")
    .http_only(true);
    if config.security.secure_cookies {
        builder = builder.secure(true);
    }
    let same = match config.security.same_site.as_str() {
        "strict" => actix_web::cookie::SameSite::Strict,
        "none" => actix_web::cookie::SameSite::None,
        _ => actix_web::cookie::SameSite::Lax,
    };
    builder = builder.same_site(same);
    builder.finish()
}

/// Build an expired cookie for logout.
pub fn expire_session_cookie() -> actix_web::cookie::Cookie<'static> {
    actix_web::cookie::Cookie::build(SESSION_COOKIE, "")
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish()
}

/// Error response helper for 404.
pub fn not_found(resource: &str) -> HttpResponse {
    let code = format!("{}_NOT_FOUND", resource.to_uppercase());
    HttpResponse::NotFound()
        .json(ApiError::new(&code, format!("{resource} not found")))
}

/// Error response helper for 409 version conflict.
pub fn version_conflict(current: i64) -> HttpResponse {
    HttpResponse::Conflict().json(
        ApiError::new("VERSION_CONFLICT", "version conflict")
            .with_details(serde_json::json!({"current_version": current})),
    )
}

/// Role-forbidden response.
pub fn forbidden() -> HttpResponse {
    HttpResponse::Forbidden()
        .json(ApiError::new("ROLE_FORBIDDEN", "insufficient permissions"))
}
