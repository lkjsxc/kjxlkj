//! Record database operations

use super::DbPool;
use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub slug: String,
    pub body: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A revision of a record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRevision {
    pub revision_number: i32,
    pub body: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

/// List records with optional privacy filter
pub async fn list_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
) -> Result<Vec<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let q = if include_private {
        "SELECT slug, body, is_private, created_at, updated_at FROM records \
         WHERE deleted_at IS NULL ORDER BY updated_at DESC, slug ASC LIMIT $1"
    } else {
        "SELECT slug, body, is_private, created_at, updated_at FROM records \
         WHERE deleted_at IS NULL AND is_private = FALSE ORDER BY updated_at DESC, slug ASC LIMIT $1"
    };
    let rows = client
        .query(q, &[&limit])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(row_to_record).collect())
}

/// Get a single record by slug
pub async fn get_record(pool: &DbPool, slug: &str) -> Result<Option<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "SELECT slug, body, is_private, created_at, updated_at FROM records \
             WHERE slug = $1 AND deleted_at IS NULL",
            &[&slug],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_record))
}

/// Create a new record
pub async fn create_record(
    pool: &DbPool,
    slug: &str,
    body: &str,
    is_private: bool,
) -> Result<Record, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_one(
            "INSERT INTO records (slug, body, is_private) VALUES ($1, $2, $3) \
             RETURNING slug, body, is_private, created_at, updated_at",
            &[&slug, &body, &is_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row_to_record(row))
}

/// Update a record (creates revision first)
pub async fn update_record(
    pool: &DbPool,
    slug: &str,
    body: &str,
    is_private: bool,
) -> Result<Option<Record>, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    // Create revision from current state
    client
        .execute(
            "INSERT INTO record_revisions (record_slug, body, is_private, revision_number) \
             SELECT slug, body, is_private, \
             COALESCE((SELECT MAX(revision_number) FROM record_revisions WHERE record_slug = $1), 0) + 1 \
             FROM records WHERE slug = $1 AND deleted_at IS NULL",
            &[&slug],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "UPDATE records SET body = $2, is_private = $3, updated_at = NOW() \
             WHERE slug = $1 AND deleted_at IS NULL \
             RETURNING slug, body, is_private, created_at, updated_at",
            &[&slug, &body, &is_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map(row_to_record))
}

/// Soft delete a record
pub async fn delete_record(pool: &DbPool, slug: &str) -> Result<bool, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let count = client
        .execute(
            "UPDATE records SET deleted_at = NOW() WHERE slug = $1 AND deleted_at IS NULL",
            &[&slug],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(count > 0)
}

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
    Ok(rows
        .into_iter()
        .map(|row| RecordRevision {
            revision_number: row.get("revision_number"),
            body: row.get("body"),
            is_private: row.get("is_private"),
            created_at: row.get("created_at"),
        })
        .collect())
}

fn row_to_record(row: tokio_postgres::Row) -> Record {
    Record {
        slug: row.get("slug"),
        body: row.get("body"),
        is_private: row.get("is_private"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
