use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::{EventId, NoteId, ProjectId, UserId, WorkspaceId};

/// Note kind per /docs/spec/api/types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

/// Access scope per /docs/spec/api/types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessScope {
    Workspace,
    Project,
    Private,
}

/// Note stream per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteStream {
    pub id: NoteId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<ProjectId>,
    pub title: String,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub current_version: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub deleted_at: Option<OffsetDateTime>,
}

/// Note projection per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjection {
    pub note_id: NoteId,
    pub workspace_id: WorkspaceId,
    pub project_id: Option<ProjectId>,
    pub title: String,
    pub note_kind: NoteKind,
    pub version: i64,
    pub markdown: String,
    pub rendered_html: String,
    pub metadata_json: serde_json::Value,
}

/// Note event per /docs/spec/domain/events.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    pub event_id: EventId,
    pub note_id: NoteId,
    pub seq: i64,
    pub event_type: String,
    pub payload_json: serde_json::Value,
    pub actor_id: UserId,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Patch operation for note content per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchOp {
    Retain(usize),
    Insert(String),
    Delete(usize),
}
