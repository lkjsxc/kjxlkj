use actix_web::{web, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    auth::{auth_session, enforce_csrf},
    error::AppError,
    models::AttachmentMeta,
};

pub async fn upload_attachment(
    req: HttpRequest,
    state: web::Data<AppState>,
    note_id: web::Path<Uuid>,
    mut payload: web::Payload,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;

    let note_id = note_id.into_inner();
    let exists: Option<Uuid> =
        sqlx::query_scalar("select id from note_streams where id = $1 and deleted_at is null")
            .bind(note_id)
            .fetch_optional(&state.pool)
            .await?;
    if exists.is_none() {
        return Err(AppError::NotFound("note not found".to_string()));
    }

    let filename = req
        .headers()
        .get("x-filename")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("upload.bin")
        .to_string();
    let mime = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let mut tx = state.pool.begin().await?;
    let attachment_id = Uuid::now_v7();
    sqlx::query("insert into attachments (id, note_id, filename, mime, size_bytes, sha256, chunk_count) values ($1, $2, $3, $4, 0, '', 0)")
        .bind(attachment_id)
        .bind(note_id)
        .bind(&filename)
        .bind(&mime)
        .execute(&mut *tx)
        .await?;

    let mut total: i64 = 0;
    let mut chunk_index: i32 = 0;
    let mut file_hash = Sha256::new();
    let mut carry = Vec::with_capacity(state.config.attachment_chunk_bytes + 1024);

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|_| AppError::BadRequest("invalid upload chunk".to_string()))?;
        total += chunk.len() as i64;
        if total > state.config.attachment_max_bytes {
            return Err(AppError::PayloadTooLarge);
        }
        file_hash.update(&chunk);
        carry.extend_from_slice(&chunk);

        while carry.len() >= state.config.attachment_chunk_bytes {
            let split: Vec<u8> = carry.drain(..state.config.attachment_chunk_bytes).collect();
            insert_chunk(&mut tx, attachment_id, chunk_index, split).await?;
            chunk_index += 1;
        }
    }

    if !carry.is_empty() {
        insert_chunk(&mut tx, attachment_id, chunk_index, carry.clone()).await?;
        chunk_index += 1;
    }

    let digest = hex::encode(file_hash.finalize());
    sqlx::query(
        "update attachments set size_bytes = $2, sha256 = $3, chunk_count = $4 where id = $1",
    )
    .bind(attachment_id)
    .bind(total)
    .bind(digest)
    .bind(chunk_index)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    let meta = sqlx::query_as::<_, AttachmentMeta>(
        "select id, note_id, filename, mime, size_bytes, sha256, chunk_count, created_at from attachments where id = $1",
    )
    .bind(attachment_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(HttpResponse::Created().json(meta))
}

pub async fn download_attachment(
    state: web::Data<AppState>,
    attachment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let attachment_id = attachment_id.into_inner();
    let meta = sqlx::query_as::<_, AttachmentMeta>(
        "select id, note_id, filename, mime, size_bytes, sha256, chunk_count, created_at from attachments where id = $1",
    )
    .bind(attachment_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("attachment not found".to_string()))?;

    let rows = sqlx::query_as::<_, (i32, Vec<u8>)>(
        "select chunk_index, bytes from attachment_chunks where attachment_id = $1 order by chunk_index asc",
    )
    .bind(attachment_id)
    .fetch_all(&state.pool)
    .await?;
    if rows.len() != meta.chunk_count as usize {
        return Err(AppError::Internal);
    }
    for (expected, (idx, _)) in rows.iter().enumerate() {
        if *idx != expected as i32 {
            return Err(AppError::Internal);
        }
    }

    let stream = futures_util::stream::iter(
        rows.into_iter()
            .map(|(_, bytes)| Ok::<Bytes, actix_web::Error>(Bytes::from(bytes))),
    );
    Ok(HttpResponse::Ok()
        .append_header(("content-type", meta.mime))
        .append_header((
            "content-disposition",
            format!("attachment; filename=\"{}\"", meta.filename),
        ))
        .streaming(stream))
}

pub async fn delete_attachment(
    req: HttpRequest,
    state: web::Data<AppState>,
    attachment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let session = auth_session(&req, &state).await?;
    enforce_csrf(&req, &session)?;
    let attachment_id = attachment_id.into_inner();

    let mut tx = state.pool.begin().await?;
    sqlx::query("delete from attachment_chunks where attachment_id = $1")
        .bind(attachment_id)
        .execute(&mut *tx)
        .await?;
    let deleted = sqlx::query("delete from attachments where id = $1")
        .bind(attachment_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    tx.commit().await?;

    if deleted == 0 {
        return Err(AppError::NotFound("attachment not found".to_string()));
    }
    Ok(HttpResponse::NoContent().finish())
}

async fn insert_chunk(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    attachment_id: Uuid,
    idx: i32,
    bytes: Vec<u8>,
) -> Result<(), AppError> {
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let sha = hex::encode(hasher.finalize());
    sqlx::query("insert into attachment_chunks (attachment_id, chunk_index, bytes, sha256) values ($1, $2, $3, $4)")
        .bind(attachment_id)
        .bind(idx)
        .bind(bytes)
        .bind(sha)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
