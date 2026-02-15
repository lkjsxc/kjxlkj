//! Workspace domain logic per /docs/spec/domain/workspaces.md.

use crate::types::Role;

/// Check if a role can perform workspace admin operations.
pub fn can_manage_workspace(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if a role can manage members.
pub fn can_manage_members(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if a role can create/edit notes.
pub fn can_edit_notes(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if a role can delete notes.
pub fn can_delete_notes(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if a role can view notes.
pub fn can_view_notes(_role: Role) -> bool {
    true // All roles can view
}

/// Check if a role can manage automation rules.
pub fn can_manage_automation(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        assert!(can_manage_workspace(Role::Owner));
        assert!(can_manage_workspace(Role::Admin));
        assert!(!can_manage_workspace(Role::Editor));
        assert!(!can_manage_workspace(Role::Viewer));

        assert!(can_edit_notes(Role::Editor));
        assert!(!can_edit_notes(Role::Viewer));

        assert!(can_view_notes(Role::Viewer));
    }
}
