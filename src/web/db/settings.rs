//! App settings and dashboard stats queries

use super::{AppSettings, DbPool, NoteStats};
use crate::error::AppError;

pub async fn get_settings(pool: &DbPool) -> Result<AppSettings, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_opt(
            "SELECT home_recent_limit, home_favorite_limit, search_results_per_page \
             FROM app_settings WHERE id = 1",
            &[],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(row.map_or_else(AppSettings::default, row_to_settings))
}

pub async fn update_settings(pool: &DbPool, settings: &AppSettings) -> Result<(), AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    client
        .execute(
            "INSERT INTO app_settings (id, home_recent_limit, home_favorite_limit, search_results_per_page) \
             VALUES (1, $1, $2, $3) \
             ON CONFLICT (id) DO UPDATE SET \
             home_recent_limit = EXCLUDED.home_recent_limit, \
             home_favorite_limit = EXCLUDED.home_favorite_limit, \
             search_results_per_page = EXCLUDED.search_results_per_page, \
             updated_at = NOW()",
            &[
                &settings.home_recent_limit,
                &settings.home_favorite_limit,
                &settings.search_results_per_page,
            ],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}

pub async fn get_note_stats(pool: &DbPool, include_private: bool) -> Result<NoteStats, AppError> {
    let client = pool
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let row = client
        .query_one(
            "SELECT \
             COUNT(*) AS total, \
             COUNT(*) FILTER (WHERE is_private = FALSE) AS public_count, \
             COUNT(*) FILTER (WHERE is_private = TRUE) AS private_count, \
             COUNT(*) FILTER (WHERE is_favorite = TRUE) AS favorite_count, \
             COUNT(*) FILTER (WHERE created_at >= date_trunc('month', NOW())) AS created_this_month, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('month', NOW())) AS updated_this_month, \
             COUNT(*) FILTER (WHERE created_at >= date_trunc('year', NOW())) AS created_this_year, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('year', NOW())) AS updated_this_year \
             FROM records WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE)",
            &[&include_private],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(NoteStats {
        total: row.get("total"),
        public_count: row.get("public_count"),
        private_count: row.get("private_count"),
        favorite_count: row.get("favorite_count"),
        created_this_month: row.get("created_this_month"),
        updated_this_month: row.get("updated_this_month"),
        created_this_year: row.get("created_this_year"),
        updated_this_year: row.get("updated_this_year"),
    })
}

fn row_to_settings(row: tokio_postgres::Row) -> AppSettings {
    AppSettings {
        home_recent_limit: row.get("home_recent_limit"),
        home_favorite_limit: row.get("home_favorite_limit"),
        search_results_per_page: row.get("search_results_per_page"),
    }
}
