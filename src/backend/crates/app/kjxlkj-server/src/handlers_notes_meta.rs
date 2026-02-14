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
    model::{Role, Store},
    state::AppState,
};

#[derive(Deserialize)]
pub struct SetMetadataRequest {
    pub value: Value,
}

#[derive(Deserialize)]
pub struct ReplaceTagsRequest {
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn note_metadata_upsert(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((note_id, key)): Path<(String, String)>,
    Json(payload): Json<SetMetadataRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let note = store
        .notes
        .get_mut(&note_id)
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
    note.metadata_json.insert(key, payload.value);

    Ok((StatusCode::OK, Json(json!({ "item": note, "request_id": rid }))))
}

pub async fn note_metadata_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((note_id, key)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let note = store
        .notes
        .get_mut(&note_id)
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
    note.metadata_json.remove(&key);

    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}

pub async fn tags_list(State(state): State<AppState>, headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let mut tags = store
        .notes
        .values()
        .flat_map(|note| note.tags.clone())
        .collect::<Vec<String>>();
    tags.sort();
    tags.dedup();
    Ok((StatusCode::OK, Json(json!({ "items": tags, "request_id": rid }))))
}

pub async fn note_tags_replace(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
    Json(payload): Json<ReplaceTagsRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let note = store
        .notes
        .get_mut(&note_id)
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
    note.tags = payload.tags;

    Ok((StatusCode::OK, Json(json!({ "item": note, "request_id": rid }))))
}

pub async fn note_backlinks(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let marker = format!("[[{note_id}]]");
    let store = state.store.read().await;
    let links = store
        .notes
        .values()
        .filter(|note| note.markdown.contains(&marker))
        .map(|note| json!({ "note_id": note.id, "title": note.title }))
        .collect::<Vec<Value>>();

    Ok((StatusCode::OK, Json(json!({ "items": links, "request_id": rid }))))
}

pub async fn search(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let term = query.q.unwrap_or_default().to_lowercase();
    let store = state.store.read().await;

    let items = store
        .notes
        .values()
        .filter(|note| !note.deleted)
        .filter(|note| {
            if term.is_empty() {
                return true;
            }
            note.title.to_lowercase().contains(&term) || note.markdown.to_lowercase().contains(&term)
        })
        .map(|note| json!({ "note_id": note.id, "title": note.title, "version": note.current_version }))
        .collect::<Vec<Value>>();

    Ok((StatusCode::OK, Json(json!({ "items": items, "request_id": rid }))))
}

pub async fn notes_media_create(headers: HeaderMap) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(json!({
            "item": {"id": Store::next_id(), "note_kind": "media_image"},
            "request_id": request_id(&headers)
        })),
    )
}
