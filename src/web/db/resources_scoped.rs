use super::models::{Resource, ResourceKind};
use super::resource_support::{
    current_favorite_state, map_write_error, next_position, resolve_position, row_to_resource,
    RETURNING_RECORD, SELECT_RECORD,
};
use super::write_support::{create_snapshot, next_snapshot_number};
use super::DbPool;
use crate::core::{derive_summary, derive_title, derive_title_with_fallback};
use crate::error::AppError;

pub async fn get_resource_by_ref_in_space(
    pool: &DbPool,
    space_slug: &str,
    reference: &str,
) -> Result<Option<Resource>, AppError> {
    if let Some(resource) = get_resource_where(
        pool,
        "space_id = (SELECT id FROM spaces WHERE slug = $1::CITEXT) AND alias = $2",
        &[&space_slug, &reference],
    )
    .await?
    {
        return Ok(Some(resource));
    }
    get_resource_where(
        pool,
        "space_id = (SELECT id FROM spaces WHERE slug = $1::CITEXT) AND id = $2",
        &[&space_slug, &reference],
    )
    .await
}

pub async fn create_resource_in_space(
    pool: &DbPool,
    space_slug: &str,
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
                "INSERT INTO resources (id, space_id, kind, alias, title, summary, body, is_favorite, favorite_position, visibility) \
                 SELECT $1, id, $3, $4, $5, $6, $7, $8, $9, \
                 CASE WHEN $10 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END \
                 FROM spaces WHERE slug = $2::CITEXT {RETURNING_RECORD}"
            ),
            &[
                &id, &space_slug, &ResourceKind::Note.as_str(), &alias,
                &derive_title(body), &derive_summary(body), &body, &is_favorite,
                &next_position(&tx, is_favorite).await?, &is_private,
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

pub async fn update_resource_in_space(
    pool: &DbPool,
    space_slug: &str,
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
                "UPDATE resources SET alias = $3, title = $4, summary = $5, body = $6, \
                 is_favorite = $7, favorite_position = $8, \
                 visibility = CASE WHEN $9 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END, \
                 updated_at = NOW() \
                 WHERE id = $1 AND space_id = (SELECT id FROM spaces WHERE slug = $2::CITEXT) \
                 AND deleted_at IS NULL {RETURNING_RECORD}"
            ),
            &[
                &id, &space_slug, &alias, &derive_title_for_kind(kind, body),
                &derive_summary(body), &body, &is_favorite,
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

pub async fn delete_resource_in_space(
    pool: &DbPool,
    space_slug: &str,
    id: &str,
) -> Result<bool, AppError> {
    let count = client(pool)
        .await?
        .execute(
            "UPDATE resources SET deleted_at = NOW() WHERE id = $1 \
             AND space_id = (SELECT id FROM spaces WHERE slug = $2::CITEXT) \
             AND deleted_at IS NULL",
            &[&id, &space_slug],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}

fn derive_title_for_kind(kind: ResourceKind, body: &str) -> String {
    match kind {
        ResourceKind::Note => derive_title(body),
        ResourceKind::Media => derive_title_with_fallback(body, "Untitled media"),
    }
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
