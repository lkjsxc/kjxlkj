//! Workspace domain types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

use crate::types::WorkspaceRole;

/// Workspace entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new(name: String, slug: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Workspace membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMembership {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: WorkspaceRole,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl WorkspaceMembership {
    /// Create a new membership.
    pub fn new(workspace_id: Uuid, user_id: Uuid, role: WorkspaceRole) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            workspace_id,
            user_id,
            role,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if member has admin privileges.
    pub fn is_admin(&self) -> bool {
        self.role == WorkspaceRole::Admin
    }

    /// Check if member can edit.
    pub fn can_edit(&self) -> bool {
        matches!(self.role, WorkspaceRole::Admin | WorkspaceRole::Editor)
    }
}

/// Saved view for workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedView {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub view_type: ViewType,
    pub filters: serde_json::Value,
    pub sort: Option<String>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// View type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    List,
    Grid,
    Graph,
    Calendar,
}

impl Default for ViewType {
    fn default() -> Self {
        Self::List
    }
}
