use kjxlkj_domain::ids::{AttachmentId, NoteId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// Attachment metadata row per /docs/spec/domain/attachments.md.
#[derive(FromRow)]
pub struct AttachmentRow {
    pub id: Uuid,
    pub note_id: Uuid,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
    pub created_at: OffsetDateTime,
}

/// Chunk row for streaming.
#[derive(FromRow)]
pub struct ChunkRow {
    pub chunk_index: i32,
    pub data: Vec<u8>,
}

/// Create attachment metadata record.
pub async fn create_attachment(
    pool: &PgPool,
    id: AttachmentId,
    note_id: NoteId,
    filename: &str,
    mime: &str,
    size_bytes: i64,
    sha256: &str,
    chunk_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachments
         (id, note_id, filename, mime, size_bytes, sha256, chunk_count)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id.0)
    .bind(note_id.0)
    .bind(filename)
    .bind(mime)
    .bind(size_bytes)
    .bind(sha256)
    .bind(chunk_count)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert a single chunk.
pub async fn insert_chunk(
    pool: &PgPool,
    attachment_id: AttachmentId,
    chunk_index: i32,
    data: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachment_chunks (attachment_id, chunk_index, data)
         VALUES ($1, $2, $3)",
    )
    .bind(attachment_id.0)
    .bind(chunk_index)
    .bind(data)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find attachment metadata.
pub async fn find_attachment(
    pool: &PgPool,
    id: AttachmentId,
) -> Result<Option<AttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_, AttachmentRow>(
        "SELECT id, note_id, filename, mime, size_bytes,
                sha256, chunk_count, created_at
         FROM attachments WHERE id = $1",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await
}

/// List attachment chunks in order.
pub async fn list_chunks(
    pool: &PgPool,
    attachment_id: AttachmentId,
) -> Result<Vec<ChunkRow>, sqlx::Error> {
    sqlx::query_as::<_, ChunkRow>(
        "SELECT chunk_index, data
         FROM attachment_chunks
         WHERE attachment_id = $1
         ORDER BY chunk_index ASC",
    )
    .bind(attachment_id.0)
    .fetch_all(pool)
    .await
}

/// Delete attachment and its chunks atomically.
/// Per /docs/spec/domain/attachments.md: deletion MUST remove
/// chunk rows and metadata row atomically.
pub async fn delete_attachment(
    pool: &PgPool,
    id: AttachmentId,
) -> Result<bool, sqlx::Error> {
    // CASCADE on attachment_chunks handles chunk deletion
    let result = sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(id.0)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// List attachments for a note.
pub async fn list_attachments_for_note(
    pool: &PgPool,
    note_id: NoteId,
) -> Result<Vec<AttachmentRow>, sqlx::Error> {
    sqlx::query_as::<_, AttachmentRow>(
        "SELECT id, note_id, filename, mime, size_bytes,
                sha256, chunk_count, created_at
         FROM attachments
         WHERE note_id = $1
         ORDER BY created_at DESC",
    )
    .bind(note_id.0)
    .fetch_all(pool)
    .await
}
