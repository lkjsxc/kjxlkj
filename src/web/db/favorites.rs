//! Favorite ordering queries

use super::listing_cursor::row_to_listed_record;
use super::{DbPool, ListedRecord};
use crate::error::AppError;
use std::collections::HashSet;

pub async fn list_all_favorite_records(
    pool: &DbPool,
    include_private: bool,
) -> Result<Vec<ListedRecord>, AppError> {
    let rows = client(pool)
        .await?
        .query(
            "SELECT id, alias, title, summary, body, is_favorite, favorite_position, is_private, \
             view_count_total, last_viewed_at, created_at, updated_at, summary AS preview, NULL::BIGINT AS popular_views \
             FROM records WHERE deleted_at IS NULL AND is_favorite = TRUE AND ($1 OR is_private = FALSE) \
             ORDER BY favorite_position ASC NULLS LAST, id ASC",
            &[&include_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(rows.into_iter().map(row_to_listed_record).collect())
}

pub async fn reorder_favorites(pool: &DbPool, ids: &[String]) -> Result<(), AppError> {
    let mut client = client(pool).await?;
    let transaction = client
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let current = transaction
        .query(
            "SELECT id FROM records WHERE deleted_at IS NULL AND is_favorite = TRUE \
             ORDER BY favorite_position ASC NULLS LAST, id ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .into_iter()
        .map(|row| row.get::<_, String>("id"))
        .collect::<Vec<_>>();
    validate_ids(&current, ids)?;
    transaction
        .execute(
            "UPDATE records SET favorite_position = NULL WHERE deleted_at IS NULL AND is_favorite = TRUE",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    for (index, id) in ids.iter().enumerate() {
        transaction
            .execute(
                "UPDATE records SET favorite_position = $2 WHERE id = $1",
                &[id, &((index + 1) as i64)],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    transaction
        .commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn validate_ids(current: &[String], requested: &[String]) -> Result<(), AppError> {
    let requested_set = requested.iter().collect::<HashSet<_>>();
    if requested.len() != requested_set.len() {
        return Err(AppError::InvalidRequest(
            "favorite order contains duplicate ids".to_string(),
        ));
    }
    if current.len() != requested.len() || current.iter().collect::<HashSet<_>>() != requested_set {
        return Err(AppError::InvalidRequest(
            "favorite order must include every current favorite exactly once".to_string(),
        ));
    }
    Ok(())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::validate_ids;

    #[test]
    fn validate_ids_rejects_duplicates_and_mismatches() {
        assert!(validate_ids(&["a".into(), "b".into()], &["a".into(), "a".into()]).is_err());
        assert!(validate_ids(&["a".into(), "b".into()], &["a".into()]).is_err());
    }

    #[test]
    fn validate_ids_accepts_full_reordered_set() {
        assert!(validate_ids(&["a".into(), "b".into()], &["b".into(), "a".into()]).is_ok());
    }
}
