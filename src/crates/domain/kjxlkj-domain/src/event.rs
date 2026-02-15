//! Event types per /docs/spec/domain/events.md.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Note event per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: String,
}

/// Workspace event per /docs/spec/domain/events.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEvent {
    pub event_id: Uuid,
    pub workspace_id: Uuid,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: Uuid,
    pub created_at: String,
}
