use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::{now_iso, request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{Role, WorkspaceRecord},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub slug: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: Role,
}

pub async fn workspaces_list(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let items: Vec<WorkspaceRecord> = store.workspaces.values().cloned().collect();
    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn workspaces_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let mut store = state.store.write().await;
    let workspace = WorkspaceRecord {
        id: crate::model::Store::next_id(),
        slug: payload.slug,
        name: payload.name,
        owner_user_id: identity.user_id.clone(),
        created_at: now_iso(),
    };
    store
        .workspace_members
        .entry(workspace.id.clone())
        .or_default()
        .insert(identity.user_id, Role::Owner);
    store
        .workspaces
        .insert(workspace.id.clone(), workspace.clone());

    Ok((StatusCode::CREATED, Json(json!({ "item": workspace, "request_id": rid }))))
}

pub async fn workspaces_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workspace_id): Path<String>,
    Json(payload): Json<UpdateWorkspaceRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let mut store = state.store.write().await;
    let workspace = store
        .workspaces
        .get_mut(&workspace_id)
        .ok_or_else(|| ApiError::not_found("WORKSPACE_NOT_FOUND", "workspace not found", rid.clone()))?;
    workspace.name = payload.name;
    Ok((StatusCode::OK, Json(json!({ "item": workspace, "request_id": rid }))))
}

pub async fn workspaces_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workspace_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner], rid.clone())?;

    let mut store = state.store.write().await;
    store.workspaces.remove(&workspace_id);
    store.workspace_members.remove(&workspace_id);
    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}

pub async fn workspace_members_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(workspace_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let members = store
        .workspace_members
        .get(&workspace_id)
        .cloned()
        .unwrap_or_default();
    Ok((StatusCode::OK, Json(json!({ "items": members, "request_id": rid }))))
}

pub async fn workspace_members_upsert(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((workspace_id, user_id)): Path<(String, String)>,
    Json(payload): Json<UpdateMemberRoleRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin], rid.clone())?;

    let mut store = state.store.write().await;
    store
        .workspace_members
        .entry(workspace_id)
        .or_default()
        .insert(user_id, payload.role);
    Ok((StatusCode::OK, Json(json!({ "request_id": rid }))))
}
