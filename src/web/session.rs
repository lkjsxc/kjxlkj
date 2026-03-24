use actix_web::cookie::{time::Duration as CookieDuration, Cookie};
use actix_web::{web, HttpRequest};
use chrono::Utc;
use uuid::Uuid;

use crate::error::AppError;

use super::state::AppState;

pub const SESSION_COOKIE_NAME: &str = "session_id";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionState {
    pub session_id: Uuid,
    pub admin_id: i64,
}

pub fn session_id_from_request(request: &HttpRequest) -> Option<Uuid> {
    let raw = request.cookie(SESSION_COOKIE_NAME)?;
    Uuid::parse_str(raw.value()).ok()
}

pub async fn valid_session(
    request: &HttpRequest,
    state: &web::Data<AppState>,
) -> Result<Option<SessionState>, AppError> {
    let Some(session_id) = session_id_from_request(request) else {
        return Ok(None);
    };
    let session = state.auth_store.lookup_session(session_id).await?;
    let Some(session) = session else {
        return Ok(None);
    };
    if session.expires_at <= Utc::now() {
        state.auth_store.delete_session(session_id).await?;
        return Ok(None);
    }
    Ok(Some(SessionState {
        session_id,
        admin_id: session.admin_id,
    }))
}

pub async fn require_admin_session(
    request: &HttpRequest,
    state: &web::Data<AppState>,
) -> Result<SessionState, actix_web::HttpResponse> {
    match state.auth_store.has_admin_user().await {
        Ok(false) => return Err(redirect_guard(request, "/setup")),
        Ok(true) => {}
        Err(_) => return Err(actix_web::HttpResponse::InternalServerError().finish()),
    }
    let Some(session_id) = session_id_from_request(request) else {
        return Err(redirect_guard(request, "/login"));
    };
    let session = match state.auth_store.lookup_session(session_id).await {
        Ok(session) => session,
        Err(_) => return Err(actix_web::HttpResponse::InternalServerError().finish()),
    };
    let Some(session) = session else {
        return Err(redirect_guard(request, "/login"));
    };
    if session.expires_at <= Utc::now() {
        if state.auth_store.delete_session(session_id).await.is_err() {
            return Err(actix_web::HttpResponse::InternalServerError().finish());
        }
        return Err(redirect_guard(request, "/login"));
    }
    Ok(SessionState {
        session_id,
        admin_id: session.admin_id,
    })
}

pub fn session_cookie(session_id: Uuid, request: &HttpRequest) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, session_id.to_string())
        .path("/")
        .http_only(true)
        .secure(request_uses_https(request))
        .finish()
}

pub fn clear_session_cookie(request: &HttpRequest) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .secure(request_uses_https(request))
        .max_age(CookieDuration::seconds(0))
        .finish()
}

fn redirect_guard(request: &HttpRequest, location: &str) -> actix_web::HttpResponse {
    if is_hx_request(request) {
        actix_web::HttpResponse::Unauthorized()
            .append_header(("HX-Redirect", location))
            .finish()
    } else {
        actix_web::HttpResponse::Found()
            .append_header(("Location", location))
            .finish()
    }
}

fn is_hx_request(request: &HttpRequest) -> bool {
    request
        .headers()
        .get("HX-Request")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn request_uses_https(request: &HttpRequest) -> bool {
    request
        .connection_info()
        .scheme()
        .eq_ignore_ascii_case("https")
}
