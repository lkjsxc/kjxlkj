//! Note-view analytics queries

use super::listing_cursor::row_to_listed_record;
use super::{DbPool, ListedRecord, NoteViewStats, PopularWindow};
use crate::error::AppError;

pub async fn record_note_view(pool: &DbPool, id: &str) -> Result<(), AppError> {
    let mut db = client(pool).await?;
    let tx = db
        .transaction()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tx.execute(
        "UPDATE records SET view_count_total = view_count_total + 1, last_viewed_at = NOW() \
         WHERE id = $1 AND deleted_at IS NULL",
        &[&id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tx.execute(
        "INSERT INTO record_daily_views (record_id, view_date, view_count) VALUES ($1, CURRENT_DATE, 1) \
         ON CONFLICT (record_id, view_date) DO UPDATE \
         SET view_count = record_daily_views.view_count + 1",
        &[&id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}

pub async fn get_note_view_stats(pool: &DbPool, id: &str) -> Result<NoteViewStats, AppError> {
    client(pool)
        .await?
        .query_one(
            "SELECT r.view_count_total AS total, r.last_viewed_at, \
             COALESCE(SUM(dv.view_count) FILTER (WHERE dv.view_date >= CURRENT_DATE - 6), 0)::BIGINT AS views_7d, \
             COALESCE(SUM(dv.view_count) FILTER (WHERE dv.view_date >= CURRENT_DATE - 29), 0)::BIGINT AS views_30d, \
             COALESCE(SUM(dv.view_count) FILTER (WHERE dv.view_date >= CURRENT_DATE - 89), 0)::BIGINT AS views_90d \
             FROM records r LEFT JOIN record_daily_views dv ON dv.record_id = r.id \
             WHERE r.id = $1 AND r.deleted_at IS NULL GROUP BY r.id, r.view_count_total, r.last_viewed_at",
            &[&id],
        )
        .await
        .map(|row| NoteViewStats {
            total: row.get("total"),
            views_7d: row.get("views_7d"),
            views_30d: row.get("views_30d"),
            views_90d: row.get("views_90d"),
            last_viewed_at: row.get("last_viewed_at"),
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn list_popular_records(
    pool: &DbPool,
    include_private: bool,
    limit: i64,
    window: PopularWindow,
) -> Result<Vec<ListedRecord>, AppError> {
    client(pool)
        .await?
        .query(
            "WITH popular AS (SELECT record_id, SUM(view_count)::BIGINT AS popular_views \
             FROM record_daily_views WHERE view_date >= CURRENT_DATE - ($3::INT - 1) GROUP BY record_id) \
             SELECT r.id, r.alias, r.title, r.summary, r.body, r.is_favorite, r.favorite_position, \
             r.is_private, r.view_count_total, r.last_viewed_at, r.created_at, r.updated_at, \
             r.summary AS preview, COALESCE(p.popular_views, 0)::BIGINT AS popular_views \
             FROM records r LEFT JOIN popular p ON p.record_id = r.id \
             WHERE r.deleted_at IS NULL AND ($1 OR r.is_private = FALSE) \
             ORDER BY COALESCE(p.popular_views, 0) DESC, r.view_count_total DESC, r.updated_at DESC, r.id ASC LIMIT $2",
            &[&include_private, &limit, &window.days()],
        )
        .await
        .map(|rows| rows.into_iter().map(row_to_listed_record).collect())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
