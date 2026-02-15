//! Snapshot and event-replay functions split from repo_note.rs.
//! Per docs/spec/domain/events.md (snapshot every 100 events).

use kjxlkj_domain::ids::NoteId;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

use crate::repo_note_event::NoteEventRow;

#[derive(FromRow)]
pub struct SnapshotRow {
    pub note_id: Uuid,
    pub at_seq: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
}

/// List note events from a given seq (for replay).
pub async fn list_note_events_from(
    pool: &PgPool,
    note_id: NoteId,
    from_seq: i64,
    limit: i64,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT event_id, note_id, seq, event_type,
                payload_json, actor_id, created_at
         FROM note_events
         WHERE note_id = $1 AND seq > $2
         ORDER BY seq ASC
         LIMIT $3",
    )
    .bind(note_id.0)
    .bind(from_seq)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Store a snapshot per /docs/spec/domain/events.md (every 100 events).
pub async fn store_snapshot(
    pool: &PgPool,
    note_id: NoteId,
    at_seq: i64,
    markdown: &str,
    metadata_json: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO note_snapshots (note_id, at_seq, markdown, metadata_json)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (note_id, at_seq) DO UPDATE
         SET markdown = $3, metadata_json = $4",
    )
    .bind(note_id.0)
    .bind(at_seq)
    .bind(markdown)
    .bind(metadata_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find the latest snapshot at or before a given seq.
pub async fn find_latest_snapshot(
    pool: &PgPool,
    note_id: NoteId,
    before_seq: i64,
) -> Result<Option<SnapshotRow>, sqlx::Error> {
    sqlx::query_as::<_, SnapshotRow>(
        "SELECT note_id, at_seq, markdown, metadata_json
         FROM note_snapshots
         WHERE note_id = $1 AND at_seq <= $2
         ORDER BY at_seq DESC
         LIMIT 1",
    )
    .bind(note_id.0)
    .bind(before_seq)
    .fetch_optional(pool)
    .await
}
