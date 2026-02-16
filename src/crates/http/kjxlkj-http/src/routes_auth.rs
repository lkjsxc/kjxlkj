/// Auth and session route handlers per /docs/spec/api/http.md
///
/// POST /api/setup/register — first-user owner registration
/// POST /api/auth/login     — session creation (rate-limited per IMP-SEC-02)
/// POST /api/auth/logout    — session destruction
/// GET  /api/auth/session   — current session check
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, Json, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use kjxlkj_auth::AuthService;
use kjxlkj_db::user_repo::{UserRepo, SessionRepo};
use serde::Deserialize;
use std::net::SocketAddr;

/// POST /api/setup/register payload per /docs/spec/api/http.md
#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

/// POST /api/auth/login payload
#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

/// Extract client IP for rate limiting.
/// Falls back to "unknown" if ConnectInfo is not available.
fn client_ip(addr: Option<&SocketAddr>) -> String {
    addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string())
}

/// POST /api/setup/register
/// Per /docs/spec/security/auth.md: first user becomes owner.
/// Rate-limited per IMP-SEC-02.
pub async fn setup_register(
    State(state): State<AppState>,
    connect_info: Option<ConnectInfo<SocketAddr>>,
    Json(input): Json<RegisterInput>,
) -> Response {
    // Rate limit check
    let ip = client_ip(connect_info.as_ref().map(|c| &c.0));
    if let Err(retry_after) = state.auth_rate_limiter.check(&ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, retry_after.to_string())],
            Json(serde_json::json!({
                "code": "RATE_LIMITED",
                "message": "Too many requests. Try again later.",
                "details": null,
                "request_id": ""
            })),
        ).into_response();
    }
    let count = match state.user_repo.user_count() {
        Ok(c) => c,
        Err(e) => return domain_error_response(e),
    };
    if count > 0 {
        return domain_error_response(
            kjxlkj_domain::DomainError::SetupAlreadyCompleted,
        );
    }
    let user = match AuthService::build_owner_user(&input.username, &input.password) {
        Ok(u) => u,
        Err(e) => return domain_error_response(e),
    };
    if let Err(e) = state.user_repo.create_user(&user) {
        return domain_error_response(e);
    }
    (StatusCode::CREATED, Json(serde_json::json!({
        "message": "owner created",
        "username": user.username,
        "id": user.id,
    }))).into_response()
}

/// POST /api/auth/login
/// Per /docs/spec/security/sessions.md: create session cookie with 7-day TTL.
/// Rate-limited per IMP-SEC-02.
pub async fn auth_login(
    State(state): State<AppState>,
    connect_info: Option<ConnectInfo<SocketAddr>>,
    Json(input): Json<LoginInput>,
) -> Response {
    // Rate limit check
    let ip = client_ip(connect_info.as_ref().map(|c| &c.0));
    if let Err(retry_after) = state.auth_rate_limiter.check(&ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, retry_after.to_string())],
            Json(serde_json::json!({
                "code": "RATE_LIMITED",
                "message": "Too many requests. Try again later.",
                "details": null,
                "request_id": ""
            })),
        ).into_response();
    }
    let user = match state.user_repo.get_user_by_username(&input.username) {
        Ok(Some(u)) => u,
        Ok(None) => {
            return domain_error_response(kjxlkj_domain::DomainError::InvalidCredentials)
        }
        Err(e) => return domain_error_response(e),
    };
    if user.disabled {
        return domain_error_response(kjxlkj_domain::DomainError::InvalidCredentials);
    }
    let valid = match AuthService::verify_password(&input.password, &user.password_hash) {
        Ok(v) => v,
        Err(e) => return domain_error_response(e),
    };
    if !valid {
        return domain_error_response(kjxlkj_domain::DomainError::InvalidCredentials);
    }
    let session = AuthService::build_session(user.id, user.role);
    if let Err(e) = state.session_repo.create_session(&session) {
        return domain_error_response(e);
    }
    // Per /docs/spec/security/sessions.md: HttpOnly, SameSite=Lax, path=/
    let cookie_value = format!(
        "kjxlkj_session={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=604800",
        session.token
    );
    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie_value)],
        Json(serde_json::json!({
            "message": "session created",
            "token": session.token,
            "csrf_token": session.csrf_token,
            "expires_at": session.expires_at.to_string(),
        })),
    ).into_response()
}

/// POST /api/auth/logout
/// Per /docs/spec/security/sessions.md: delete session row and clear cookie.
pub async fn auth_logout(
    State(state): State<AppState>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    if let Some(token) = crate::middleware::extract_session_token(&req) {
        let _ = state.session_repo.delete_session(&token);
    }
    // Clear cookie per /docs/spec/security/sessions.md
    let clear_cookie = "kjxlkj_session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0";
    (StatusCode::NO_CONTENT, [(header::SET_COOKIE, clear_cookie)])
}

/// GET /api/auth/session
/// Per /docs/spec/api/http.md: returns current user info or unauthenticated.
pub async fn auth_session(
    State(state): State<AppState>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    if let Some(token) = crate::middleware::extract_session_token(&req) {
        if let Ok(Some(session)) = state.session_repo.get_session_by_token(&token) {
            return Json(serde_json::json!({
                "authenticated": true,
                "user_id": session.user_id,
                "role": format!("{:?}", session.role),
                "csrf_token": session.csrf_token,
                "expires_at": session.expires_at.to_string(),
            }));
        }
    }
    Json(serde_json::json!({ "authenticated": false }))
}
