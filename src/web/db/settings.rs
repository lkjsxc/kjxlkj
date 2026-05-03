//! Personal-space settings and dashboard stats queries

use super::{AppSettings, DbPool, ResourceStats};
use crate::error::AppError;

pub async fn get_settings(pool: &DbPool) -> Result<AppSettings, AppError> {
    let row = client(pool)
        .await?
        .query_opt(
            "SELECT home_recent_limit, home_favorite_limit, home_popular_limit, home_intro_markdown, \
             home_recent_visible, home_favorite_visible, home_popular_visible, home_recent_position, \
             home_favorite_position, home_popular_position, search_results_per_page, \
             default_new_resource_visibility = 'private' AS default_new_resource_is_private, \
             media_webp_quality, site_name, site_description, public_base_url, \
             nostr_names, nostr_relays, live_default_source, live_default_camera_facing, \
             live_default_height, live_default_fps, live_default_microphone_enabled, \
             site_icon_key, site_icon_content_type \
             FROM space_settings ORDER BY updated_at DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(db_err)?;
    Ok(row.map_or_else(AppSettings::default, row_to_settings))
}

pub async fn update_settings(pool: &DbPool, settings: &AppSettings) -> Result<(), AppError> {
    client(pool)
        .await?
        .execute(
            "UPDATE space_settings SET home_recent_limit = $1, home_favorite_limit = $2, \
             home_popular_limit = $3, home_intro_markdown = $4, home_recent_visible = $5, \
             home_favorite_visible = $6, home_popular_visible = $7, home_recent_position = $8, \
             home_favorite_position = $9, home_popular_position = $10, search_results_per_page = $11, \
             default_new_resource_visibility = CASE WHEN $12 THEN 'private'::resource_visibility ELSE 'public'::resource_visibility END, \
             media_webp_quality = $13, site_name = $14, site_description = $15, public_base_url = $16, \
             nostr_names = $17, nostr_relays = $18, live_default_source = $19, live_default_camera_facing = $20, \
             live_default_height = $21, live_default_fps = $22, live_default_microphone_enabled = $23, \
             site_icon_key = $24, site_icon_content_type = $25, \
             site_icon_updated_at = CASE WHEN site_icon_key IS DISTINCT FROM $24 THEN NOW() ELSE site_icon_updated_at END, \
             updated_at = NOW() WHERE space_id = default_space_id()",
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
                &settings.default_new_resource_is_private,
                &settings.media_webp_quality,
                &settings.site_name,
                &settings.site_description,
                &settings.public_base_url,
                &settings.nostr_names,
                &settings.nostr_relays,
                &settings.live_default_source,
                &settings.live_default_camera_facing,
                &settings.live_default_height,
                &settings.live_default_fps,
                &settings.live_default_microphone_enabled,
                &settings.site_icon_key,
                &settings.site_icon_content_type,
            ],
        )
        .await
        .map(|_| ())
        .map_err(db_err)
}

pub async fn get_resource_stats(
    pool: &DbPool,
    space_slug: Option<&str>,
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
             SELECT COUNT(*) AS total, COUNT(*) FILTER (WHERE visibility = 'public') AS public_count, \
             COUNT(*) FILTER (WHERE visibility = 'private') AS private_count, \
             COUNT(*) FILTER (WHERE is_favorite = TRUE) AS favorite_count, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('month', NOW())) AS updated_this_month, \
             COUNT(*) FILTER (WHERE updated_at >= date_trunc('year', NOW())) AS updated_this_year, \
             COALESCE(SUM(view_count_total), 0)::BIGINT AS view_count_total, \
             COALESCE(SUM(rollup.view_count_1d), 0)::BIGINT AS view_count_1d, \
             COALESCE(SUM(rollup.view_count_7d), 0)::BIGINT AS view_count_7d, \
             COALESCE(SUM(rollup.view_count_30d), 0)::BIGINT AS view_count_30d, \
             COALESCE(SUM(rollup.view_count_90d), 0)::BIGINT AS view_count_90d \
             FROM resources LEFT JOIN rollup ON rollup.resource_id = resources.id \
             WHERE deleted_at IS NULL AND ($1 OR visibility = 'public') \
             AND ($2::TEXT IS NULL OR space_id = (SELECT id FROM spaces WHERE slug = $2::CITEXT))",
            &[&include_private, &space_slug],
        )
        .await
        .map(row_to_stats)
        .map_err(db_err)
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
        session_timeout_minutes: 1440,
        default_new_resource_is_private: row.get("default_new_resource_is_private"),
        media_webp_quality: row.get("media_webp_quality"),
        site_name: row.get("site_name"),
        site_description: row.get("site_description"),
        public_base_url: row.get("public_base_url"),
        nostr_names: row.get("nostr_names"),
        nostr_relays: row.get("nostr_relays"),
        live_default_source: row.get("live_default_source"),
        live_default_camera_facing: row.get("live_default_camera_facing"),
        live_default_height: row.get("live_default_height"),
        live_default_fps: row.get("live_default_fps"),
        live_default_microphone_enabled: row.get("live_default_microphone_enabled"),
        google_maps_embed_api_key: String::new(),
        site_icon_key: row.get("site_icon_key"),
        site_icon_content_type: row.get("site_icon_content_type"),
    }
}

fn row_to_stats(row: tokio_postgres::Row) -> ResourceStats {
    ResourceStats {
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
    }
}

fn db_err(error: tokio_postgres::Error) -> AppError {
    AppError::DatabaseError(error.to_string())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
