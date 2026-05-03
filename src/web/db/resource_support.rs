use crate::error::AppError;
use crate::media::media_variants_from_json;
use crate::web::db::models::{MediaFamily, Resource, ResourceKind};
use deadpool_postgres::GenericClient;
use tokio_postgres::error::SqlState;

pub(super) const RETURNING_RECORD: &str = "RETURNING id, kind, alias, title, summary, body, \
media_family, file_key, content_type, byte_size, sha256_hex, original_filename, width, height, \
duration_ms, media_variants, owner_note_id, is_favorite, favorite_position, \
(visibility = 'private') AS is_private, view_count_total, last_viewed_at, \
created_at, updated_at";
pub(super) const SELECT_RECORD: &str = "SELECT id, kind, alias, title, summary, body, \
media_family, file_key, content_type, byte_size, sha256_hex, original_filename, width, height, \
duration_ms, media_variants, owner_note_id, is_favorite, favorite_position, \
(visibility = 'private') AS is_private, view_count_total, last_viewed_at, \
created_at, updated_at";

pub(super) async fn current_favorite_state<C: GenericClient>(
    db: &C,
    id: &str,
) -> Result<Option<(ResourceKind, bool, Option<i64>)>, AppError> {
    db.query_opt(
        "SELECT kind, is_favorite, favorite_position FROM resources WHERE id = $1 AND deleted_at IS NULL",
        &[&id],
    )
    .await
    .map(|row| {
        row.map(|item| {
            (
                ResourceKind::from_db(&item.get::<_, String>("kind")),
                item.get("is_favorite"),
                item.get("favorite_position"),
            )
        })
    })
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub(super) async fn next_position<C: GenericClient>(
    db: &C,
    is_favorite: bool,
) -> Result<Option<i64>, AppError> {
    if !is_favorite {
        return Ok(None);
    }
    db.query_one(
        "SELECT COALESCE(MAX(favorite_position), 0) + 1 AS position \
         FROM resources WHERE deleted_at IS NULL AND is_favorite = TRUE",
        &[],
    )
    .await
    .map(|row| Some(row.get("position")))
    .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub(super) async fn resolve_position<C: GenericClient>(
    db: &C,
    was_favorite: bool,
    current_position: Option<i64>,
    is_favorite: bool,
) -> Result<Option<i64>, AppError> {
    match (was_favorite, is_favorite) {
        (_, false) => Ok(None),
        (true, true) => match current_position {
            Some(position) => Ok(Some(position)),
            None => next_position(db, true).await,
        },
        (false, true) => next_position(db, true).await,
    }
}

pub(super) fn map_write_error(error: tokio_postgres::Error) -> AppError {
    if error.code() == Some(&SqlState::UNIQUE_VIOLATION) {
        return AppError::InvalidRequest("alias already exists".to_string());
    }
    AppError::DatabaseError(error.to_string())
}

pub(crate) fn row_to_resource(row: tokio_postgres::Row) -> Resource {
    Resource {
        id: row.get("id"),
        kind: ResourceKind::from_db(&row.get::<_, String>("kind")),
        alias: row.get("alias"),
        title: row.get("title"),
        summary: row.get("summary"),
        body: row.get("body"),
        media_family: MediaFamily::from_db(row.get("media_family")),
        file_key: row.get("file_key"),
        content_type: row.get("content_type"),
        byte_size: row.get("byte_size"),
        sha256_hex: row.get("sha256_hex"),
        original_filename: row.get("original_filename"),
        width: row.get("width"),
        height: row.get("height"),
        duration_ms: row.get("duration_ms"),
        media_variants: media_variants_from_json(row.get("media_variants")),
        owner_note_id: row.get("owner_note_id"),
        is_favorite: row.get("is_favorite"),
        favorite_position: row.get("favorite_position"),
        is_private: row.get("is_private"),
        view_count_total: row.get("view_count_total"),
        last_viewed_at: row.get("last_viewed_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
