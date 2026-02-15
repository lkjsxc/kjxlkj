use kjxlkj_db::{repo_project, repo_workspace};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::ids::{ProjectId, UserId, WorkspaceId};
use kjxlkj_rbac::guard;
use sqlx::PgPool;
use uuid::Uuid;

/// Create workspace per /docs/spec/domain/workspaces.md.
pub async fn create_workspace(
    pool: &PgPool,
    slug: &str,
    name: &str,
    owner_user_id: UserId,
) -> Result<WorkspaceId, DomainError> {
    let id = WorkspaceId(Uuid::now_v7());
    repo_workspace::create_workspace(pool, id, slug, name, owner_user_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(id)
}

/// Create project per /docs/spec/domain/projects.md.
/// Project name MUST be unique within its workspace.
pub async fn create_project(
    pool: &PgPool,
    workspace_id: WorkspaceId,
    actor_id: UserId,
    name: &str,
    description: Option<&str>,
) -> Result<ProjectId, DomainError> {
    let role = guard::resolve_workspace_role(pool, workspace_id, actor_id).await?;
    guard::require_editor(role)?;

    let id = ProjectId(Uuid::now_v7());
    repo_project::create_project(pool, id, workspace_id, name, description)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(id)
}
