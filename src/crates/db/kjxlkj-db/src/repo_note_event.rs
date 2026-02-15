use kjxlkj_domain::ids::{EventId, NoteId, UserId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow)]
pub struct NoteEventRow {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

pub async fn append_note_event(
    pool: &PgPool,
    event_id: EventId,
    note_id: NoteId,
    seq: i64,
    event_type: &str,
    payload_json: &serde_json::Value,
    actor_id: UserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_events
         (event_id, note_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(event_id.0)
    .bind(note_id.0)
    .bind(seq)
    .bind(event_type)
    .bind(payload_json)
    .bind(actor_id.0)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_note_events(
    pool: &PgPool,
    note_id: NoteId,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT event_id, note_id, seq, event_type,
                payload_json, actor_id, created_at
         FROM note_events WHERE note_id = $1
         ORDER BY seq ASC",
    )
    .bind(note_id.0)
    .fetch_all(pool)
    .await
}
