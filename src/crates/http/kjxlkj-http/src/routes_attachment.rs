/// Attachment HTTP route handlers per /docs/spec/domain/attachments.md
///
/// POST   /api/notes/:id/attachments      — upload attachment
/// GET    /api/notes/:id/attachments      — list attachments for note
/// GET    /api/attachments/:id/download   — download attachment
/// DELETE /api/attachments/:id            — delete attachment
use crate::error_response::domain_error_response;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use kjxlkj_db::repo::AttachmentRepo;
use kjxlkj_domain::attachment::{AttachmentChunk, AttachmentMeta, CHUNK_SIZE, MAX_FILE_SIZE};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Upload input (simplified: base64-encoded body for in-memory impl)
#[derive(Deserialize)]
pub struct UploadInput {
    pub filename: String,
    pub content_type: String,
    /// Base64-encoded file data (for the in-memory path)
    pub data_base64: String,
}

/// POST /api/notes/:id/attachments
/// Per /docs/spec/domain/attachments.md: reject >500 MiB with 413.
pub async fn upload_attachment(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    Json(input): Json<UploadInput>,
) -> Response {
    let data = match base64_decode(&input.data_base64) {
        Some(d) => d,
        None => {
            return domain_error_response(kjxlkj_domain::DomainError::BadRequest(
                "invalid base64".into(),
            ))
        }
    };
    if data.len() > MAX_FILE_SIZE {
        return domain_error_response(kjxlkj_domain::DomainError::PayloadTooLarge);
    }
    // Compute SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256 = format!("{:x}", hasher.finalize());
    let att_id = Uuid::new_v4();
    let chunk_count = ((data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE).max(1) as i32;
    let meta = AttachmentMeta {
        id: att_id,
        note_id,
        filename: input.filename,
        content_type: input.content_type,
        size_bytes: data.len() as i64,
        sha256: sha256.clone(),
        chunk_count,
        created_at: chrono::Utc::now().naive_utc(),
    };
    if let Err(e) = state.attachment_repo.create_attachment(&meta) {
        return domain_error_response(e);
    }
    // Store chunks
    for (i, chunk_data) in data.chunks(CHUNK_SIZE).enumerate() {
        let mut ch = Sha256::new();
        ch.update(chunk_data);
        let chunk = AttachmentChunk {
            attachment_id: att_id,
            chunk_index: i as i32,
            data: chunk_data.to_vec(),
            sha256: format!("{:x}", ch.finalize()),
        };
        if let Err(e) = state.attachment_repo.store_chunk(&chunk) {
            return domain_error_response(e);
        }
    }
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": att_id,
            "filename": meta.filename,
            "size_bytes": meta.size_bytes,
            "sha256": sha256,
            "chunk_count": chunk_count,
        })),
    )
        .into_response()
}

/// GET /api/notes/:id/attachments
pub async fn list_attachments(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
) -> Response {
    match state.attachment_repo.list_attachments(note_id) {
        Ok(list) => (StatusCode::OK, Json(serde_json::json!(list))).into_response(),
        Err(e) => domain_error_response(e),
    }
}

/// GET /api/attachments/:id/download
/// Per /docs/spec/domain/attachments.md: stream chunks in index order.
pub async fn download_attachment(
    State(state): State<AppState>,
    Path(att_id): Path<Uuid>,
) -> Response {
    let meta = match state.attachment_repo.get_attachment(att_id) {
        Ok(Some(m)) => m,
        Ok(None) => {
            return domain_error_response(kjxlkj_domain::DomainError::NoteNotFound)
        }
        Err(e) => return domain_error_response(e),
    };
    let chunks = match state.attachment_repo.get_chunks(att_id) {
        Ok(c) => c,
        Err(e) => return domain_error_response(e),
    };
    let body: Vec<u8> = chunks.into_iter().flat_map(|c| c.data).collect();
    (
        StatusCode::OK,
        [
            ("content-type", meta.content_type.as_str()),
            ("content-disposition", &format!("attachment; filename=\"{}\"", meta.filename)),
        ],
        body,
    )
        .into_response()
}

/// DELETE /api/attachments/:id
/// Per /docs/spec/domain/attachments.md: remove meta and chunks atomically.
pub async fn delete_attachment(
    State(state): State<AppState>,
    Path(att_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.attachment_repo.delete_attachment(att_id) {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Simple base64 decode helper (no external dep needed beyond std)
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    use base64_engine::*;
    Some(STANDARD.decode(input).ok()?)
}

/// Minimal base64 engine (avoids adding a dep for this single use)
mod base64_engine {
    pub struct Base64Standard;
    pub const STANDARD: Base64Standard = Base64Standard;
    impl Base64Standard {
        pub fn decode(&self, input: &str) -> Result<Vec<u8>, ()> {
            let input = input.trim();
            if input.is_empty() {
                return Ok(Vec::new());
            }
            let lut: [u8; 256] = {
                let mut t = [255u8; 256];
                for (i, &c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
                    t[c as usize] = i as u8;
                }
                t[b'=' as usize] = 0;
                t
            };
            let bytes = input.as_bytes();
            if bytes.len() % 4 != 0 {
                return Err(());
            }
            let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
            for chunk in bytes.chunks(4) {
                let a = lut[chunk[0] as usize];
                let b = lut[chunk[1] as usize];
                let c = lut[chunk[2] as usize];
                let d = lut[chunk[3] as usize];
                if a == 255 || b == 255 {
                    return Err(());
                }
                out.push((a << 2) | (b >> 4));
                if chunk[2] != b'=' {
                    out.push((b << 4) | (c >> 2));
                }
                if chunk[3] != b'=' {
                    out.push((c << 6) | d);
                }
            }
            Ok(out)
        }
    }
}
