use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    auth::{request_id, require_auth, require_csrf, require_role},
    error::ApiError,
    model::{NoteEvent, Role},
    state::{AppState, WsEnvelope},
};

#[derive(Deserialize)]
pub struct PatchNoteRequest {
    pub base_version: u64,
    pub patch_ops: Vec<Value>,
    pub idempotency_key: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchTitleRequest {
    pub base_version: u64,
    pub title: String,
}

pub async fn notes_patch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
    Json(payload): Json<PatchNoteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let rid = request_id(&headers);
    let identity = require_auth(&state, &headers, rid.clone()).await?;
    require_csrf(&headers, &identity, rid.clone())?;
    require_role(&identity, &[Role::Owner, Role::Admin, Role::Editor], rid.clone())?;

    let mut store = state.store.write().await;
    let key = payload.idempotency_key.clone();
    if let Some(existing) = {
        let note = store
            .notes
            .get(&note_id)
            .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;
        key.as_ref().and_then(|idempotency_key| {
            note.idempotency
                .get(idempotency_key)
                .map(|(version, event_seq)| (*version, *event_seq, note.clone()))
        })
    } {
        let (version, event_seq, snapshot) = existing;
        return Ok((
            StatusCode::OK,
            Json(json!({ "item": snapshot, "event_seq": event_seq, "version": version, "idempotency_replayed": true, "request_id": rid })),
        ));
    }

    let (version, markdown_snapshot) = {
        let note = store
            .notes
            .get_mut(&note_id)
            .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", rid.clone()))?;

        if payload.base_version != note.current_version {
            return Err(ApiError::conflict(
                "VERSION_CONFLICT",
                "stale base version",
                rid,
                payload.base_version,
                note.current_version,
            ));
        }

        let next_markdown = apply_patch_ops(&note.markdown, &payload.patch_ops, request_id(&headers))?;
        note.current_version += 1;
        note.markdown = next_markdown;
        note.history.push(NoteEvent {
            event_seq: note.current_version,
            version: note.current_version,
            event_type: "note_patched".to_string(),
            payload: json!({ "markdown": note.markdown }),
        });
        (note.current_version, note.markdown.clone())
    };

    let stream_id = format!("note:{note_id}");
    let event_seq = store.next_stream_seq(&stream_id);
    if let Some(idempotency_key) = key {
        if let Some(note) = store.notes.get_mut(&note_id) {
            note.idempotency
                .insert(idempotency_key, (version, event_seq));
        }
    }
    let ws_payload = json!({
        "type": "note_event",
        "note_id": note_id,
        "event_seq": event_seq,
        "version": version,
        "event_type": "note_patched",
        "payload": { "markdown": markdown_snapshot }
    });
    store.append_stream_event(&stream_id, ws_payload.clone());
    let snapshot = store
        .notes
        .get(&note_id)
        .cloned()
        .ok_or_else(|| ApiError::not_found("NOTE_NOT_FOUND", "note not found", request_id(&headers)))?;
    let _ = state.ws_tx.send(WsEnvelope { stream_id, payload: ws_payload });

    Ok((
        StatusCode::OK,
        Json(json!({ "item": snapshot, "event_seq": event_seq, "request_id": request_id(&headers) })),
    ))
}

pub async fn notes_title_patch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(note_id): Path<String>,
    Json(payload): Json<PatchTitleRequest>,
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
    if payload.base_version != note.current_version {
        return Err(ApiError::conflict(
            "VERSION_CONFLICT",
            "stale base version",
            rid,
            payload.base_version,
            note.current_version,
        ));
    }

    note.current_version += 1;
    note.title = payload.title;
    Ok((StatusCode::OK, Json(json!({ "item": note, "request_id": request_id(&headers) }))))
}

fn apply_patch_ops(base: &str, patch_ops: &[Value], request_id: String) -> Result<String, ApiError> {
    let mut cursor = 0usize;
    let mut out = String::new();
    for op in patch_ops {
        if let Some(retain) = op.get("retain").and_then(Value::as_u64) {
            let retain = retain as usize;
            if cursor + retain > base.len() {
                return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_PATCH", "retain overflow", request_id));
            }
            out.push_str(&base[cursor..cursor + retain]);
            cursor += retain;
        } else if let Some(insert) = op.get("insert").and_then(Value::as_str) {
            out.push_str(insert);
        } else if let Some(delete) = op.get("delete").and_then(Value::as_u64) {
            let delete = delete as usize;
            if cursor + delete > base.len() {
                return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_PATCH", "delete overflow", request_id));
            }
            cursor += delete;
        } else {
            return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_PATCH", "invalid op", request_id));
        }
    }
    out.push_str(&base[cursor..]);
    Ok(out)
}
