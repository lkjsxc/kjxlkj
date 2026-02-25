//! Authentication handlers

use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::Json,
    Extension,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kjxlkj_auth::{Session, SessionStore, generate_csrf_token};
use crate::state::{HttpResult, HttpError};
use crate::routes::HttpState;

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub email: String,
}

/// Register request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

/// Register response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub email: String,
}

/// Session response
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub user_id: Uuid,
    pub email: String,
}

/// Login handler
pub async fn login(
    State(state): State<HttpState>,
    Json(req): Json<LoginRequest>,
) -> HttpResult<(StatusCode, Json<LoginResponse>, HeaderMap)> {
    // Stub implementation - accept any credentials
    let user_id = Uuid::new_v4();
    
    // Create session
    let csrf_token = generate_csrf_token();
    let session = Session::new(user_id, csrf_token, 24);
    state.session_store.create(session).await;

    let mut headers = HeaderMap::new();
    // In production, set session cookie here

    let response = LoginResponse {
        user_id,
        email: req.email,
    };

    Ok((StatusCode::OK, Json(response), headers))
}

/// Logout handler
pub async fn logout(
    State(state): State<HttpState>,
) -> HttpResult<StatusCode> {
    // Stub implementation
    Ok(StatusCode::NO_CONTENT)
}

/// Get current session handler
pub async fn get_session(
    State(state): State<HttpState>,
) -> HttpResult<Json<SessionResponse>> {
    // Stub implementation - return test user
    Ok(Json(SessionResponse {
        user_id: Uuid::new_v4(),
        email: "user@example.com".to_string(),
    }))
}

/// Register handler (first-run setup)
pub async fn register(
    State(state): State<HttpState>,
    Json(req): Json<RegisterRequest>,
) -> HttpResult<(StatusCode, Json<RegisterResponse>)> {
    if req.password != req.password_confirm {
        return Err(HttpError::BadRequest("Passwords do not match".into()));
    }

    // Stub implementation - create owner user
    let user_id = Uuid::new_v4();

    let response = RegisterResponse {
        user_id,
        email: req.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
