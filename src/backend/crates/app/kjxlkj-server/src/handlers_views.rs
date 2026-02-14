use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    auth::{request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{Role, SavedViewRecord, Store},
    state::AppState,
};

#[derive(Deserialize)]
pub struct ViewsQuery {
    pub workspace_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateViewRequest {
    pub workspace_id: String,
    pub query_json: Option<Value>,
    pub sort: Option<String>,
    pub filters: Option<Value>,
}

#[derive(Deserialize)]
pub struct UpdateViewRequest {
    pub query_json: Option<Value>,
    pub sort: Option<String>,
    pub filters: Option<Value>,
}

pub async fn views_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ViewsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;

    let items = store
        .views
        .values()
        .filter(|item| {
            query
                .workspace_id
                .as_ref()
                .is_none_or(|workspace_id| item.workspace_id == *workspace_id)
        })
        .cloned()
        .collect::<Vec<SavedViewRecord>>();

    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn views_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateViewRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let item = SavedViewRecord {
        id: Store::next_id(),
        workspace_id: payload.workspace_id,
        query_json: payload.query_json.unwrap_or_else(|| json!({})),
        sort: payload.sort.unwrap_or_else(|| "updated_desc".to_string()),
        filters: payload.filters.unwrap_or_else(|| json!({})),
        owner_user_id: identity.user_id,
    };

    let mut store = state.store.write().await;
    store.views.insert(item.id.clone(), item.clone());

    Ok((StatusCode::CREATED, Json(json!({ "item": item, "request_id": rid }))))
}

pub async fn views_update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(view_id): Path<String>,
    Json(payload): Json<UpdateViewRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let item = store
        .views
        .get_mut(&view_id)
        .ok_or_else(|| ApiError::not_found("VIEW_NOT_FOUND", "saved view not found", rid.clone()))?;

    if let Some(query_json) = payload.query_json {
        item.query_json = query_json;
    }
    if let Some(sort) = payload.sort {
        item.sort = sort;
    }
    if let Some(filters) = payload.filters {
        item.filters = filters;
    }

    Ok((StatusCode::OK, Json(json!({ "item": item, "request_id": rid }))))
}

pub async fn views_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(view_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    store.views.remove(&view_id);

    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}
