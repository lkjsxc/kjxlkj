use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::core::settings::{SiteSettings, DEFAULT_SESSION_TIMEOUT_MINUTES, DEFAULT_SITE_TITLE};
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct SettingsRepository {
    pool: PgPool,
}

impl SettingsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn load(&self) -> Result<SiteSettings, AppError> {
        let row = sqlx::query_as::<_, (String, i32, Option<DateTime<Utc>>)>(
            "SELECT site_title, session_timeout_minutes, search_last_reindex_at FROM site_settings WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::database_query)?;
        Ok(row.map_or_else(
            default_settings,
            |(site_title, session_timeout_minutes, search_last_reindex_at)| SiteSettings {
                site_title,
                session_timeout_minutes,
                search_last_reindex_at,
            },
        ))
    }

    pub async fn save(
        &self,
        site_title: &str,
        session_timeout_minutes: i32,
    ) -> Result<SiteSettings, AppError> {
        let (site_title, session_timeout_minutes, search_last_reindex_at) =
            sqlx::query_as::<_, (String, i32, Option<DateTime<Utc>>)>(
                "INSERT INTO site_settings (id, site_title, session_timeout_minutes)
                 VALUES (1, $1, $2)
                 ON CONFLICT (id) DO UPDATE
                 SET site_title = EXCLUDED.site_title,
                     session_timeout_minutes = EXCLUDED.session_timeout_minutes
                 RETURNING site_title, session_timeout_minutes, search_last_reindex_at",
            )
            .bind(site_title)
            .bind(session_timeout_minutes)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::database_query)?;
        Ok(SiteSettings {
            site_title,
            session_timeout_minutes,
            search_last_reindex_at,
        })
    }

    pub async fn touch_reindex_timestamp(
        &self,
        at: DateTime<Utc>,
    ) -> Result<SiteSettings, AppError> {
        let (site_title, session_timeout_minutes, search_last_reindex_at) =
            sqlx::query_as::<_, (String, i32, Option<DateTime<Utc>>)>(
                "INSERT INTO site_settings (id, site_title, session_timeout_minutes, search_last_reindex_at)
                 VALUES (1, $1, $2, $3)
                 ON CONFLICT (id) DO UPDATE
                 SET search_last_reindex_at = EXCLUDED.search_last_reindex_at
                 RETURNING site_title, session_timeout_minutes, search_last_reindex_at",
            )
            .bind(DEFAULT_SITE_TITLE)
            .bind(DEFAULT_SESSION_TIMEOUT_MINUTES)
            .bind(at)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::database_query)?;
        Ok(SiteSettings {
            site_title,
            session_timeout_minutes,
            search_last_reindex_at,
        })
    }
}

fn default_settings() -> SiteSettings {
    SiteSettings::default()
}
