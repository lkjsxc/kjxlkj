//! Note domain types and invariants.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::types::*;

/// Note entity representing a note stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    pub state: NoteState,
    pub version: Version,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Note {
    /// Create a new note with default values.
    pub fn new(workspace_id: Uuid, title: String, body: String, note_kind: NoteKind) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            workspace_id,
            project_id: None,
            title,
            body,
            note_kind,
            access_scope: AccessScope::default(),
            state: NoteState::Active,
            version: Version::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Apply a body update with optimistic version check.
    pub fn update_body(&mut self, new_body: String, base_version: Version) -> Result<(), NoteError> {
        if self.version != base_version {
            return Err(NoteError::VersionConflict {
                expected: self.version,
                provided: base_version,
            });
        }
        self.body = new_body;
        self.version = self.version.increment();
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Apply a title update with optimistic version check.
    pub fn update_title(&mut self, new_title: String, base_version: Version) -> Result<(), NoteError> {
        if self.version != base_version {
            return Err(NoteError::VersionConflict {
                expected: self.version,
                provided: base_version,
            });
        }
        self.title = new_title;
        self.version = self.version.increment();
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Soft delete the note.
    pub fn soft_delete(&mut self) {
        self.state = NoteState::SoftDeleted;
        self.updated_at = OffsetDateTime::now_utc();
    }

    /// Check if note is deleted.
    pub fn is_deleted(&self) -> bool {
        self.state == NoteState::SoftDeleted
    }
}

/// Note history event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteHistoryEvent {
    pub id: Uuid,
    pub note_id: Uuid,
    pub event_type: NoteEventType,
    pub title: Option<String>,
    pub body: Option<String>,
    pub version: Version,
    pub actor_id: Uuid,
    pub created_at: OffsetDateTime,
}

/// Note event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteEventType {
    Created,
    BodyUpdated,
    TitleUpdated,
    Deleted,
    Restored,
}

/// Note metadata entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub note_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Note tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteTag {
    pub note_id: Uuid,
    pub tag: String,
    pub created_at: OffsetDateTime,
}

/// Backlink reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlink {
    pub source_note_id: Uuid,
    pub target_note_id: Uuid,
    pub link_text: String,
    pub created_at: OffsetDateTime,
}

/// Note domain errors.
#[derive(Debug, thiserror::Error)]
pub enum NoteError {
    #[error("version conflict: expected {expected:?}, got {provided:?}")]
    VersionConflict {
        expected: Version,
        provided: Version,
    },
    #[error("note not found: {0}")]
    NotFound(Uuid),
    #[error("access denied")]
    AccessDenied,
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
}

/// Note patch for WebSocket operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotePatch {
    pub note_id: Uuid,
    pub base_version: Version,
    pub patch_id: Uuid,
    pub operations: Vec<PatchOperation>,
}

/// Patch operation types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PatchOperation {
    BodyReplace { content: String },
    TitleReplace { content: String },
}
