// Attachment repository per /docs/spec/domain/attachments.md
use kjxlkj_domain::types::Attachment;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_attachment(pool: &PgPool, a: &Attachment) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachments (id, note_id, filename, mime, size_bytes, sha256, chunk_count)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(a.id).bind(a.note_id).bind(&a.filename).bind(&a.mime)
    .bind(a.size_bytes).bind(&a.sha256).bind(a.chunk_count)
    .execute(pool).await?;
    Ok(())
}

pub async fn insert_chunk(pool: &PgPool, att_id: Uuid, idx: i32, data: &[u8]) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attachment_chunks (attachment_id, chunk_index, data)
         VALUES ($1, $2, $3)",
    )
    .bind(att_id).bind(idx).bind(data)
    .execute(pool).await?;
    Ok(())
}

pub async fn find_attachment(pool: &PgPool, id: Uuid) -> Result<Option<Attachment>, sqlx::Error> {
    let row: Option<(Uuid, Uuid, String, String, i64, String, i32)> = sqlx::query_as(
        "SELECT id, note_id, filename, mime, size_bytes, sha256, chunk_count
         FROM attachments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| Attachment {
        id: r.0, note_id: r.1, filename: r.2, mime: r.3,
        size_bytes: r.4, sha256: r.5, chunk_count: r.6,
    }))
}

pub async fn delete_attachment(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query("DELETE FROM attachment_chunks WHERE attachment_id = $1")
        .bind(id).execute(pool).await?;
    let result = sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(id).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

/// Max attachment size in bytes (500 MB per spec)
pub const MAX_ATTACHMENT_SIZE: i64 = 500 * 1024 * 1024;
