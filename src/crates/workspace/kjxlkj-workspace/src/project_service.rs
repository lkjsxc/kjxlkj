use kjxlkj_db::models::note::ProjectRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_project(
    pool: &PgPool,
    workspace_id: Uuid,
    name: &str,
) -> Result<ProjectRow, DomainError> {
    let id = Uuid::new_v4();
    repos::projects::create(pool, id, workspace_id, name)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn list_projects(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<ProjectRow>, DomainError> {
    repos::projects::list(pool, workspace_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn update_project(
    pool: &PgPool,
    id: Uuid,
    name: &str,
) -> Result<ProjectRow, DomainError> {
    repos::projects::update(pool, id, name)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "project".into(),
        })
}

pub async fn delete_project(pool: &PgPool, id: Uuid) -> Result<(), DomainError> {
    repos::projects::delete(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}
