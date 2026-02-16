/// Note route handlers per /docs/spec/api/http.md
///
/// GET    /api/notes             — list notes
/// POST   /api/notes             — create note
/// GET    /api/notes/{id}        — get note projection
/// PATCH  /api/notes/{id}        — update body
/// DELETE /api/notes/{id}        — soft-delete
/// PATCH  /api/notes/{id}/title  — rename
/// GET    /api/notes/{id}/history   — event history
/// GET    /api/notes/{id}/backlinks — backlink list
use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

/// GET /api/notes query parameters
#[derive(Deserialize)]
pub struct ListNotesQuery {
    pub workspace_id: Option<Uuid>,
    pub include_deleted: Option<bool>,
}

/// POST /api/notes body per /docs/spec/api/types.md
#[derive(Deserialize)]
pub struct CreateNoteBody {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<String>,
    pub markdown: Option<String>,
}

/// PATCH /api/notes/{id} body per /docs/spec/api/types.md
#[derive(Deserialize)]
pub struct PatchNoteBody {
    pub base_version: i64,
    pub markdown: Option<String>,
}

/// PATCH /api/notes/{id}/title body
#[derive(Deserialize)]
pub struct UpdateTitleBody {
    pub base_version: i64,
    pub title: String,
}

/// GET /api/notes
pub async fn list_notes(Query(_q): Query<ListNotesQuery>) -> impl IntoResponse {
    Json(serde_json::json!([]))
}

/// POST /api/notes
/// Per /docs/spec/domain/notes.md: default title is "YYYY-MM-DD HH:mm:ss"
pub async fn create_note(Json(input): Json<CreateNoteBody>) -> impl IntoResponse {
    let title = input
        .title
        .unwrap_or_else(kjxlkj_domain::note::default_note_title);
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": Uuid::new_v4(),
        "title": title,
        "workspace_id": input.workspace_id,
        "note_kind": input.note_kind.unwrap_or_else(|| "markdown".into()),
        "current_version": 1,
    })))
}

/// GET /api/notes/{id}
pub async fn get_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "title": "stub",
        "version": 1,
        "markdown": "",
        "metadata_json": {},
    }))
}

/// PATCH /api/notes/{id}
/// Per /docs/spec/api/types.md: base_version for optimistic concurrency
pub async fn patch_note(
    Path(id): Path<Uuid>,
    Json(_input): Json<PatchNoteBody>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "version": 2,
    }))
}

/// DELETE /api/notes/{id}
/// Per /docs/spec/domain/notes.md: soft-delete sets state to soft_deleted
pub async fn delete_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "note_id": id,
        "state": "soft_deleted",
    })))
}

/// PATCH /api/notes/{id}/title
pub async fn update_title(
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTitleBody>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "title": input.title,
        "version": input.base_version + 1,
    }))
}

/// GET /api/notes/{id}/history
/// Per /docs/spec/domain/events.md: returns ordered event list
pub async fn note_history(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "events": [],
    }))
}

/// GET /api/notes/{id}/backlinks
/// Per /docs/spec/domain/search.md: backlinks are wiki-link sources
pub async fn note_backlinks(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(serde_json::json!({
        "note_id": id,
        "backlinks": [],
    }))
}
