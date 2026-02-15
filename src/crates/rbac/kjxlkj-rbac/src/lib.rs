//! kjxlkj-rbac: Role and permission enforcement.
//! Per /docs/spec/domain/permissions.md.

use kjxlkj_domain::types::Role;

/// Parse role string to enum.
pub fn parse_role(s: &str) -> Option<Role> {
    match s {
        "owner" => Some(Role::Owner),
        "admin" => Some(Role::Admin),
        "editor" => Some(Role::Editor),
        "viewer" => Some(Role::Viewer),
        _ => None,
    }
}

/// Check if role can manage workspace (owner/admin).
pub fn can_manage_workspace(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can manage members (owner/admin).
pub fn can_manage_members(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can edit notes (owner/admin/editor).
pub fn can_edit(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if role can delete notes (owner/admin/editor).
pub fn can_delete(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin | Role::Editor)
}

/// Check if role can manage automation rules (owner/admin).
pub fn can_manage_automation(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

/// Check if role can admin users (owner only at global level).
pub fn can_admin_users(role: Role) -> bool {
    matches!(role, Role::Owner | Role::Admin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_parsing() {
        assert_eq!(parse_role("owner"), Some(Role::Owner));
        assert_eq!(parse_role("admin"), Some(Role::Admin));
        assert_eq!(parse_role("editor"), Some(Role::Editor));
        assert_eq!(parse_role("viewer"), Some(Role::Viewer));
        assert_eq!(parse_role("unknown"), None);
    }

    #[test]
    fn test_permissions() {
        assert!(can_manage_workspace(Role::Owner));
        assert!(can_manage_workspace(Role::Admin));
        assert!(!can_manage_workspace(Role::Editor));
        assert!(can_edit(Role::Editor));
        assert!(!can_edit(Role::Viewer));
    }
}
