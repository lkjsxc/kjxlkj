use super::models::Record;
use super::record_support::{
    current_favorite_state, map_write_error, next_position, resolve_position, row_to_record,
    RETURNING_RECORD, SELECT_RECORD,
};
use super::resource_ids::next_resource_id;
use super::DbPool;
use crate::core::{derive_summary, derive_title};
use crate::error::AppError;

pub async fn get_record(pool: &DbPool, id: &str) -> Result<Option<Record>, AppError> {
    get_record_where(pool, "id = $1", &[&id]).await
}
pub async fn get_record_by_alias(pool: &DbPool, alias: &str) -> Result<Option<Record>, AppError> {
    get_record_where(pool, "alias = $1", &[&alias]).await
}
pub async fn get_record_by_ref(pool: &DbPool, reference: &str) -> Result<Option<Record>, AppError> {
    if let Some(record) = get_record_by_alias(pool, reference).await? {
        return Ok(Some(record));
    }
    get_record(pool, reference).await
}
pub async fn create_record(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Record, AppError> {
    let db = client(pool).await?;
    let row = db
        .query_one(
            &format!(
                "INSERT INTO records (id, alias, title, summary, body, is_favorite, favorite_position, is_private) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8) {RETURNING_RECORD}"
            ),
            &[
                &id,
                &alias,
                &derive_title(body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &next_position(&db, is_favorite).await?,
                &is_private,
            ],
    )
    .await
    .map_err(map_write_error)?;
    Ok(row_to_record(row))
}
pub async fn update_record(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    is_favorite: bool,
    is_private: bool,
) -> Result<Option<Record>, AppError> {
    let db = client(pool).await?;
    let Some((was_favorite, current_position)) = current_favorite_state(&db, id).await? else {
        return Ok(None);
    };
    let revision_id = next_resource_id(&db).await?;
    db.execute(
        "INSERT INTO record_revisions (id, record_id, body, is_private, revision_number) \
         SELECT $2, id, body, is_private, \
         COALESCE((SELECT MAX(revision_number) FROM record_revisions WHERE record_id = $1), 0) + 1 \
         FROM records WHERE id = $1 AND deleted_at IS NULL",
        &[&id, &revision_id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = db
        .query_one(
            &format!(
                "UPDATE records SET alias = $2, title = $3, summary = $4, body = $5, \
                 is_favorite = $6, favorite_position = $7, is_private = $8, updated_at = NOW() \
                 WHERE id = $1 AND deleted_at IS NULL {RETURNING_RECORD}"
            ),
            &[
                &id,
                &alias,
                &derive_title(body),
                &derive_summary(body),
                &body,
                &is_favorite,
                &resolve_position(&db, was_favorite, current_position, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    Ok(Some(row_to_record(row)))
}
pub async fn delete_record(pool: &DbPool, id: &str) -> Result<bool, AppError> {
    let count = client(pool)
        .await?
        .execute(
            "UPDATE records SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}
async fn get_record_where(
    pool: &DbPool,
    predicate: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Option<Record>, AppError> {
    client(pool)
        .await?
        .query_opt(
            &format!("{SELECT_RECORD} FROM records WHERE {predicate} AND deleted_at IS NULL"),
            params,
        )
        .await
        .map(|row| row.map(row_to_record))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
