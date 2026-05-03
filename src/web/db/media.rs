use super::models::{MediaFamily, Resource, ResourceKind};
use super::resource_support::{map_write_error, next_position, row_to_resource, RETURNING_RECORD};
use super::write_support::create_snapshot;
use super::DbPool;
use crate::core::{derive_summary, derive_title_with_fallback};
use crate::error::AppError;
use crate::media::{media_variants_to_json, MediaVariants};

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
    pub media_variants: Option<MediaVariants>,
}

pub struct ScopedMediaCreate<'a> {
    pub space_slug: &'a str,
    pub id: &'a str,
    pub alias: Option<&'a str>,
    pub body: &'a str,
    pub blob: &'a MediaBlob<'a>,
    pub is_favorite: bool,
    pub is_private: bool,
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
    let media_variants = media_variants_to_json(&blob.media_variants);
    let row = tx
        .query_one(
            &format!(
                "INSERT INTO resources (id, space_id, kind, alias, title, summary, body, media_family, file_key, content_type, \
                 byte_size, sha256_hex, original_filename, width, height, duration_ms, media_variants, owner_note_id, is_favorite, favorite_position, visibility) \
                 VALUES ($1, default_space_id(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NULL, $17, $18, \
                 CASE WHEN $19 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END) {RETURNING_RECORD}"
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
                &media_variants,
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

pub async fn create_media_in_space(
    pool: &DbPool,
    input: ScopedMediaCreate<'_>,
) -> Result<Resource, AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let media_variants = media_variants_to_json(&input.blob.media_variants);
    let row = tx
        .query_one(
            &format!(
                "INSERT INTO resources (id, space_id, kind, alias, title, summary, body, media_family, file_key, content_type, \
                 byte_size, sha256_hex, original_filename, width, height, duration_ms, media_variants, owner_note_id, is_favorite, favorite_position, visibility) \
                 SELECT $1, id, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, NULL, $18, $19, \
                 CASE WHEN $20 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END \
                 FROM spaces WHERE slug = $2::CITEXT {RETURNING_RECORD}"
            ),
            &[
                &input.id, &input.space_slug, &ResourceKind::Media.as_str(), &input.alias,
                &derive_title_with_fallback(input.body, "Untitled media"),
                &derive_summary(input.body), &input.body, &input.blob.media_family.as_str(),
                &input.blob.file_key, &input.blob.content_type, &input.blob.byte_size,
                &input.blob.sha256_hex, &input.blob.original_filename, &input.blob.width,
                &input.blob.height, &input.blob.duration_ms, &media_variants,
                &input.is_favorite, &next_position(&tx, input.is_favorite).await?,
                &input.is_private,
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
