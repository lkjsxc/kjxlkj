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
    model::{ProjectRecord, Role},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub workspace_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn projects_list(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let items: Vec<ProjectRecord> = store.projects.values().cloned().collect();
    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn projects_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let item = ProjectRecord {
        id: crate::model::Store::next_id(),
        workspace_id: payload.workspace_id,
        name: payload.name,
        description: payload.description.unwrap_or_default(),
        created_at: now_iso(),
    };
    store.projects.insert(item.id.clone(), item.clone());
    Ok((StatusCode::CREATED, Json(json!({ "item": item, "request_id": rid }))))
}

pub async fn projects_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(project_id): Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let item = store
        .projects
        .get_mut(&project_id)
        .ok_or_else(|| ApiError::not_found("PROJECT_NOT_FOUND", "project not found", rid.clone()))?;
    if let Some(name) = payload.name {
        item.name = name;
    }
    if let Some(description) = payload.description {
        item.description = description;
    }
    Ok((StatusCode::OK, Json(json!({ "item": item, "request_id": rid }))))
}

pub async fn projects_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(project_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    store.projects.remove(&project_id);
    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}
