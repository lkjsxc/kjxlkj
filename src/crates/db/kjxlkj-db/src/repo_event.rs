use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(FromRow, serde::Serialize)]
pub struct NoteEventRow {
    pub id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: Uuid,
    pub actor_type: String,
    pub created_at: OffsetDateTime,
}

/// Get note event history.
pub async fn get_note_history(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT id, note_id, seq, event_type, payload,
                actor_id, actor_type, created_at
         FROM note_events
         WHERE note_id = $1
         ORDER BY seq ASC"
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
}

/// Get events after a certain sequence (for replay).
pub async fn get_events_after(
    pool: &PgPool,
    note_id: Uuid,
    after_seq: i64,
    limit: i64,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT id, note_id, seq, event_type, payload,
                actor_id, actor_type, created_at
         FROM note_events
         WHERE note_id = $1 AND seq > $2
         ORDER BY seq ASC
         LIMIT $3"
    )
    .bind(note_id)
    .bind(after_seq)
    .bind(limit)
    .fetch_all(pool)
    .await
}
