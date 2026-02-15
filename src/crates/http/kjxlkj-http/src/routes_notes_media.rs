//! Media note creation handler per /docs/spec/api/http.md.
//! POST /notes/media — create standalone media note from upload.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_note;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto_views::CreateMediaNoteRequest;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

const MEDIA_KINDS: &[&str] = &["media_image", "media_video"];

/// POST /notes/media per /docs/spec/api/http.md.
/// Creates a standalone media note stream.
pub async fn create_media_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateMediaNoteRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(body.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(),
        ws_id,
        identity.user_id,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    // Validate media note_kind per /docs/spec/domain/note-types.md.
    if !MEDIA_KINDS.contains(&body.note_kind.as_str()) {
        return domain_error_response(
            DomainError::BadRequest(format!(
                "note_kind must be media_image or media_video, got: {}",
                body.note_kind
            )),
            &rid,
        );
    }

    let note_id = NoteId(Uuid::now_v7());
    let title = body.title.as_deref().unwrap_or("Untitled Media");

    match repo_note::create_note_stream(
        pool.get_ref(),
        note_id,
        ws_id,
        body.project_id,
        title,
        &body.note_kind,
        "workspace",
    )
    .await
    {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "note_id": note_id.0,
            "request_id": rid
        })),
        Err(e) => {
            domain_error_response(DomainError::Internal(e.to_string()), &rid)
        }
    }
}
