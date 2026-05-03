//! Discovery queries

use super::{DbPool, SitemapResource};
use crate::error::AppError;

pub async fn list_public_sitemap_resources(
    pool: &DbPool,
) -> Result<Vec<SitemapResource>, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .query(
            "SELECT id, alias, updated_at FROM resources \
             WHERE deleted_at IS NULL AND visibility = 'public' \
             ORDER BY updated_at DESC, id ASC",
            &[],
        )
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|row| SitemapResource {
                    id: row.get("id"),
                    alias: row.get("alias"),
                    updated_at: row.get("updated_at"),
                })
                .collect()
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
