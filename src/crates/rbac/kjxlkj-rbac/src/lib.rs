// kjxlkj-rbac: role-based access control
// Spec: /docs/spec/domain/permissions.md

use kjxlkj_domain::types::Role;

/// Check if role can manage workspace settings.
pub fn can_manage_workspace(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can manage members.
pub fn can_manage_members(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can create/edit notes.
pub fn can_edit_notes(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if role can delete notes.
pub fn can_delete_notes(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if role can manage automation rules.
pub fn can_manage_automation(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can read content.
pub fn can_read(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor | Role::Viewer)
}

/// Check if role can manage projects.
pub fn can_manage_projects(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can manage views.
pub fn can_manage_views(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_cannot_edit() {
        assert!(!can_edit_notes(Role::Viewer));
        assert!(can_read(Role::Viewer));
    }

    #[test]
    fn test_editor_can_edit() {
        assert!(can_edit_notes(Role::Editor));
        assert!(!can_manage_workspace(Role::Editor));
    }

    #[test]
    fn test_owner_full_access() {
        assert!(can_manage_workspace(Role::Owner));
        assert!(can_manage_members(Role::Owner));
        assert!(can_edit_notes(Role::Owner));
        assert!(can_manage_automation(Role::Owner));
    }
}
