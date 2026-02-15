use kjxlkj_db::repo_workspace;
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{UserId, WorkspaceId};
use kjxlkj_domain::user::Role;
use sqlx::PgPool;

/// Parse string role to domain Role type.
pub fn parse_role(s: &str) -> Option<Role> {
    match s {
        "owner" => Some(Role::Owner),
        "admin" => Some(Role::Admin),
        "editor" => Some(Role::Editor),
        "viewer" => Some(Role::Viewer),
        _ => None,
    }
}

/// Resolve the workspace role for a given user.
/// Per /docs/spec/domain/permissions.md: authorization checks
/// MUST include workspace scope.
pub async fn resolve_workspace_role(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    user_id: UserId,
) -> Result<Role, DomainError> {
    let member = repo_workspace::find_member(pool, workspace_id, user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

    match member {
        Some(m) => parse_role(&m.role)
            .ok_or_else(|| DomainError::Internal("invalid role in db".into())),
        None => Err(DomainError::RoleForbidden(
            "not a member of this workspace".into(),
        )),
    }
}

/// Require at least edit-level permission.
pub fn require_editor(role: Role) -> Result<(), DomainError> {
    if kjxlkj_domain::permission::can_edit_notes(role) {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden("editor role required".into()))
    }
}

/// Require at least admin-level permission.
pub fn require_admin(role: Role) -> Result<(), DomainError> {
    if kjxlkj_domain::permission::can_manage_workspace(role) {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden("admin role required".into()))
    }
}

/// Require owner permission.
pub fn require_owner(role: Role) -> Result<(), DomainError> {
    if kjxlkj_domain::permission::can_transfer_ownership(role) {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden("owner role required".into()))
    }
}
