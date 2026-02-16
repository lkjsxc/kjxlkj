/// Workspace domain types per /docs/spec/domain/workspaces.md
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workspace lifecycle state per /docs/spec/domain/workspaces.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceState {
    Active,
    Archived,
    Deleted,
}

/// Workspace entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_user_id: Uuid,
    pub state: WorkspaceState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Input for creating a workspace
#[derive(Debug, Clone, Deserialize)]
pub struct CreateWorkspaceInput {
    pub slug: String,
    pub name: String,
}

/// Workspace membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: crate::permission::Role,
    pub created_at: NaiveDateTime,
}
