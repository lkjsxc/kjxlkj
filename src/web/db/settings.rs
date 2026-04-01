//! App settings and dashboard stats queries

use super::{AppSettings, DbPool, NoteStats};
use crate::error::AppError;

pub async fn get_settings(pool: &DbPool) -> Result<AppSettings, AppError> {
    let row = client(pool)
        .await?
        .query_opt(
            "SELECT home_recent_limit, home_favorite_limit, home_popular_limit, \
             home_recent_visible, home_favorite_visible, home_popular_visible, \
             home_recent_position, home_favorite_position, home_popular_position, \
             home_intro_markdown, search_results_per_page, default_new_note_is_private \
             FROM app_settings WHERE id = 1",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map_or_else(AppSettings::default, row_to_settings))
}

pub async fn update_settings(pool: &DbPool, settings: &AppSettings) -> Result<(), AppError> {
    client(pool)
        .await?
        .execute(
            "INSERT INTO app_settings (id, home_recent_limit, home_favorite_limit, home_popular_limit, \
             home_recent_visible, home_favorite_visible, home_popular_visible, \
             home_recent_position, home_favorite_position, home_popular_position, \
             home_intro_markdown, search_results_per_page, default_new_note_is_private) \
             VALUES (1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) \
             ON CONFLICT (id) DO UPDATE SET home_recent_limit = EXCLUDED.home_recent_limit, \
             home_favorite_limit = EXCLUDED.home_favorite_limit, \
             home_popular_limit = EXCLUDED.home_popular_limit, \
             home_recent_visible = EXCLUDED.home_recent_visible, \
             home_favorite_visible = EXCLUDED.home_favorite_visible, \
             home_popular_visible = EXCLUDED.home_popular_visible, \
             home_recent_position = EXCLUDED.home_recent_position, \
             home_favorite_position = EXCLUDED.home_favorite_position, \
             home_popular_position = EXCLUDED.home_popular_position, \
             home_intro_markdown = EXCLUDED.home_intro_markdown, \
             search_results_per_page = EXCLUDED.search_results_per_page, \
             default_new_note_is_private = EXCLUDED.default_new_note_is_private, updated_at = NOW()",
            &[
                &settings.home_recent_limit,
                &settings.home_favorite_limit,
                &settings.home_popular_limit,
                &settings.home_recent_visible,
                &settings.home_favorite_visible,
                &settings.home_popular_visible,
                &settings.home_recent_position,
                &settings.home_favorite_position,
                &settings.home_popular_position,
                &settings.home_intro_markdown,
                &settings.search_results_per_page,
                &settings.default_new_note_is_private,
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}

pub async fn get_note_stats(pool: &DbPool, include_private: bool) -> Result<NoteStats, AppError> {
    client(pool)
        .await?
        .query_one(
            "WITH rollup AS (SELECT record_id, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 6), 0)::BIGINT AS view_count_7d, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 29), 0)::BIGINT AS view_count_30d, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 89), 0)::BIGINT AS view_count_90d \
             FROM record_daily_views GROUP BY record_id) \
             SELECT COUNT(*) AS total, COUNT(*) FILTER (WHERE is_private = FALSE) AS public_count, \
             COUNT(*) FILTER (WHERE is_private = TRUE) AS private_count, \
             COUNT(*) FILTER (WHERE is_favorite = TRUE) AS favorite_count, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('month', NOW())) AS updated_this_month, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('year', NOW())) AS updated_this_year, \
             COALESCE(SUM(view_count_total), 0)::BIGINT AS view_count_total, \
             COALESCE(SUM(rollup.view_count_7d), 0)::BIGINT AS view_count_7d, \
             COALESCE(SUM(rollup.view_count_30d), 0)::BIGINT AS view_count_30d, \
             COALESCE(SUM(rollup.view_count_90d), 0)::BIGINT AS view_count_90d \
             FROM records LEFT JOIN rollup ON rollup.record_id = records.id \
             WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE)",
            &[&include_private],
        )
        .await
        .map(|row| NoteStats {
            total: row.get("total"),
            public_count: row.get("public_count"),
            private_count: row.get("private_count"),
            favorite_count: row.get("favorite_count"),
            updated_this_month: row.get("updated_this_month"),
            updated_this_year: row.get("updated_this_year"),
            view_count_total: row.get("view_count_total"),
            view_count_7d: row.get("view_count_7d"),
            view_count_30d: row.get("view_count_30d"),
            view_count_90d: row.get("view_count_90d"),
        })
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn row_to_settings(row: tokio_postgres::Row) -> AppSettings {
    AppSettings {
        home_recent_limit: row.get("home_recent_limit"),
        home_favorite_limit: row.get("home_favorite_limit"),
        home_popular_limit: row.get("home_popular_limit"),
        home_recent_visible: row.get("home_recent_visible"),
        home_favorite_visible: row.get("home_favorite_visible"),
        home_popular_visible: row.get("home_popular_visible"),
        home_recent_position: row.get("home_recent_position"),
        home_favorite_position: row.get("home_favorite_position"),
        home_popular_position: row.get("home_popular_position"),
        home_intro_markdown: row.get("home_intro_markdown"),
        search_results_per_page: row.get("search_results_per_page"),
        default_new_note_is_private: row.get("default_new_note_is_private"),
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
