//! Revision and navigation queries

use super::models::RecordRevision;
use super::DbPool;
use crate::error::AppError;

/// Get revisions for a record
pub async fn get_record_revisions(
    pool: &DbPool,
    slug: &str,
) -> Result<Vec<RecordRevision>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let rows = client
        .query(
            "SELECT revision_number, body, is_private, created_at FROM record_revisions \
             WHERE record_slug = $1 ORDER BY revision_number DESC",
            &[&slug],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(row_to_revision).collect())
}

/// Get a single revision for a record
pub async fn get_record_revision(
    pool: &DbPool,
    slug: &str,
    revision_number: i32,
) -> Result<Option<RecordRevision>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "SELECT revision_number, body, is_private, created_at FROM record_revisions \
             WHERE record_slug = $1 AND revision_number = $2",
            &[&slug, &revision_number],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_revision))
}

/// Get the previous note slug by created order
pub async fn get_previous_slug(
    pool: &DbPool,
    slug: &str,
    include_private: bool,
) -> Result<Option<String>, AppError> {
    adjacent_slug(pool, slug, include_private, true).await
}

/// Get the next note slug by created order
pub async fn get_next_slug(
    pool: &DbPool,
    slug: &str,
    include_private: bool,
) -> Result<Option<String>, AppError> {
    adjacent_slug(pool, slug, include_private, false).await
}

async fn adjacent_slug(
    pool: &DbPool,
    slug: &str,
    include_private: bool,
    older: bool,
) -> Result<Option<String>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let query = if older {
        "SELECT slug FROM records \
         WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at < (SELECT created_at FROM records WHERE slug = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE slug = $1 AND deleted_at IS NULL) AND slug < $1)) \
         ORDER BY created_at DESC, slug DESC LIMIT 1"
    } else {
        "SELECT slug FROM records \
         WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at > (SELECT created_at FROM records WHERE slug = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM records WHERE slug = $1 AND deleted_at IS NULL) AND slug > $1)) \
         ORDER BY created_at ASC, slug ASC LIMIT 1"
    };
    let row = client
        .query_opt(query, &[&slug, &include_private])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(|row| row.get("slug")))
}

fn row_to_revision(row: tokio_postgres::Row) -> RecordRevision {
    RecordRevision {
        revision_number: row.get("revision_number"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
    }
}
