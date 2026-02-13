//! Role-based access control for kjxlkj.
//!
//! This crate contains authorization logic for global and workspace roles.

use uuid::Uuid;
use thiserror::Error;

use kjxlkj_domain::{GlobalRole, WorkspaceRole};

/// Authorization errors.
#[derive(Debug, Error)]
pub enum AuthzError {
    #[error("access denied")]
    AccessDenied,
    #[error("resource not found")]
    NotFound,
}

/// Permission check result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Allowed,
    Denied,
}

/// Check if a global role can perform an action.
pub fn check_global_permission(role: GlobalRole, action: GlobalAction) -> Permission {
    match (role, action) {
        // Owners can do everything
        (GlobalRole::Owner, _) => Permission::Allowed,
        // Admins can manage users and workspaces
        (GlobalRole::Admin, GlobalAction::ManageUsers) => Permission::Allowed,
        (GlobalRole::Admin, GlobalAction::ManageWorkspaces) => Permission::Allowed,
        (GlobalRole::Admin, GlobalAction::ViewUsers) => Permission::Allowed,
        // Editors and viewers can view
        (GlobalRole::Editor, GlobalAction::ViewUsers) => Permission::Allowed,
        (GlobalRole::Viewer, GlobalAction::ViewUsers) => Permission::Allowed,
        _ => Permission::Denied,
    }
}

/// Check if a workspace role can perform an action.
pub fn check_workspace_permission(role: WorkspaceRole, action: WorkspaceAction) -> Permission {
    match (role, action) {
        // Admins can do everything in workspace
        (WorkspaceRole::Admin, _) => Permission::Allowed,
        // Editors can create/edit/delete notes
        (WorkspaceRole::Editor, WorkspaceAction::CreateNote) => Permission::Allowed,
        (WorkspaceRole::Editor, WorkspaceAction::EditNote) => Permission::Allowed,
        (WorkspaceRole::Editor, WorkspaceAction::DeleteNote) => Permission::Allowed,
        (WorkspaceRole::Editor, WorkspaceAction::ViewNote) => Permission::Allowed,
        (WorkspaceRole::Editor, WorkspaceAction::ManageViews) => Permission::Allowed,
        (WorkspaceRole::Editor, WorkspaceAction::ManageAutomation) => Permission::Allowed,
        // Viewers can only view
        (WorkspaceRole::Viewer, WorkspaceAction::ViewNote) => Permission::Allowed,
        _ => Permission::Denied,
    }
}

/// Global actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalAction {
    ManageUsers,
    ManageWorkspaces,
    ViewUsers,
}

/// Workspace actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceAction {
    CreateNote,
    EditNote,
    DeleteNote,
    ViewNote,
    ManageViews,
    ManageAutomation,
    ManageMembers,
}

/// Authorization context for a user.
#[derive(Debug, Clone)]
pub struct AuthzContext {
    pub user_id: Uuid,
    pub global_role: GlobalRole,
    pub workspace_role: Option<WorkspaceRole>,
    pub workspace_id: Option<Uuid>,
}

impl AuthzContext {
    /// Create a new authorization context.
    pub fn new(user_id: Uuid, global_role: GlobalRole) -> Self {
        Self {
            user_id,
            global_role,
            workspace_role: None,
            workspace_id: None,
        }
    }

    /// Set workspace context.
    pub fn with_workspace(mut self, workspace_id: Uuid, role: WorkspaceRole) -> Self {
        self.workspace_id = Some(workspace_id);
        self.workspace_role = Some(role);
        self
    }

    /// Check global permission.
    pub fn can(&self, action: GlobalAction) -> bool {
        check_global_permission(self.global_role, action) == Permission::Allowed
    }

    /// Check workspace permission.
    pub fn can_in_workspace(&self, action: WorkspaceAction) -> bool {
        if self.global_role == GlobalRole::Owner {
            return true;
        }
        self.workspace_role
            .map(|role| check_workspace_permission(role, action) == Permission::Allowed)
            .unwrap_or(false)
    }

    /// Check if user is owner.
    pub fn is_owner(&self) -> bool {
        self.global_role == GlobalRole::Owner
    }

    /// Check if user is admin or higher.
    pub fn is_admin(&self) -> bool {
        matches!(self.global_role, GlobalRole::Owner | GlobalRole::Admin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_can_do_everything() {
        let ctx = AuthzContext::new(Uuid::nil(), GlobalRole::Owner);
        assert!(ctx.can(GlobalAction::ManageUsers));
        assert!(ctx.can(GlobalAction::ManageWorkspaces));
    }

    #[test]
    fn test_viewer_cannot_manage() {
        let ctx = AuthzContext::new(Uuid::nil(), GlobalRole::Viewer);
        assert!(!ctx.can(GlobalAction::ManageUsers));
        assert!(ctx.can(GlobalAction::ViewUsers));
    }

    #[test]
    fn test_workspace_editor() {
        let ctx = AuthzContext::new(Uuid::nil(), GlobalRole::Editor)
            .with_workspace(Uuid::nil(), WorkspaceRole::Editor);
        assert!(ctx.can_in_workspace(WorkspaceAction::CreateNote));
        assert!(ctx.can_in_workspace(WorkspaceAction::EditNote));
        assert!(ctx.can_in_workspace(WorkspaceAction::DeleteNote));
    }

    #[test]
    fn test_workspace_viewer() {
        let ctx = AuthzContext::new(Uuid::nil(), GlobalRole::Viewer)
            .with_workspace(Uuid::nil(), WorkspaceRole::Viewer);
        assert!(!ctx.can_in_workspace(WorkspaceAction::CreateNote));
        assert!(!ctx.can_in_workspace(WorkspaceAction::EditNote));
        assert!(ctx.can_in_workspace(WorkspaceAction::ViewNote));
    }
}
