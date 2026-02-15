use kjxlkj_domain::ids::{EventId, UserId, WorkspaceId};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

/// Workspace event row per /docs/spec/domain/events.md.
#[derive(FromRow)]
pub struct WorkspaceEventRow {
    pub event_id: Uuid,
    pub workspace_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

/// Get next workspace event sequence number.
pub async fn next_workspace_seq(
    pool: &PgPool,
    workspace_id: WorkspaceId,
) -> Result<i64, sqlx::Error> {
    #[derive(FromRow)]
    struct MaxRow {
        max_seq: Option<i64>,
    }
    let row = sqlx::query_as::<_, MaxRow>(
        "SELECT MAX(seq) as max_seq FROM workspace_events
         WHERE workspace_id = $1",
    )
    .bind(workspace_id.0)
    .fetch_one(pool)
    .await?;
    Ok(row.max_seq.unwrap_or(0) + 1)
}

/// Append a workspace event.
pub async fn append_workspace_event(
    pool: &PgPool,
    event_id: EventId,
    workspace_id: WorkspaceId,
    seq: i64,
    event_type: &str,
    payload_json: &serde_json::Value,
    actor_id: UserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_events
         (event_id, workspace_id, seq, event_type, payload_json, actor_id)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(event_id.0)
    .bind(workspace_id.0)
    .bind(seq)
    .bind(event_type)
    .bind(payload_json)
    .bind(actor_id.0)
    .execute(pool)
    .await?;
    Ok(())
}

/// List workspace events from a given seq (for replay).
pub async fn list_workspace_events_from(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    from_seq: i64,
    limit: i64,
) -> Result<Vec<WorkspaceEventRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceEventRow>(
        "SELECT event_id, workspace_id, seq, event_type,
                payload_json, actor_id, created_at
         FROM workspace_events
         WHERE workspace_id = $1 AND seq > $2
         ORDER BY seq ASC
         LIMIT $3",
    )
    .bind(workspace_id.0)
    .bind(from_seq)
    .bind(limit)
    .fetch_all(pool)
    .await
}
