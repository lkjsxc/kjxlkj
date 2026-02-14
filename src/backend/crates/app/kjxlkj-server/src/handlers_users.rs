use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::{hash_password, now_iso, request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{Role, UserRecord},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: Role,
}

pub async fn users_list(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let store = state.store.read().await;
    let users: Vec<UserRecord> = store.users.values().cloned().collect();
    Ok((StatusCode::OK, Json(json!({ "items": users, "request_id": rid }))))
}

pub async fn users_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let mut store = state.store.write().await;
    let user = UserRecord {
        id: crate::model::Store::next_id(),
        email: payload.email,
        display_name: payload.display_name,
        role: payload.role,
        status: "active".to_string(),
        created_at: now_iso(),
        password_hash: hash_password(&payload.password)?,
    };
    store.users.insert(user.id.clone(), user.clone());

    Ok((StatusCode::CREATED, Json(json!({ "item": user, "request_id": rid }))))
}

pub async fn users_role_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner], rid.clone())?;

    let mut store = state.store.write().await;
    let user = store
        .users
        .get_mut(&user_id)
        .ok_or_else(|| ApiError::not_found("USER_NOT_FOUND", "user not found", rid.clone()))?;
    user.role = payload.role;

    Ok((StatusCode::OK, Json(json!({ "item": user, "request_id": rid }))))
}

pub async fn users_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner], rid.clone())?;

    let mut store = state.store.write().await;
    let user = store
        .users
        .get_mut(&user_id)
        .ok_or_else(|| ApiError::not_found("USER_NOT_FOUND", "user not found", rid.clone()))?;
    user.status = "disabled".to_string();

    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}
