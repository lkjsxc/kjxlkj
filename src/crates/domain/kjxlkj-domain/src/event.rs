//! Event sourcing entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event sequence number
pub type EventSeq = u64;

/// Actor type for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Actor {
    User { user_id: Uuid },
    Agent { agent_run_id: Uuid },
}

/// Event types for note stream
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum NoteEvent {
    Created {
        title: String,
        markdown: String,
        workspace_id: Uuid,
        project_id: Option<Uuid>,
        note_kind: String,
    },
    Updated {
        patch_ops: Vec<PatchOp>,
        new_version: u64,
    },
    TitleChanged {
        old_title: String,
        new_title: String,
    },
    Deleted {
        deleted_at: DateTime<Utc>,
    },
    Undeleted {
        undeleted_at: DateTime<Utc>,
    },
}

/// Patch operation for incremental updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOp {
    Retain { count: usize },
    Insert { text: String },
    Delete { count: usize },
}

/// Domain event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: Uuid,
    pub note_id: Uuid,
    pub event: NoteEvent,
    pub event_seq: EventSeq,
    pub version: u64,
    pub actor: Actor,
    pub timestamp: DateTime<Utc>,
}

impl DomainEvent {
    pub fn new(note_id: Uuid, event: NoteEvent, event_seq: EventSeq, version: u64, actor: Actor) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            note_id,
            event,
            event_seq,
            version,
            actor,
            timestamp: Utc::now(),
        }
    }
}
