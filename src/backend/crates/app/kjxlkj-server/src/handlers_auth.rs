use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::{
        hash_password, issue_session, now_iso, request_id, require_auth, require_csrf,
        verify_password, SESSION_COOKIE,
    },
    error::ApiError,
    model::{Role, UserRecord},
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn setup_register(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let mut store = state.store.write().await;
    if store.owner_exists() {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "SETUP_LOCKED",
            "setup is locked",
            rid,
        ));
    }

    let user = UserRecord {
        id: crate::model::Store::next_id(),
        email: payload.email,
        display_name: payload.display_name,
        role: Role::Owner,
        status: "active".to_string(),
        created_at: now_iso(),
        password_hash: hash_password(&payload.password)?,
    };
    store.users.insert(user.id.clone(), user.clone());

    let session = issue_session(&mut store, user.id.clone());
    let mut response_headers = HeaderMap::new();
    let cookie = format!(
        "{SESSION_COOKIE}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
        session.id
    );
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).map_err(|_| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "failed to set cookie",
                rid.clone(),
            )
        })?,
    );

    Ok((
        StatusCode::CREATED,
        response_headers,
        Json(json!({
            "user": user,
            "csrf_token": session.csrf_token,
            "request_id": rid,
        })),
    ))
}

pub async fn auth_login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let mut store = state.store.write().await;
    let user = store
        .users
        .values()
        .find(|user| user.email.eq_ignore_ascii_case(&payload.email))
        .cloned()
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", "invalid credentials", rid.clone()))?;

    if !verify_password(&user.password_hash, &payload.password) || user.status != "active" {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "invalid credentials",
            rid,
        ));
    }

    let session = issue_session(&mut store, user.id.clone());
    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "{SESSION_COOKIE}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
            session.id
        ))
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "failed to set cookie", rid.clone()))?,
    );

    Ok((
        StatusCode::OK,
        response_headers,
        Json(json!({
            "user": user,
            "csrf_token": session.csrf_token,
            "request_id": rid,
        })),
    ))
}

pub async fn auth_logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;

    let mut store = state.store.write().await;
    store.sessions.retain(|_, session| session.user_id != identity.user_id);
    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}

pub async fn auth_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let user = store
        .users
        .get(&identity.user_id)
        .cloned()
        .ok_or_else(|| ApiError::auth_required(rid.clone()))?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "user": user,
            "csrf_token": identity.csrf_token,
            "request_id": rid
        })),
    ))
}
