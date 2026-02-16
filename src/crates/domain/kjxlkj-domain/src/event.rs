use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Actor type for event attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    User,
    Agent,
    System,
}

/// Core note event per docs/spec/domain/events.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    pub id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: Uuid,
    pub actor_type: ActorType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Core workspace event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEvent {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: Uuid,
    pub actor_type: ActorType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
