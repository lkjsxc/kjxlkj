use kjxlkj_db::models::view::ViewRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_view(
    pool: &PgPool,
    workspace_id: Uuid,
    name: &str,
    filter_json: serde_json::Value,
    sort_json: serde_json::Value,
    created_by: Uuid,
) -> Result<ViewRow, DomainError> {
    let id = Uuid::new_v4();
    repos::views::create(pool, id, workspace_id, name, &filter_json, &sort_json, created_by)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn list_views(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<ViewRow>, DomainError> {
    repos::views::list(pool, workspace_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

pub async fn update_view(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    filter_json: &serde_json::Value,
    sort_json: &serde_json::Value,
) -> Result<ViewRow, DomainError> {
    repos::views::update(pool, id, name, filter_json, sort_json)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or(DomainError::NotFound {
            entity: "view".into(),
        })
}

pub async fn delete_view(pool: &PgPool, id: Uuid) -> Result<(), DomainError> {
    repos::views::delete(pool, id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
    Ok(())
}
