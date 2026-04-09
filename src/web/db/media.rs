use super::models::{MediaFamily, Resource, ResourceKind};
use super::resource_support::{map_write_error, next_position, row_to_resource, RETURNING_RECORD};
use super::write_support::create_snapshot;
use super::DbPool;
use crate::core::{derive_summary, derive_title_with_fallback};
use crate::error::AppError;

pub struct MediaBlob<'a> {
    pub media_family: MediaFamily,
    pub file_key: &'a str,
    pub content_type: &'a str,
    pub byte_size: i64,
    pub sha256_hex: &'a str,
    pub original_filename: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_ms: Option<i64>,
}

pub async fn create_media(
    pool: &DbPool,
    id: &str,
    alias: Option<&str>,
    body: &str,
    blob: &MediaBlob<'_>,
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
                "INSERT INTO resources (id, kind, alias, title, summary, body, media_family, file_key, content_type, \
                 byte_size, sha256_hex, original_filename, width, height, duration_ms, is_favorite, favorite_position, is_private) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18) {RETURNING_RECORD}"
            ),
            &[
                &id,
                &ResourceKind::Media.as_str(),
                &alias,
                &derive_title_with_fallback(body, "Untitled media"),
                &derive_summary(body),
                &body,
                &blob.media_family.as_str(),
                &blob.file_key,
                &blob.content_type,
                &blob.byte_size,
                &blob.sha256_hex,
                &blob.original_filename,
                &blob.width,
                &blob.height,
                &blob.duration_ms,
                &is_favorite,
                &next_position(&tx, is_favorite).await?,
                &is_private,
            ],
        )
        .await
        .map_err(map_write_error)?;
    let resource = row_to_resource(row);
    create_snapshot(&tx, &resource, 1).await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(resource)
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
