use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::app_state::AppState;
use crate::core::settings::SiteSettings;
use crate::error::AppError;
use crate::web::state::SettingsStore;

#[derive(Clone)]
pub struct RuntimeSettingsStore {
    pub app_state: AppState,
}

#[async_trait]
impl SettingsStore for RuntimeSettingsStore {
    async fn load_settings(&self) -> Result<SiteSettings, AppError> {
        self.app_state.postgres.settings().load().await
    }

    async fn save_settings(
        &self,
        site_title: &str,
        session_timeout_minutes: i32,
    ) -> Result<SiteSettings, AppError> {
        self.app_state
            .postgres
            .settings()
            .save(site_title, session_timeout_minutes)
            .await
    }

    async fn touch_reindex_timestamp(&self, at: DateTime<Utc>) -> Result<SiteSettings, AppError> {
        self.app_state
            .postgres
            .settings()
            .touch_reindex_timestamp(at)
            .await
    }
}
