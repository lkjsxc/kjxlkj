use actix_web::cookie::{time::Duration as CookieDuration, Cookie};
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::error::AppError;
use crate::web::session::{session_id_from_request, SessionState};
use crate::web::state::WebState;

pub const SESSION_COOKIE_NAME: &str = "session_id";

pub async fn has_admin_user(state: &web::Data<WebState>) -> Result<bool, HttpResponse> {
    state
        .admin_store
        .has_admin_user()
        .await
        .map_err(internal_error)
}

pub async fn enforce_setup_completion(state: &web::Data<WebState>) -> Result<(), HttpResponse> {
    match has_admin_user(state).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(redirect_to_setup()),
        Err(response) => Err(response),
    }
}

pub async fn enforce_setup_pending(state: &web::Data<WebState>) -> Result<(), HttpResponse> {
    match has_admin_user(state).await {
        Ok(false) => Ok(()),
        Ok(true) => Err(HttpResponse::NotFound().finish()),
        Err(response) => Err(response),
    }
}

pub async fn require_admin_session(
    request: &HttpRequest,
    state: &web::Data<WebState>,
) -> Result<SessionState, HttpResponse> {
    enforce_setup_completion(state).await?;

    let Some(session_id) = session_id_from_request(request) else {
        return Err(redirect_to_login());
    };

    let now = Utc::now();
    let session = state
        .session_store
        .lookup_session(session_id)
        .await
        .map_err(internal_error)?;
    let Some(session) = session else {
        return Err(redirect_to_login());
    };
    if session.expires_at <= now {
        let _ = state.session_store.delete_session(session_id).await;
        return Err(redirect_to_login());
    }

    Ok(SessionState {
        session_id,
        admin_id: session.admin_id,
    })
}

pub fn internal_error(error: AppError) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!("{}: {}", error.code(), error))
}

pub fn redirect_to_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "/login"))
        .finish()
}

pub fn redirect_to_setup() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "/setup"))
        .finish()
}

pub fn session_cookie(session_id: Uuid) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, session_id.to_string())
        .path("/")
        .http_only(true)
        .secure(true)
        .finish()
}

pub fn clear_session_cookie() -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(CookieDuration::seconds(0))
        .finish()
}
