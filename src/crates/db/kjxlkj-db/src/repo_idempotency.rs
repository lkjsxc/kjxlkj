use kjxlkj_domain::ids::NoteId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

/// Idempotency key record per /docs/spec/api/websocket.md.
#[derive(FromRow)]
pub struct IdempotencyRow {
    pub idempotency_key: String,
    pub note_id: Uuid,
    pub event_id: Uuid,
    pub version: i64,
    pub event_seq: i64,
}

/// Check if an idempotency key exists for a note.
/// Returns the existing commit identity if found.
pub async fn find_idempotency(
    pool: &PgPool,
    note_id: NoteId,
    key: &str,
) -> Result<Option<IdempotencyRow>, sqlx::Error> {
    sqlx::query_as::<_, IdempotencyRow>(
        "SELECT idempotency_key, note_id, event_id, version, event_seq
         FROM idempotency_keys
         WHERE note_id = $1 AND idempotency_key = $2",
    )
    .bind(note_id.0)
    .bind(key)
    .fetch_optional(pool)
    .await
}

/// Store an idempotency key after successful commit.
pub async fn store_idempotency(
    pool: &PgPool,
    note_id: NoteId,
    key: &str,
    event_id: Uuid,
    version: i64,
    event_seq: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO idempotency_keys
         (idempotency_key, note_id, event_id, version, event_seq)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (note_id, idempotency_key) DO NOTHING",
    )
    .bind(key)
    .bind(note_id.0)
    .bind(event_id)
    .bind(version)
    .bind(event_seq)
    .execute(pool)
    .await?;
    Ok(())
}

/// Clean up old idempotency keys (older than 24 hours).
pub async fn cleanup_old_keys(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM idempotency_keys
         WHERE created_at < now() - interval '24 hours'",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
