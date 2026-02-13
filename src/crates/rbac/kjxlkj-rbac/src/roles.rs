use kjxlkj_domain::types::{GlobalRole, WorkspaceRole};

/// Minimum global role required for each action category.
pub fn can_manage_users(role: GlobalRole) -> bool {
    matches!(role, GlobalRole::Owner | GlobalRole::Admin)
}

pub fn can_create_workspace(role: GlobalRole) -> bool {
    matches!(
        role,
        GlobalRole::Owner | GlobalRole::Admin | GlobalRole::Editor
    )
}

/// Workspace-level permission checks per permissions.md.
pub fn can_manage_workspace(role: WorkspaceRole) -> bool {
    matches!(role, WorkspaceRole::Owner | WorkspaceRole::Admin)
}

pub fn can_manage_members(role: WorkspaceRole) -> bool {
    matches!(role, WorkspaceRole::Owner | WorkspaceRole::Admin)
}

pub fn can_edit_notes(role: WorkspaceRole) -> bool {
    matches!(
        role,
        WorkspaceRole::Owner | WorkspaceRole::Admin | WorkspaceRole::Editor
    )
}

pub fn can_view_workspace(_role: WorkspaceRole) -> bool {
    true // All roles can view
}

pub fn can_manage_automation(role: WorkspaceRole) -> bool {
    matches!(role, WorkspaceRole::Owner | WorkspaceRole::Admin)
}

pub fn can_manage_views(role: WorkspaceRole) -> bool {
    matches!(
        role,
        WorkspaceRole::Owner | WorkspaceRole::Admin | WorkspaceRole::Editor
    )
}

/// Check if a global role can assign the target global role.
pub fn can_assign_global_role(assigner: GlobalRole, target: GlobalRole) -> bool {
    match assigner {
        GlobalRole::Owner => true,
        GlobalRole::Admin => !matches!(target, GlobalRole::Owner),
        _ => false,
    }
}

/// Role rank for comparison (lower = more privileged).
pub fn role_rank(role: WorkspaceRole) -> u8 {
    match role {
        WorkspaceRole::Owner => 0,
        WorkspaceRole::Admin => 1,
        WorkspaceRole::Editor => 2,
        WorkspaceRole::Viewer => 3,
    }
}
