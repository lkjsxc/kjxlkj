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
use crate::error_response::domain_error_response;
use crate::note_events::build_note_event;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use kjxlkj_db::repo::{NoteRepo, SearchRepo};
use kjxlkj_domain::event::NoteEventType;
use kjxlkj_domain::note::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListNotesQuery {
    pub workspace_id: Option<Uuid>,
    pub include_deleted: Option<bool>,
}
#[derive(Deserialize)]
pub struct CreateNoteBody {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub note_kind: Option<String>,
    pub markdown: Option<String>,
}
#[derive(Deserialize)]
pub struct PatchNoteBody { pub base_version: i64, pub markdown: Option<String> }
#[derive(Deserialize)]
pub struct UpdateTitleBody { pub base_version: i64, pub title: String }

pub async fn list_notes(State(s): State<AppState>, Query(q): Query<ListNotesQuery>) -> Response {
    let ws_id = match q.workspace_id {
        Some(id) => id,
        None => return domain_error_response(
            kjxlkj_domain::DomainError::BadRequest("workspace_id required".into()),
        ),
    };
    match s.note_repo.list_notes(ws_id, q.include_deleted.unwrap_or(false)) {
        Ok(notes) => Json(serde_json::to_value(&notes).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn create_note(State(s): State<AppState>, Json(input): Json<CreateNoteBody>) -> Response {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().naive_utc();
    let title = input.title.unwrap_or_else(default_note_title);
    let kind = input.note_kind.as_deref().and_then(NoteKind::from_str).unwrap_or(NoteKind::Markdown);
    let md = input.markdown.unwrap_or_default();
    let stream = NoteStream {
        id, workspace_id: input.workspace_id, project_id: input.project_id,
        title: title.clone(), note_kind: kind, access_scope: AccessScope::Workspace,
        state: NoteState::Active, current_version: 1, created_at: now, updated_at: now,
    };
    let proj = NoteProjection {
        note_id: id, title: title.clone(), version: 1, markdown: md.clone(),
        metadata_json: serde_json::json!({}), updated_at: now,
    };
    let evt = build_note_event(id, 1, NoteEventType::Created, serde_json::json!({"title": &title}));
    if let Err(e) = s.note_repo.create_note(&stream, &proj, &evt) {
        return domain_error_response(e);
    }
    s.search_repo.index_note(id, input.workspace_id, &title, &md);
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": id, "title": title, "workspace_id": input.workspace_id,
        "note_kind": kind.as_str(), "current_version": 1,
    }))).into_response()
}

pub async fn get_note(State(s): State<AppState>, Path(id): Path<Uuid>) -> Response {
    match s.note_repo.get_note_projection(id) {
        Ok(Some(p)) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Ok(None) => domain_error_response(kjxlkj_domain::DomainError::NoteNotFound),
        Err(e) => domain_error_response(e),
    }
}

pub async fn patch_note(
    State(s): State<AppState>, Path(id): Path<Uuid>, Json(input): Json<PatchNoteBody>,
) -> Response {
    let evt = build_note_event(id, input.base_version + 1, NoteEventType::BodyUpdated, serde_json::json!({}));
    match s.note_repo.update_note(id, input.base_version, input.markdown.as_deref(), None, &evt) {
        Ok(p) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn delete_note(State(s): State<AppState>, Path(id): Path<Uuid>) -> Response {
    let evt = build_note_event(id, 0, NoteEventType::SoftDeleted, serde_json::json!({}));
    match s.note_repo.soft_delete_note(id, &evt) {
        Ok(()) => {
            s.search_repo.remove_note(id);
            Json(serde_json::json!({"note_id": id, "state": "soft_deleted"})).into_response()
        }
        Err(e) => domain_error_response(e),
    }
}

pub async fn update_title(
    State(s): State<AppState>, Path(id): Path<Uuid>, Json(input): Json<UpdateTitleBody>,
) -> Response {
    let evt = build_note_event(
        id, input.base_version + 1, NoteEventType::TitleUpdated,
        serde_json::json!({"title": &input.title}),
    );
    match s.note_repo.update_note(id, input.base_version, None, Some(&input.title), &evt) {
        Ok(p) => Json(serde_json::to_value(&p).unwrap()).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn note_history(State(s): State<AppState>, Path(id): Path<Uuid>) -> Response {
    match s.note_repo.get_note_history(id) {
        Ok(events) => Json(serde_json::json!({
            "note_id": id, "events": serde_json::to_value(&events).unwrap(),
        })).into_response(),
        Err(e) => domain_error_response(e),
    }
}

pub async fn note_backlinks(State(s): State<AppState>, Path(id): Path<Uuid>) -> Response {
    match s.search_repo.get_backlinks(id) {
        Ok(blinks) => Json(serde_json::json!({
            "note_id": id, "backlinks": serde_json::to_value(&blinks).unwrap(),
        })).into_response(),
        Err(e) => domain_error_response(e),
    }
}
