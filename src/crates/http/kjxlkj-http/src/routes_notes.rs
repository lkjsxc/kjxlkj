use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_note;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

// Re-export handlers from split modules for route wiring.
pub use crate::routes_notes_patch::{patch_note, patch_title};
pub use crate::routes_notes_read::{get_note, list_notes, note_history};
pub use crate::routes_notes_rollback::rollback_note;

/// Valid note kinds per /docs/spec/domain/note-types.md.
const VALID_KINDS: &[&str] = &["markdown", "canvas", "checklist", "template"];
/// Valid access scopes per /docs/spec/domain/permissions.md.
const VALID_SCOPES: &[&str] = &["workspace", "project", "private"];

/// POST /notes per /docs/spec/api/http.md.
pub async fn create_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateNoteRequest>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let ws_id = WorkspaceId(body.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(), ws_id, identity.user_id,
    ).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    let note_id = NoteId(Uuid::now_v7());
    let title = body.title.as_deref().unwrap_or("Untitled");
    let kind = body.note_kind.as_deref().unwrap_or("markdown");
    let access_scope = body.access_scope.as_deref().unwrap_or("workspace");

    // Validate note_kind
    if !VALID_KINDS.contains(&kind) {
        return domain_error_response(
            DomainError::BadRequest(format!("invalid note_kind: {kind}")),
            &rid,
        );
    }

    // Validate access_scope
    if !VALID_SCOPES.contains(&access_scope) {
        return domain_error_response(
            DomainError::BadRequest(
                format!("invalid access_scope: {access_scope}"),
            ),
            &rid,
        );
    }

    // Project scope requires project_id
    if access_scope == "project" && body.project_id.is_none() {
        return domain_error_response(
            DomainError::BadRequest(
                "project_id required for project scope".into(),
            ),
            &rid,
        );
    }

    match repo_note::create_note_stream(
        pool.get_ref(), note_id, ws_id,
        body.project_id, title, kind, access_scope,
    ).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "note_id": note_id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}

/// DELETE /notes/{id} per /docs/spec/api/http.md.
pub async fn delete_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    let stream = match repo_note::find_note_stream(
        pool.get_ref(), note_id,
    ).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };

    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(
        pool.get_ref(), ws_id, identity.user_id,
    ).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    match repo_note::soft_delete_note(pool.get_ref(), note_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
