use actix_web::{web, HttpRequest, HttpResponse};
use kjxlkj_db::{repo_attachment, repo_note};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{AttachmentId, NoteId, WorkspaceId};
use kjxlkj_rbac::guard;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error_response::{domain_error_response, new_request_id};
use crate::extractors::extract_session;

/// Maximum file size per /docs/spec/domain/attachments.md: 500 MiB.
const MAX_FILE_SIZE: usize = 500 * 1024 * 1024;
/// Chunk size per /docs/spec/domain/attachments.md: 4 MiB.
const CHUNK_SIZE: usize = 4 * 1024 * 1024;

/// POST /notes/{id}/attachments per /docs/spec/api/http.md.
/// Accepts raw binary body, computes SHA-256, stores in chunks.
pub async fn upload_attachment(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Bytes,
) -> HttpResponse {
    let rid = new_request_id();
    let identity = match extract_session(&req, pool.get_ref()).await {
        Ok(i) => i,
        Err(e) => return domain_error_response(e, &rid),
    };
    let note_id = NoteId(path.into_inner());

    // Verify note exists and check RBAC
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

    let data = body.to_vec();
    if data.len() > MAX_FILE_SIZE {
        return domain_error_response(
            DomainError::BadRequest("file too large".into()), &rid,
        );
    }

    // Extract filename from Content-Disposition or default
    let filename = req
        .headers()
        .get("X-Filename")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("attachment")
        .to_string();

    let mime = req
        .headers()
        .get("Content-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    // Compute SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256 = hex::encode(hasher.finalize());

    let att_id = AttachmentId(Uuid::now_v7());
    let size = data.len() as i64;
    let chunk_count = ((data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE) as i32;

    if let Err(e) = repo_attachment::create_attachment(
        pool.get_ref(), att_id, note_id,
        &filename, &mime, size, &sha256, chunk_count,
    ).await {
        return domain_error_response(
            DomainError::Internal(e.to_string()), &rid,
        );
    }

    // Store chunks
    for (i, chunk_data) in data.chunks(CHUNK_SIZE).enumerate() {
        if let Err(e) = repo_attachment::insert_chunk(
            pool.get_ref(), att_id, i as i32, chunk_data,
        ).await {
            return domain_error_response(
                DomainError::Internal(e.to_string()), &rid,
            );
        }
    }

    HttpResponse::Created().json(serde_json::json!({
        "attachment_id": att_id.0,
        "filename": filename,
        "size_bytes": size,
        "sha256": sha256,
        "chunk_count": chunk_count,
        "request_id": rid
    }))
}

// Re-export download/delete handlers from split module.
pub use crate::routes_attachments_dl::{
    delete_attachment, download_attachment,
};
