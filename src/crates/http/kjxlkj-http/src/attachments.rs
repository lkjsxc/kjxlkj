//! Attachment handlers per /docs/spec/api/http.md.

use crate::dto::ApiError;
use crate::middleware;
use actix_web::{HttpRequest, HttpResponse, web};
use futures::StreamExt;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

const CHUNK_SIZE: usize = 1_048_576; // 1 MB chunks

/// POST /api/notes/{id}/attachments — chunked upload.
pub async fn upload(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
    mut payload: web::Payload,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let note_id = path.into_inner();
    let max_bytes = config.storage.max_attachment_mb * 1_048_576;
    // Read content-type and filename from headers
    let mime = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    let filename = req
        .headers()
        .get("x-filename")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("attachment")
        .to_string();
    // Collect body
    let mut body_bytes = Vec::new();
    while let Some(chunk) = payload.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ApiError::new("BAD_REQUEST", e.to_string()));
            }
        };
        body_bytes.extend_from_slice(&chunk);
        if body_bytes.len() as u64 > max_bytes {
            return HttpResponse::PayloadTooLarge().json(ApiError::new(
                "ATTACHMENT_TOO_LARGE",
                format!("exceeds {max_bytes} bytes"),
            ));
        }
    }
    let size = body_bytes.len() as i64;
    let sha = hex::encode(Sha256::digest(&body_bytes));
    let chunk_count = ((body_bytes.len() + CHUNK_SIZE - 1) / CHUNK_SIZE) as i32;
    let att_id = kjxlkj_domain::types::new_id();
    if let Err(e) = kjxlkj_db::repo::attachment::create_attachment(
        pool.get_ref(),
        att_id,
        note_id,
        &filename,
        &mime,
        size,
        &sha,
        chunk_count,
    )
    .await
    {
        return HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
    }
    // Store chunks
    for (i, chunk) in body_bytes.chunks(CHUNK_SIZE).enumerate() {
        if let Err(e) = kjxlkj_db::repo::attachment::add_chunk(
            pool.get_ref(),
            att_id,
            i as i32,
            chunk,
        )
        .await
        {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
        }
    }
    HttpResponse::Created().json(serde_json::json!({
        "id": att_id, "filename": filename,
        "size_bytes": size, "chunk_count": chunk_count, "sha256": sha
    }))
}

/// GET /api/attachments/{id} — download.
pub async fn download(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    let att_id = path.into_inner();
    let att = match kjxlkj_db::repo::attachment::get_attachment(
        pool.get_ref(),
        att_id,
    )
    .await
    {
        Ok(Some(a)) => a,
        Ok(None) => return middleware::not_found("attachment"),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
        }
    };
    let chunks =
        match kjxlkj_db::repo::attachment::get_chunks(pool.get_ref(), att_id)
            .await
        {
            Ok(c) => c,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(ApiError::new("INTERNAL_ERROR", e.to_string()));
            }
        };
    let body: Vec<u8> = chunks.into_iter().flat_map(|(_, d)| d).collect();
    HttpResponse::Ok()
        .content_type(att.mime)
        .insert_header(("content-disposition", format!("attachment; filename=\"{}\"", att.filename)))
        .body(body)
}

/// DELETE /api/attachments/{id}
pub async fn delete(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    config: web::Data<kjxlkj_db::config::AppConfig>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let ctx = match middleware::require_session(&req, pool.get_ref()).await {
        Ok(c) => c,
        Err(r) => return r,
    };
    if let Err(r) = middleware::validate_csrf(&req, &ctx, &config) {
        return r;
    }
    let att_id = path.into_inner();
    match kjxlkj_db::repo::attachment::delete_attachment(pool.get_ref(), att_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => middleware::not_found("attachment"),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiError::new("INTERNAL_ERROR", e.to_string())),
    }
}
