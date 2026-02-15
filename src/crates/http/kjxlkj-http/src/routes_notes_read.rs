//! Read-only note route handlers split from routes_notes.rs.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::repo_note;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{NoteId, WorkspaceId};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::*;
use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

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
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
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
        Ok(None) => domain_error_response(
            DomainError::NotFound("note".into()), &rid,
        ),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
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
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
