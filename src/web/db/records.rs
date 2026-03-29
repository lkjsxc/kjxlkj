//! Record database operations

use super::models::Record;
use super::DbPool;
use crate::core::{derive_summary, derive_title};
use crate::error::AppError;
use tokio_postgres::error::SqlState;

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
    let title = derive_title(body);
    let summary = derive_summary(body);
    let row = client(pool)
        .await?
        .query_one(
            "INSERT INTO records (id, alias, title, summary, body, is_favorite, is_private) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at",
            &[&id, &alias, &title, &summary, &body, &is_favorite, &is_private],
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
    db.execute(
        "INSERT INTO record_revisions (record_id, body, is_private, revision_number) \
         SELECT id, body, is_private, \
         COALESCE((SELECT MAX(revision_number) FROM record_revisions WHERE record_id = $1), 0) + 1 \
         FROM records WHERE id = $1 AND deleted_at IS NULL",
        &[&id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let title = derive_title(body);
    let summary = derive_summary(body);
    let row = db
        .query_opt(
            "UPDATE records SET alias = $2, title = $3, summary = $4, body = $5, \
             is_favorite = $6, is_private = $7, updated_at = NOW() \
             WHERE id = $1 AND deleted_at IS NULL \
             RETURNING id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at",
            &[&id, &alias, &title, &summary, &body, &is_favorite, &is_private],
        )
        .await
        .map_err(map_write_error)?;
    Ok(row.map(row_to_record))
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
    let row = client(pool)
        .await?
        .query_opt(
            &format!(
                "SELECT id, alias, title, summary, body, is_favorite, is_private, created_at, updated_at \
                 FROM records WHERE {predicate} AND deleted_at IS NULL"
            ),
            params,
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_record))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn map_write_error(error: tokio_postgres::Error) -> AppError {
    if error.code() == Some(&SqlState::UNIQUE_VIOLATION) {
        return AppError::InvalidRequest("alias already exists".to_string());
    }
    AppError::DatabaseError(error.to_string())
}

pub(crate) fn row_to_record(row: tokio_postgres::Row) -> Record {
    Record {
        id: row.get("id"),
        alias: row.get("alias"),
        title: row.get("title"),
        summary: row.get("summary"),
        body: row.get("body"),
        is_favorite: row.get("is_favorite"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
