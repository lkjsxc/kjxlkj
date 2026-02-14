// Domain types per /docs/spec/api/types.md
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role enum per /docs/spec/domain/permissions.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

/// User status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Disabled,
}

/// Note kind taxonomy per /docs/spec/domain/note-types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    Markdown,
    Settings,
    MediaImage,
    MediaVideo,
}

/// Access scope for notes per /docs/spec/api/types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    Workspace,
    Project,
    Private,
}

/// Librarian provider kind per /docs/spec/api/types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibrarianProviderKind {
    Openrouter,
    Lmstudio,
}

/// Automation run status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Librarian operation kind per /docs/spec/api/types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibrarianOperationKind {
    CreateNote,
    RewriteNote,
    RetitleNote,
    RelinkNote,
    RetagNote,
    Defer,
}

/// Core user model per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: Role,
    pub status: UserStatus,
    pub password_hash: String,
    pub created_at: String,
}

/// Workspace model per /docs/spec/domain/workspaces.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub created_at: String,
}

/// Workspace member per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub joined_at: String,
}

/// Project model per /docs/spec/domain/projects.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

/// Note stream per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteStream {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: NoteKind,
    pub access_scope: AccessScope,
    pub created_at: String,
    pub updated_at: String,
    pub current_version: i64,
    pub deleted_at: Option<String>,
}

/// Note projection per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteProjection {
    pub note_id: Uuid,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub title: String,
    pub note_kind: NoteKind,
    pub version: i64,
    pub markdown: String,
    pub metadata_json: serde_json::Value,
}

/// Note event per /docs/spec/domain/events.md
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

/// Automation rule per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
}

/// Automation run per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: RunStatus,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub result_json: Option<serde_json::Value>,
}

/// Attachment per /docs/spec/domain/attachments.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub note_id: Uuid,
    pub filename: String,
    pub mime: String,
    pub size_bytes: i64,
    pub sha256: String,
    pub chunk_count: i32,
}

/// Saved view per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedView {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub query_json: serde_json::Value,
    pub sort: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub owner_user_id: Uuid,
}

/// Session record per /docs/spec/security/sessions.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: String,
    pub created_at: String,
}
