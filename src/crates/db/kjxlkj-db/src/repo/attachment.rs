//! Attachment repository per /docs/spec/domain/attachments.md.

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct AttachmentRow {
    pub id: Uuid,
    pub note_id: Uuid,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
}

/// Create attachment record (chunks added separately).
pub async fn create_attachment(
    pool: &PgPool, id: Uuid, note_id: Uuid, filename: &str,
    mime: &str, size_bytes: i64, sha256: &str, chunk_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachments (id,note_id,filename,mime,size_bytes,sha256,chunk_count) \
         VALUES ($1,$2,$3,$4,$5,$6,$7)"
    ).bind(id).bind(note_id).bind(filename).bind(mime)
     .bind(size_bytes).bind(sha256).bind(chunk_count)
     .execute(pool).await?;
    Ok(())
}

/// Add a chunk.
pub async fn add_chunk(
    pool: &PgPool, attachment_id: Uuid, chunk_index: i32, data: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachment_chunks (attachment_id, chunk_index, data) VALUES ($1,$2,$3)"
    ).bind(attachment_id).bind(chunk_index).bind(data)
     .execute(pool).await?;
    Ok(())
}

/// Get attachment metadata.
pub async fn get_attachment(
    pool: &PgPool, id: Uuid,
) -> Result<Option<AttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_, AttachmentRow>(
        "SELECT id, note_id, filename, mime, size_bytes, sha256, chunk_count \
         FROM attachments WHERE id = $1"
    ).bind(id).fetch_optional(pool).await
}

/// Get all chunks for an attachment in order.
pub async fn get_chunks(
    pool: &PgPool, attachment_id: Uuid,
) -> Result<Vec<(i32, Vec<u8>)>, sqlx::Error> {
    let rows: Vec<(i32, Vec<u8>)> = sqlx::query_as(
        "SELECT chunk_index, data FROM attachment_chunks \
         WHERE attachment_id = $1 ORDER BY chunk_index"
    ).bind(attachment_id).fetch_all(pool).await?;
    Ok(rows)
}

/// Delete attachment and its chunks atomically.
pub async fn delete_attachment(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let r = sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(id).execute(pool).await?;
    Ok(r.rows_affected() > 0)
}
