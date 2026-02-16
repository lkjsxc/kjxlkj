/// Event sourcing types per /docs/spec/domain/events.md
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Actor type for audit trail
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    User,
    Agent,
    System,
}

/// Note event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteEventType {
    Created,
    BodyUpdated,
    TitleUpdated,
    MetadataUpdated,
    SoftDeleted,
    Restored,
}

/// NoteEvent row (append-only) per /docs/spec/domain/events.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    pub id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: NoteEventType,
    pub actor_type: ActorType,
    pub actor_id: Uuid,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
}

/// Workspace event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceEventType {
    Created,
    MemberAdded,
    MemberRemoved,
    RoleChanged,
    Archived,
    Deleted,
}

/// WorkspaceEvent row (append-only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEvent {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub seq: i64,
    pub event_type: WorkspaceEventType,
    pub actor_type: ActorType,
    pub actor_id: Uuid,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
}
