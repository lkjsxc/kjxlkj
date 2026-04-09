//! Discovery queries

use super::{DbPool, SitemapRecord};
use crate::error::AppError;

pub async fn list_public_sitemap_records(pool: &DbPool) -> Result<Vec<SitemapRecord>, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .query(
            "SELECT id, alias, updated_at FROM resources \
             WHERE deleted_at IS NULL AND is_private = FALSE \
             ORDER BY updated_at DESC, id ASC",
            &[],
        )
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|row| SitemapRecord {
                    id: row.get("id"),
                    alias: row.get("alias"),
                    updated_at: row.get("updated_at"),
                })
                .collect()
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
