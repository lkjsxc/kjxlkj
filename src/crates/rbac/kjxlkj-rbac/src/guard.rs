use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::WorkspaceRole;
use sqlx::PgPool;
use uuid::Uuid;

/// Resolve the effective workspace role for a user.
/// Returns None if the user is not a member of the workspace.
pub async fn resolve_workspace_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Option<WorkspaceRole>, DomainError> {
    let role = repos::workspaces::get_member_role(pool, workspace_id, user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(role)
}

/// Require at minimum the given workspace role, returning DomainError::RoleForbidden on failure.
pub async fn require_workspace_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    check: fn(WorkspaceRole) -> bool,
) -> Result<WorkspaceRole, DomainError> {
    let role = resolve_workspace_role(pool, workspace_id, user_id)
        .await?
        .ok_or(DomainError::RoleForbidden)?;
    if !check(role) {
        return Err(DomainError::RoleForbidden);
    }
    Ok(role)
}
