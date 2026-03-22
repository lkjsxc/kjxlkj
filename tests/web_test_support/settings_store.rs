use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use kjxlkj::core::settings::SiteSettings;
use kjxlkj::error::AppError;
use kjxlkj::web::state::SettingsStore;

#[derive(Clone, Default)]
pub struct MockSettingsStore {
    inner: Arc<Mutex<SiteSettings>>,
}

#[async_trait]
impl SettingsStore for MockSettingsStore {
    async fn load_settings(&self) -> Result<SiteSettings, AppError> {
        Ok(self.inner.lock().expect("settings lock poisoned").clone())
    }

    async fn save_settings(
        &self,
        site_title: &str,
        session_timeout_minutes: i32,
    ) -> Result<SiteSettings, AppError> {
        let mut current = self.inner.lock().expect("settings lock poisoned");
        current.site_title = site_title.to_owned();
        current.session_timeout_minutes = session_timeout_minutes;
        Ok(current.clone())
    }

    async fn touch_reindex_timestamp(&self, at: DateTime<Utc>) -> Result<SiteSettings, AppError> {
        let mut current = self.inner.lock().expect("settings lock poisoned");
        current.search_last_reindex_at = Some(at);
        Ok(current.clone())
    }
}
