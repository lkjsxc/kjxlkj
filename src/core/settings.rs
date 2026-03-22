use chrono::{DateTime, Utc};

pub const DEFAULT_SITE_TITLE: &str = "Knowledge Base";
pub const DEFAULT_SESSION_TIMEOUT_MINUTES: i32 = 24 * 60;
pub const MIN_SESSION_TIMEOUT_MINUTES: i32 = 5;
pub const MAX_SESSION_TIMEOUT_MINUTES: i32 = 7 * 24 * 60;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteSettings {
    pub site_title: String,
    pub session_timeout_minutes: i32,
    pub search_last_reindex_at: Option<DateTime<Utc>>,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            site_title: DEFAULT_SITE_TITLE.to_owned(),
            session_timeout_minutes: DEFAULT_SESSION_TIMEOUT_MINUTES,
            search_last_reindex_at: None,
        }
    }
}

impl SiteSettings {
    pub fn normalized_timeout_minutes(value: i32) -> i32 {
        value.clamp(MIN_SESSION_TIMEOUT_MINUTES, MAX_SESSION_TIMEOUT_MINUTES)
    }
}
