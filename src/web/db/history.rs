//! Revision and navigation queries

use super::models::{Record, RecordRevision};
use super::DbPool;
use crate::error::AppError;

pub async fn get_record_revisions(
    pool: &DbPool,
    record_id: &str,
) -> Result<Vec<RecordRevision>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let rows = client
        .query(
            "SELECT revision_number, body, is_private, created_at FROM record_revisions \
             WHERE record_id = $1 ORDER BY revision_number DESC",
            &[&record_id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(row_to_revision).collect())
}

pub async fn get_record_revision(
    pool: &DbPool,
    record_id: &str,
    revision_number: i32,
) -> Result<Option<RecordRevision>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "SELECT revision_number, body, is_private, created_at FROM record_revisions \
             WHERE record_id = $1 AND revision_number = $2",
            &[&record_id, &revision_number],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_revision))
}

pub async fn get_previous_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
) -> Result<Option<Record>, AppError> {
    adjacent_record(pool, id, include_private, true).await
}

pub async fn get_next_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
) -> Result<Option<Record>, AppError> {
    adjacent_record(pool, id, include_private, false).await
}

async fn adjacent_record(
    pool: &DbPool,
    id: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let query = if older {
        "SELECT id, title, summary, body, is_private, created_at, updated_at \
         FROM records WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at < (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL) AND id < $1)) \
         ORDER BY created_at DESC, id DESC LIMIT 1"
    } else {
        "SELECT id, title, summary, body, is_private, created_at, updated_at \
         FROM records WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at > (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE id = $1 AND deleted_at IS NULL) AND id > $1)) \
         ORDER BY created_at ASC, id ASC LIMIT 1"
    };
    let row = client
        .query_opt(query, &[&id, &include_private])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(super::records::row_to_record))
}

fn row_to_revision(row: tokio_postgres::Row) -> RecordRevision {
    RecordRevision {
        revision_number: row.get("revision_number"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}
