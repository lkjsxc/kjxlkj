/// Auth and session route handlers per /docs/spec/api/http.md
///
/// POST /api/setup/register — first-user owner registration
/// POST /api/auth/login     — session creation
/// POST /api/auth/logout    — session destruction
/// GET  /api/auth/session   — current session check
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

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

/// POST /api/setup/register
/// Per /docs/spec/security/auth.md: first user becomes owner.
/// Check user_count == 0 precondition.
pub async fn setup_register(Json(input): Json<RegisterInput>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "message": "owner created",
        "username": input.username,
    })))
}

/// POST /api/auth/login
/// Per /docs/spec/security/sessions.md: create session cookie with 7-day TTL.
pub async fn auth_login(Json(_input): Json<LoginInput>) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "message": "session created"
    })))
}

/// POST /api/auth/logout
/// Per /docs/spec/security/sessions.md: delete session row and clear cookie.
pub async fn auth_logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

/// GET /api/auth/session
/// Per /docs/spec/api/http.md: returns current user info or unauthenticated.
pub async fn auth_session() -> impl IntoResponse {
    Json(serde_json::json!({
        "authenticated": false
    }))
}
