use crate::user::Role;

/// Permission check result per /docs/spec/domain/permissions.md.
///
/// Route-level authorization validates authenticated identity and role.
/// Domain-level authorization re-validates permission before mutation commit.
/// Denied operations return deterministic 403 with stable error code.
pub fn can_manage_members(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner | Role::Admin)
}

pub fn can_manage_workspace(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner | Role::Admin)
}

pub fn can_manage_automation(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner | Role::Admin)
}

pub fn can_edit_notes(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner | Role::Admin | Role::Editor)
}

pub fn can_read(actor_role: Role) -> bool {
    matches!(
        actor_role,
        Role::Owner | Role::Admin | Role::Editor | Role::Viewer
    )
}

pub fn can_manage_users(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner | Role::Admin)
}

pub fn can_transfer_ownership(actor_role: Role) -> bool {
    matches!(actor_role, Role::Owner)
}
