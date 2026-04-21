//! App settings and dashboard stats queries

use super::{AppSettings, DbPool, ResourceStats};
use crate::error::AppError;

pub async fn get_settings(pool: &DbPool) -> Result<AppSettings, AppError> {
    let row = client(pool)
        .await?
        .query_opt(
            "SELECT home_recent_limit, home_favorite_limit, home_popular_limit, home_intro_markdown, \
             home_recent_visible, home_favorite_visible, home_popular_visible, home_recent_position, \
             home_favorite_position, home_popular_position, search_results_per_page, session_timeout_minutes, \
             default_new_resource_is_private, media_webp_quality, site_name, site_description, public_base_url, \
             nostr_names, nostr_relays, live_ice_servers, site_icon_key, site_icon_content_type \
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
            "INSERT INTO app_settings (id, home_recent_limit, home_favorite_limit, home_popular_limit, home_intro_markdown, \
             home_recent_visible, home_favorite_visible, home_popular_visible, home_recent_position, home_favorite_position, \
             home_popular_position, search_results_per_page, session_timeout_minutes, default_new_resource_is_private, media_webp_quality, site_name, \
             site_description, public_base_url, nostr_names, nostr_relays, live_ice_servers, site_icon_key, site_icon_content_type) \
             VALUES (1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22) \
             ON CONFLICT (id) DO UPDATE SET home_recent_limit = EXCLUDED.home_recent_limit, home_favorite_limit = EXCLUDED.home_favorite_limit, \
             home_popular_limit = EXCLUDED.home_popular_limit, home_intro_markdown = EXCLUDED.home_intro_markdown, \
             home_recent_visible = EXCLUDED.home_recent_visible, home_favorite_visible = EXCLUDED.home_favorite_visible, \
             home_popular_visible = EXCLUDED.home_popular_visible, home_recent_position = EXCLUDED.home_recent_position, \
             home_favorite_position = EXCLUDED.home_favorite_position, home_popular_position = EXCLUDED.home_popular_position, \
             search_results_per_page = EXCLUDED.search_results_per_page, session_timeout_minutes = EXCLUDED.session_timeout_minutes, \
             default_new_resource_is_private = EXCLUDED.default_new_resource_is_private, media_webp_quality = EXCLUDED.media_webp_quality, \
             site_name = EXCLUDED.site_name, site_description = EXCLUDED.site_description, public_base_url = EXCLUDED.public_base_url, \
             nostr_names = EXCLUDED.nostr_names, nostr_relays = EXCLUDED.nostr_relays, live_ice_servers = EXCLUDED.live_ice_servers, \
             site_icon_key = EXCLUDED.site_icon_key, site_icon_content_type = EXCLUDED.site_icon_content_type, \
             site_icon_updated_at = CASE WHEN app_settings.site_icon_key IS DISTINCT FROM EXCLUDED.site_icon_key THEN NOW() ELSE app_settings.site_icon_updated_at END, updated_at = NOW()",
            &[
                &settings.home_recent_limit,
                &settings.home_favorite_limit,
                &settings.home_popular_limit,
                &settings.home_intro_markdown,
                &settings.home_recent_visible,
                &settings.home_favorite_visible,
                &settings.home_popular_visible,
                &settings.home_recent_position,
                &settings.home_favorite_position,
                &settings.home_popular_position,
                &settings.search_results_per_page,
                &settings.session_timeout_minutes,
                &settings.default_new_resource_is_private,
                &settings.media_webp_quality,
                &settings.site_name,
                &settings.site_description,
                &settings.public_base_url,
                &settings.nostr_names,
                &settings.nostr_relays,
                &settings.live_ice_servers,
                &settings.site_icon_key,
                &settings.site_icon_content_type,
            ],
        )
        .await
        .map(|_| ())
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn get_resource_stats(
    pool: &DbPool,
    include_private: bool,
) -> Result<ResourceStats, AppError> {
    client(pool)
        .await?
        .query_one(
            "WITH rollup AS (SELECT resource_id, COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE), 0)::BIGINT AS view_count_1d, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 6), 0)::BIGINT AS view_count_7d, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 29), 0)::BIGINT AS view_count_30d, \
             COALESCE(SUM(view_count) FILTER (WHERE view_date >= CURRENT_DATE - 89), 0)::BIGINT AS view_count_90d \
             FROM resource_daily_views GROUP BY resource_id) \
             SELECT COUNT(*) AS total, COUNT(*) FILTER (WHERE is_private = FALSE) AS public_count, COUNT(*) FILTER (WHERE is_private = TRUE) AS private_count, \
             COUNT(*) FILTER (WHERE is_favorite = TRUE) AS favorite_count, COUNT(*) FILTER (WHERE updated_at >= date_trunc('month', NOW())) AS updated_this_month, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('year', NOW())) AS updated_this_year, COALESCE(SUM(view_count_total), 0)::BIGINT AS view_count_total, \
             COALESCE(SUM(rollup.view_count_1d), 0)::BIGINT AS view_count_1d, COALESCE(SUM(rollup.view_count_7d), 0)::BIGINT AS view_count_7d, COALESCE(SUM(rollup.view_count_30d), 0)::BIGINT AS view_count_30d, \
             COALESCE(SUM(rollup.view_count_90d), 0)::BIGINT AS view_count_90d \
             FROM resources LEFT JOIN rollup ON rollup.resource_id = resources.id WHERE deleted_at IS NULL AND ($1 OR is_private = FALSE)",
            &[&include_private],
        )
        .await
        .map(|row| ResourceStats {
            total: row.get("total"),
            public_count: row.get("public_count"),
            private_count: row.get("private_count"),
            favorite_count: row.get("favorite_count"),
            updated_this_month: row.get("updated_this_month"),
            updated_this_year: row.get("updated_this_year"),
            view_count_total: row.get("view_count_total"),
            view_count_1d: row.get("view_count_1d"),
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
        home_intro_markdown: row.get("home_intro_markdown"),
        home_recent_visible: row.get("home_recent_visible"),
        home_favorite_visible: row.get("home_favorite_visible"),
        home_popular_visible: row.get("home_popular_visible"),
        home_recent_position: row.get("home_recent_position"),
        home_favorite_position: row.get("home_favorite_position"),
        home_popular_position: row.get("home_popular_position"),
        search_results_per_page: row.get("search_results_per_page"),
        session_timeout_minutes: row.get("session_timeout_minutes"),
        default_new_resource_is_private: row.get("default_new_resource_is_private"),
        media_webp_quality: row.get("media_webp_quality"),
        site_name: row.get("site_name"),
        site_description: row.get("site_description"),
        public_base_url: row.get("public_base_url"),
        nostr_names: row.get("nostr_names"),
        nostr_relays: row.get("nostr_relays"),
        live_ice_servers: row.get("live_ice_servers"),
        site_icon_key: row.get("site_icon_key"),
        site_icon_content_type: row.get("site_icon_content_type"),
    }
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
