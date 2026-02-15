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

// Re-export patch handlers for route wiring.
pub use crate::routes_notes_patch::{patch_note, patch_title};

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
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(Uuid::now_v7());
    let title = body.title.as_deref().unwrap_or("Untitled");
    let kind = body.note_kind.as_deref().unwrap_or("markdown");
    match repo_note::create_note_stream(pool.get_ref(), note_id, ws_id, title, kind).await {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({
            "note_id": note_id.0,
            "request_id": rid
        })),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes per /docs/spec/api/http.md.
pub async fn list_notes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let ws_id_str = query.get("workspace_id").map(|s| s.as_str()).unwrap_or("");
    let ws_id = match ws_id_str.parse::<Uuid>() {
        Ok(u) => WorkspaceId(u),
        Err(_) => {
            return domain_error_response(
                DomainError::BadRequest("workspace_id required".into()),
                &rid,
            )
        }
    };
    match repo_note::list_notes(pool.get_ref(), ws_id).await {
        Ok(rows) => {
            let list: Vec<NoteStreamResponse> = rows
                .into_iter()
                .map(|n| NoteStreamResponse {
                    id: n.id,
                    workspace_id: n.workspace_id,
                    project_id: n.project_id,
                    title: n.title,
                    note_kind: n.note_kind,
                    current_version: n.current_version,
                    created_at: n.created_at.to_string(),
                    updated_at: n.updated_at.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes/{id} per /docs/spec/api/http.md.
pub async fn get_note(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(path.into_inner());
    match repo_note::find_note_projection(pool.get_ref(), note_id).await {
        Ok(Some(p)) => HttpResponse::Ok().json(NoteProjectionResponse {
            note_id: p.note_id,
            workspace_id: p.workspace_id,
            title: p.title,
            note_kind: p.note_kind,
            version: p.version,
            markdown: p.markdown,
            rendered_html: p.rendered_html,
            metadata_json: p.metadata_json,
        }),
        Ok(None) => domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// DELETE /notes/{id} per /docs/spec/api/http.md.
/// Returns 204 on successful soft-delete.
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

    let stream = match repo_note::find_note_stream(pool.get_ref(), note_id).await {
        Ok(Some(s)) => s,
        Ok(None) => return domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => return domain_error_response(DomainError::Internal(e.to_string()), &rid),
    };

    let ws_id = WorkspaceId(stream.workspace_id);
    let role = match guard::resolve_workspace_role(pool.get_ref(), ws_id, identity.user_id).await {
        Ok(r) => r,
        Err(e) => return domain_error_response(e, &rid),
    };
    if let Err(e) = guard::require_editor(role) {
        return domain_error_response(e, &rid);
    }

    match repo_note::soft_delete_note(pool.get_ref(), note_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(DomainError::NotFound("note".into()), &rid),
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}

/// GET /notes/{id}/history per /docs/spec/api/http.md.
pub async fn note_history(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let note_id = NoteId(path.into_inner());
    match repo_note::list_note_events(pool.get_ref(), note_id).await {
        Ok(events) => {
            let list: Vec<serde_json::Value> = events
                .into_iter()
                .map(|e| serde_json::json!({
                    "event_id": e.event_id,
                    "note_id": e.note_id,
                    "seq": e.seq,
                    "event_type": e.event_type,
                    "payload_json": e.payload_json,
                    "actor_id": e.actor_id,
                    "created_at": e.created_at.to_string()
                }))
                .collect();
            HttpResponse::Ok().json(list)
        }
        Err(e) => domain_error_response(DomainError::Internal(e.to_string()), &rid),
    }
}
