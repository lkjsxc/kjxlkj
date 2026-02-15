//! Attachment download/delete handlers split from routes_attachments.rs.

use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::{repo_attachment, repo_note};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{AttachmentId, NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// GET /attachments/{id} per /docs/spec/api/http.md.
pub async fn download_attachment(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    if let Err(e) = extract_session(&req, pool.get_ref()).await {
        return domain_error_response(e, &rid);
    }
    let att_id = AttachmentId(path.into_inner());

    let att = match repo_attachment::find_attachment(
        pool.get_ref(), att_id,
    ).await {
        Ok(Some(a)) => a,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("attachment".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };

    let chunks = match repo_attachment::list_chunks(
        pool.get_ref(), att_id,
    ).await {
        Ok(c) => c,
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };

    // Verify chunk continuity
    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.chunk_index != i as i32 {
            return domain_error_response(
                DomainError::Internal("chunk continuity broken".into()),
                &rid,
            );
        }
    }

    let mut data = Vec::with_capacity(att.size_bytes as usize);
    for chunk in &chunks {
        data.extend_from_slice(&chunk.data);
    }

    HttpResponse::Ok()
        .content_type(att.mime.as_str())
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", att.filename),
        ))
        .body(data)
}

/// DELETE /attachments/{id} per /docs/spec/api/http.md.
pub async fn delete_attachment(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let att_id = AttachmentId(path.into_inner());

    let att = match repo_attachment::find_attachment(
        pool.get_ref(), att_id,
    ).await {
        Ok(Some(a)) => a,
        Ok(None) => return domain_error_response(
            DomainError::NotFound("attachment".into()), &rid,
        ),
        Err(e) => return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    };
    let note_id = NoteId(att.note_id);
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

    match repo_attachment::delete_attachment(pool.get_ref(), att_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => domain_error_response(
            DomainError::NotFound("attachment".into()), &rid,
        ),
        Err(e) => domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        ),
    }
}
