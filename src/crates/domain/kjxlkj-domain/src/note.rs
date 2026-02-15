//! Note domain types per /docs/spec/domain/notes.md.

use crate::types::{AccessScope, NoteKind, Role, UserStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User entity per /docs/spec/api/types.md.
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

/// Session entity per /docs/spec/security/sessions.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub csrf_token: String,
    pub expires_at: String,
    pub created_at: String,
}

/// Workspace entity per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

/// Workspace member per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub joined_at: String,
}

/// Project per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: String,
    pub archived: bool,
    pub created_at: String,
}

/// Note stream per /docs/spec/api/types.md.
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

/// Note projection per /docs/spec/api/types.md.
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
    pub tags: Vec<String>,
}

/// Saved view per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedView {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub query_json: serde_json::Value,
    pub sort: String,
    pub filters: serde_json::Value,
    pub owner_user_id: Uuid,
    pub created_at: String,
}

/// Dashboard widget per /docs/spec/api/types.md (optional extension).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub widget_type: String,
    pub config_json: serde_json::Value,
    pub layout: Option<serde_json::Value>,
}

/// Attachment per /docs/spec/api/types.md.
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

/// Automation rule per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub trigger: String,
    pub condition_json: serde_json::Value,
    pub action_json: serde_json::Value,
    pub enabled: bool,
    pub created_at: String,
}

/// Automation run per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRun {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: crate::types::RunStatus,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub result_json: Option<serde_json::Value>,
}
