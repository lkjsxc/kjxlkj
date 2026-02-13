use kjxlkj_domain::Role;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RbacError {
    #[error("forbidden")]
    Forbidden,
}

pub fn ensure_global_role_update(actor: Role) -> Result<(), RbacError> {
    if actor.can_manage_global_roles() {
        return Ok(());
    }
    Err(RbacError::Forbidden)
}

pub fn ensure_workspace_member_read(actor_workspace_role: Role) -> Result<(), RbacError> {
    if actor_workspace_role.can_view_workspace() {
        return Ok(());
    }
    Err(RbacError::Forbidden)
}

pub fn ensure_workspace_member_write(actor_workspace_role: Role) -> Result<(), RbacError> {
    if actor_workspace_role.can_manage_workspace_members() {
        return Ok(());
    }
    Err(RbacError::Forbidden)
}

pub fn ensure_note_write(actor_workspace_role: Role) -> Result<(), RbacError> {
    if actor_workspace_role.can_write_notes() {
        return Ok(());
    }
    Err(RbacError::Forbidden)
}

pub fn ensure_automation_manage(actor_workspace_role: Role) -> Result<(), RbacError> {
    if actor_workspace_role.can_manage_automation() {
        return Ok(());
    }
    Err(RbacError::Forbidden)
}
