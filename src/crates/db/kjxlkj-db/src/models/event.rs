use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct NoteEventRow {
    pub id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub version: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct WorkspaceEventRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: time::OffsetDateTime,
}
