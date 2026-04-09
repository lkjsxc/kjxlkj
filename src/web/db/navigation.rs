use super::record_support::row_to_record;
use super::DbPool;
use crate::error::AppError;
use crate::web::db::Record;

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
    let query = if older {
        "SELECT id, kind, alias, title, summary, body, media_family, file_key, content_type, \
         byte_size, sha256_hex, original_filename, width, height, duration_ms, is_favorite, \
         favorite_position, is_private, view_count_total, last_viewed_at, created_at, updated_at \
         FROM resources WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at < (SELECT created_at FROM resources WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM resources WHERE id = $1 AND deleted_at IS NULL) AND id < $1)) \
         ORDER BY created_at DESC, id DESC LIMIT 1"
    } else {
        "SELECT id, kind, alias, title, summary, body, media_family, file_key, content_type, \
         byte_size, sha256_hex, original_filename, width, height, duration_ms, is_favorite, \
         favorite_position, is_private, view_count_total, last_viewed_at, created_at, updated_at \
         FROM resources WHERE deleted_at IS NULL AND ($2 OR is_private = FALSE) \
         AND ((created_at > (SELECT created_at FROM resources WHERE id = $1 AND deleted_at IS NULL)) \
           OR (created_at = (SELECT created_at FROM resources WHERE id = $1 AND deleted_at IS NULL) AND id > $1)) \
         ORDER BY created_at ASC, id ASC LIMIT 1"
    };
    client(pool)
        .await?
        .query_opt(query, &[&id, &include_private])
        .await
        .map(|row| row.map(row_to_record))
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
