use super::models::{Resource, ResourceKind};
use super::resource_support::{
    current_favorite_state, map_write_error, next_position, resolve_position, row_to_resource,
    RETURNING_RECORD, SELECT_RECORD,
};
use super::write_support::{create_snapshot, next_snapshot_number};
use super::DbPool;
use crate::core::{derive_summary, derive_title, derive_title_with_fallback};
use crate::error::AppError;

pub async fn get_resource(pool: &DbPool, id: &str) -> Result<Option<Resource>, AppError> {
    get_resource_where(pool, "id = $1", &[&id]).await
}

pub async fn get_resource_by_alias(
    pool: &DbPool,
    alias: &str,
) -> Result<Option<Resource>, AppError> {
    get_resource_where(pool, "alias = $1", &[&alias]).await
}

pub async fn get_resource_by_ref(
    pool: &DbPool,
    reference: &str,
) -> Result<Option<Resource>, AppError> {
    if let Some(resource) = get_resource_by_alias(pool, reference).await? {
        return Ok(Some(resource));
    }
    get_resource(pool, reference).await
}

pub async fn create_resource(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Resource, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = tx
        .query_one(
            &format!(
                "INSERT INTO resources (id, kind, alias, title, summary, body, is_favorite, favorite_position, is_private) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) {RETURNING_RECORD}"
            ),
            &[
                &id,
                &ResourceKind::Note.as_str(),
                &alias,
                &derive_title(body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &next_position(&tx, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let resource = row_to_resource(row);
    create_snapshot(
        &tx,
        &resource,
        next_snapshot_number(&tx, &resource.id).await?,
    )
    .await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(resource)
}

pub async fn update_resource(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Option<Resource>, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let Some((kind, was_favorite, current_position)) = current_favorite_state(&tx, id).await?
    else {
        return Ok(None);
    };
    let row = tx
        .query_one(
            &format!(
                "UPDATE resources SET alias = $2, title = $3, summary = $4, body = $5, \
                 is_favorite = $6, favorite_position = $7, is_private = $8, updated_at = NOW() \
                 WHERE id = $1 AND deleted_at IS NULL {RETURNING_RECORD}"
            ),
            &[
                &id,
                &alias,
                &derive_title_for_kind(kind, body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &resolve_position(&tx, was_favorite, current_position, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let resource = row_to_resource(row);
    create_snapshot(
        &tx,
        &resource,
        next_snapshot_number(&tx, &resource.id).await?,
    )
    .await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(Some(resource))
}

fn derive_title_for_kind(kind: ResourceKind, body: &str) -> String {
    match kind {
        ResourceKind::Note => derive_title(body),
        ResourceKind::Media => derive_title_with_fallback(body, "Untitled media"),
    }
}

pub async fn delete_resource(pool: &DbPool, id: &str) -> Result<bool, AppError> {
    let count = client(pool)
        .await?
        .execute(
            "UPDATE resources SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}

async fn get_resource_where(
    pool: &DbPool,
    predicate: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Option<Resource>, AppError> {
    client(pool)
        .await?
        .query_opt(
            &format!("{SELECT_RECORD} FROM resources WHERE {predicate} AND deleted_at IS NULL"),
            params,
        )
        .await
        .map(|row| row.map(row_to_resource))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
