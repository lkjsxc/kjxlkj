use kjxlkj_db::models::workspace::WorkspaceRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use kjxlkj_domain::types::WorkspaceRole;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_workspace(
    pool: &PgPool,
    name: &str,
    slug: &str,
    owner_id: Uuid,
) -> Result<WorkspaceRow, DomainError> {
    let ws_id = Uuid::new_v4();
    let ws = repos::workspaces::create(pool, ws_id, name, slug, owner_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    // Auto-add creator as workspace owner member.
    repos::workspaces::upsert_member(pool, ws.id, owner_id, WorkspaceRole::Owner)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    tracing::info!(workspace_id = %ws.id, "workspace created");
    Ok(ws)
}

pub async fn list_workspaces(pool: &PgPool) -> Result<Vec<WorkspaceRow>, DomainError> {
    repos::workspaces::list(pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn get_workspace(
    pool: &PgPool,
    id: Uuid,
) -> Result<WorkspaceRow, DomainError> {
    repos::workspaces::find_by_id(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "workspace".into(),
        })
}

pub async fn update_workspace(
    pool: &PgPool,
    id: Uuid,
    name: &str,
) -> Result<WorkspaceRow, DomainError> {
    repos::workspaces::update(pool, id, name)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "workspace".into(),
        })
}

pub async fn delete_workspace(pool: &PgPool, id: Uuid) -> Result<(), DomainError> {
    repos::workspaces::delete(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}
