use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::{request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::Role,
    state::AppState,
};

#[derive(Deserialize)]
pub struct RollbackRequest {
    pub version: u64,
}

pub async fn notes_delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
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
    note.deleted = true;
    Ok((StatusCode::NO_CONTENT, Json(json!({ "request_id": rid }))))
}

pub async fn notes_history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let note = store
        .notes
        .get(&note_id)
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
    Ok((StatusCode::OK, Json(json!({ "items": note.history, "request_id": rid }))))
}

pub async fn notes_rollback(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
    Json(payload): Json<RollbackRequest>,
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

    let payload_value = note
        .history
        .iter()
        .find(|event| event.version == payload.version)
        .and_then(|event| event.payload.get("markdown"))
        .cloned()
        .ok_or_else(|| ApiError::not_found("VERSION_NOT_FOUND", "rollback version not found", rid.clone()))?;

    let markdown = payload_value.as_str().unwrap_or_default().to_string();
    note.current_version += 1;
    note.markdown = markdown;
    Ok((StatusCode::OK, Json(json!({ "item": note, "request_id": rid }))))
}
