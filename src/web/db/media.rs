use super::models::{MediaFamily, Record, RecordKind};
use super::record_support::{map_write_error, next_position, row_to_record, RETURNING_RECORD};
use super::resource_ids::next_resource_id;
use super::DbPool;
use crate::core::{derive_summary, derive_title_with_fallback};
use crate::error::AppError;
use deadpool_postgres::GenericClient;

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
) -> Result<Record, AppError> {
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
                &RecordKind::Media.as_str(),
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
    let record = row_to_record(row);
    create_snapshot(&tx, &record, 1).await?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(record)
}

async fn create_snapshot<C: GenericClient>(
    db: &C,
    record: &Record,
    snapshot_number: i32,
) -> Result<(), AppError> {
    let snapshot_id = next_resource_id(db).await?;
    db.execute(
        "INSERT INTO resource_snapshots \
         (id, resource_id, kind, snapshot_number, alias, title, summary, body, media_family, file_key, \
          content_type, byte_size, sha256_hex, original_filename, width, height, duration_ms, is_private) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)",
        &[
            &snapshot_id,
            &record.id,
            &record.kind.as_str(),
            &snapshot_number,
            &record.alias,
            &record.title,
            &record.summary,
            &record.body,
            &record.media_family.map(|family| family.as_str()),
            &record.file_key,
            &record.content_type,
            &record.byte_size,
            &record.sha256_hex,
            &record.original_filename,
            &record.width,
            &record.height,
            &record.duration_ms,
            &record.is_private,
        ],
    )
    .await
    .map(|_| ())
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
