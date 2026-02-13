use crate::models::event::{NoteEventRow, WorkspaceEventRow};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn append_note_event(
    pool: &PgPool,
    id: Uuid,
    note_id: Uuid,
    version: i64,
    event_type: &str,
    payload: &serde_json::Value,
    actor_id: Uuid,
) -> Result<NoteEventRow, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "INSERT INTO note_events (id, note_id, seq,
         version, event_type, payload, actor_id)
         VALUES ($1, $2,
           (SELECT COALESCE(MAX(seq),0)+1 FROM note_events WHERE note_id=$2),
           $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(id)
    .bind(note_id)
    .bind(version)
    .bind(event_type)
    .bind(payload)
    .bind(actor_id)
    .fetch_one(pool)
    .await
}

pub async fn list_note_events(
    pool: &PgPool,
    note_id: Uuid,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT * FROM note_events WHERE note_id = $1 ORDER BY seq",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
}

pub async fn append_workspace_event(
    pool: &PgPool,
    id: Uuid,
    workspace_id: Uuid,
    event_type: &str,
    payload: &serde_json::Value,
    actor_id: Uuid,
) -> Result<WorkspaceEventRow, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceEventRow>(
        "INSERT INTO workspace_events (id, workspace_id, seq,
         event_type, payload, actor_id)
         VALUES ($1, $2,
           (SELECT COALESCE(MAX(seq),0)+1 FROM workspace_events WHERE workspace_id=$2),
           $3, $4, $5)
         RETURNING *",
    )
    .bind(id)
    .bind(workspace_id)
    .bind(event_type)
    .bind(payload)
    .bind(actor_id)
    .fetch_one(pool)
    .await
}

pub async fn list_workspace_events_after(
    pool: &PgPool,
    workspace_id: Uuid,
    after_seq: i64,
) -> Result<Vec<WorkspaceEventRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceEventRow>(
        "SELECT * FROM workspace_events
         WHERE workspace_id = $1 AND seq > $2 ORDER BY seq",
    )
    .bind(workspace_id)
    .bind(after_seq)
    .fetch_all(pool)
    .await
}

pub async fn list_note_events_after(
    pool: &PgPool,
    note_id: Uuid,
    after_seq: i64,
) -> Result<Vec<NoteEventRow>, sqlx::Error> {
    sqlx::query_as::<_, NoteEventRow>(
        "SELECT * FROM note_events WHERE note_id = $1 AND seq > $2 ORDER BY seq",
    )
    .bind(note_id)
    .bind(after_seq)
    .fetch_all(pool)
    .await
}
