//! Note handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use kjxlkj_domain::{NoteStream, NoteKind, Actor};
use kjxlkj_db::NoteRepo;
use crate::state::{HttpResult, HttpError};
use crate::routes::HttpState;

/// Create note request
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub note_kind: Option<String>,
}

/// Create note response
#[derive(Debug, Serialize)]
pub struct CreateNoteResponse {
    pub note_id: Uuid,
    pub title: String,
    pub markdown: String,
    pub version: u64,
    pub created_at: chrono::DateTime<Utc>,
}

/// List notes query params
#[derive(Debug, Deserialize)]
pub struct ListNotesQuery {
    workspace_id: Uuid,
    project_id: Option<Uuid>,
    limit: Option<usize>,
    offset: Option<usize>,
}

/// List notes response
#[derive(Debug, Serialize)]
pub struct ListNotesResponse {
    pub notes: Vec<NoteSummary>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct NoteSummary {
    pub note_id: Uuid,
    pub title: String,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Create a new note
pub async fn create_note(
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<CreateNoteRequest>,
) -> HttpResult<(StatusCode, Json<CreateNoteResponse>)> {
    let note_repo = NoteRepo::new();
    
    // Create note with datetime title if not provided
    let mut note = if let Some(title) = req.title {
        NoteStream::new(
            title,
            req.markdown.unwrap_or_default(),
            req.workspace_id,
            req.project_id,
            NoteKind::Note,
        )
    } else {
        NoteStream::new_with_datetime_title(
            req.markdown.unwrap_or_default(),
            req.workspace_id,
            req.project_id,
            NoteKind::Note,
        )
    };

    let actor = Actor::User { user_id };
    let created = note_repo.create(note, &actor).await
        .map_err(HttpError::from)?;

    let response = CreateNoteResponse {
        note_id: created.note_id,
        title: created.title.clone(),
        markdown: created.markdown,
        version: created.version,
        created_at: created.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// List notes
pub async fn list_notes(
    Query(params): Query<ListNotesQuery>,
) -> HttpResult<Json<ListNotesResponse>> {
    let note_repo = NoteRepo::new();
    
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let notes = note_repo
        .list_by_workspace(params.workspace_id, limit, offset)
        .await
        .map_err(HttpError::from)?;

    let total = notes.len();
    let summaries: Vec<NoteSummary> = notes
        .into_iter()
        .map(|n| NoteSummary {
            note_id: n.note_id,
            title: n.title,
            updated_at: n.updated_at,
        })
        .collect();

    Ok(Json(ListNotesResponse {
        notes: summaries,
        total,
    }))
}

/// Get note by ID
pub async fn get_note(
    State(state): State<HttpState>,
    Path(note_id): Path<Uuid>,
) -> HttpResult<Json<NoteStream>> {
    let note_repo = NoteRepo::new();
    
    let note = note_repo
        .get(note_id)
        .await
        .map_err(HttpError::from)?
        .ok_or_else(|| HttpError::NotFound(format!("Note {} not found", note_id)))?;

    Ok(Json(note))
}

/// Update note
pub async fn update_note(
    State(_state): State<HttpState>,
    Path(note_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> HttpResult<Json<NoteStream>> {
    let note_repo = NoteRepo::new();
    
    let actor = Actor::User { user_id };
    let markdown = req.markdown.clone();
    
    let updated = note_repo
        .update(note_id, req.base_version, |note| {
            if let Some(_md) = &markdown {
                note.markdown = _md.clone();
                Some(kjxlkj_domain::NoteEvent::Updated {
                    patch_ops: vec![],
                    new_version: note.version + 1,
                })
            } else {
                None
            }
        }, &actor)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(updated))
}

/// Update title only
pub async fn update_title(
    State(_state): State<HttpState>,
    Path(note_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<UpdateTitleRequest>,
) -> HttpResult<Json<NoteStream>> {
    let note_repo = NoteRepo::new();
    
    let actor = Actor::User { user_id };
    let new_title = req.title.clone();
    
    let updated = note_repo
        .update(note_id, req.base_version, |note| {
            let old_title = note.title.clone();
            note.set_title(new_title.clone());
            Some(kjxlkj_domain::NoteEvent::TitleChanged {
                old_title,
                new_title: new_title.clone(),
            })
        }, &actor)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(updated))
}

/// Delete note
pub async fn delete_note(
    State(state): State<HttpState>,
    Path(note_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<DeleteNoteRequest>,
) -> HttpResult<StatusCode> {
    let note_repo = NoteRepo::new();
    
    let actor = Actor::User { user_id };
    
    note_repo
        .delete(note_id, req.base_version, &actor)
        .await
        .map_err(HttpError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get note history
pub async fn get_history(
    State(state): State<HttpState>,
    Path(note_id): Path<Uuid>,
) -> HttpResult<Json<Vec<kjxlkj_domain::DomainEvent>>> {
    let note_repo = NoteRepo::new();
    
    let events = note_repo
        .get_events(note_id)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(events))
}

/// Get backlinks
pub async fn get_backlinks(
    State(state): State<HttpState>,
    Path(note_id): Path<Uuid>,
) -> HttpResult<Json<kjxlkj_domain::BacklinkResponse>> {
    // Stub implementation
    Ok(Json(kjxlkj_domain::BacklinkResponse {
        note_id,
        backlinks: vec![],
        total: 0,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub base_version: u64,
    pub markdown: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTitleRequest {
    pub base_version: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteNoteRequest {
    pub base_version: u64,
}
