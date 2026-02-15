//! Canonical type definitions per /docs/spec/api/types.md.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Note kind enum per /docs/spec/domain/note-types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

/// Role enum per /docs/spec/domain/permissions.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Viewer,
    Editor,
    Admin,
    Owner,
}

/// Access scope per /docs/spec/api/types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    Workspace,
    Project,
    Private,
}

/// User status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Disabled,
}

/// Automation run status per /docs/spec/domain/automation.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

/// Job status for export/backup per /docs/spec/domain/export.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

/// Librarian provider kind per /docs/spec/api/types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Openrouter,
    Lmstudio,
}

/// Librarian operation kind per /docs/spec/api/types.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibrarianOpKind {
    CreateNote,
    RewriteNote,
    RetitleNote,
    RelinkNote,
    RetagNote,
    Defer,
}

/// Unique request ID for tracing.
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}
