//! Record database operations

use super::models::Record;
use super::DbPool;
use crate::core::{derive_summary, derive_title};
use crate::error::AppError;

pub async fn get_record(pool: &DbPool, id: &str) -> Result<Option<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "SELECT id, title, summary, body, is_private, created_at, updated_at FROM records \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_record))
}

pub async fn create_record(
    pool: &DbPool,
    id: &str,
    body: &str,
    is_private: bool,
) -> Result<Record, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let title = derive_title(body);
    let summary = derive_summary(body);
    let row = client
        .query_one(
            "INSERT INTO records (id, title, summary, body, is_private) VALUES ($1, $2, $3, $4, $5) \
             RETURNING id, title, summary, body, is_private, created_at, updated_at",
            &[&id, &title, &summary, &body, &is_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row_to_record(row))
}

pub async fn update_record(
    pool: &DbPool,
    id: &str,
    body: &str,
    is_private: bool,
) -> Result<Option<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    client
        .execute(
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
    let row = client
        .query_opt(
            "UPDATE records SET title = $2, summary = $3, body = $4, is_private = $5, updated_at = NOW() \
             WHERE id = $1 AND deleted_at IS NULL \
             RETURNING id, title, summary, body, is_private, created_at, updated_at",
            &[&id, &title, &summary, &body, &is_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_record))
}

pub async fn delete_record(pool: &DbPool, id: &str) -> Result<bool, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let count = client
        .execute(
            "UPDATE records SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}

pub(crate) fn row_to_record(row: tokio_postgres::Row) -> Record {
    Record {
        id: row.get("id"),
        title: row.get("title"),
        summary: row.get("summary"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
