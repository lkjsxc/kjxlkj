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
    model::{NoteEvent, NoteRecord, Role, Store},
    state::AppState,
};

#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub markdown: String,
    pub note_kind: Option<String>,
    pub access_scope: Option<String>,
}

pub async fn notes_create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let note = NoteRecord {
        id: Store::next_id(),
        workspace_id: payload.workspace_id,
        project_id: payload.project_id,
        title: payload.title,
        note_kind: payload.note_kind.unwrap_or_else(|| "markdown".to_string()),
        access_scope: payload.access_scope.unwrap_or_else(|| "workspace".to_string()),
        markdown: payload.markdown,
        current_version: 1,
        deleted: false,
        metadata_json: Default::default(),
        tags: Vec::new(),
        history: vec![NoteEvent {
            event_seq: 1,
            version: 1,
            event_type: "note_created".to_string(),
            payload: json!({ "version": 1 }),
        }],
        idempotency: Default::default(),
    };
    store.notes.insert(note.id.clone(), note.clone());

    Ok((StatusCode::CREATED, Json(json!({ "item": note, "request_id": rid }))))
}

pub async fn notes_list(State(state): State<AppState>, headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let _identity = require_auth(&state, &headers, rid.clone()).await?;
    let store = state.store.read().await;
    let notes: Vec<NoteRecord> = store
        .notes
        .values()
        .filter(|note| !note.deleted)
        .cloned()
        .collect();
    Ok((StatusCode::OK, Json(json!({ "items": notes, "request_id": rid }))))
}

pub async fn notes_get(
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
        .cloned()
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
    Ok((StatusCode::OK, Json(json!({ "item": note, "request_id": rid }))))
}
