use super::models::Resource;
use super::resource_ids::next_resource_id;
use super::DbPool;
use crate::error::AppError;
use deadpool_postgres::GenericClient;

pub async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn next_snapshot_number<C: GenericClient>(
    db: &C,
    resource_id: &str,
) -> Result<i32, AppError> {
    db.query_one(
        "SELECT COALESCE(MAX(snapshot_number), 0) + 1 AS snapshot_number \
         FROM resource_snapshots WHERE resource_id = $1",
        &[&resource_id],
    )
    .await
    .map(|row| row.get("snapshot_number"))
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn create_snapshot<C: GenericClient>(
    db: &C,
    resource: &Resource,
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
            &resource.id,
            &resource.kind.as_str(),
            &snapshot_number,
            &resource.alias,
            &resource.title,
            &resource.summary,
            &resource.body,
            &resource.media_family.map(|family| family.as_str()),
            &resource.file_key,
            &resource.content_type,
            &resource.byte_size,
            &resource.sha256_hex,
            &resource.original_filename,
            &resource.width,
            &resource.height,
            &resource.duration_ms,
            &resource.is_private,
        ],
    )
    .await
    .map(|_| ())
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}
