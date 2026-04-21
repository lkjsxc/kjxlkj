//! App settings model

use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub home_popular_limit: i64,
    pub home_intro_markdown: String,
    pub home_recent_visible: bool,
    pub home_favorite_visible: bool,
    pub home_popular_visible: bool,
    pub home_recent_position: i64,
    pub home_favorite_position: i64,
    pub home_popular_position: i64,
    pub search_results_per_page: i64,
    pub session_timeout_minutes: i64,
    pub default_new_resource_is_private: bool,
    pub media_webp_quality: i64,
    pub site_name: String,
    pub site_description: String,
    pub public_base_url: String,
    pub nostr_names: Value,
    pub nostr_relays: Value,
    pub live_ice_servers: Value,
    pub live_default_source: String,
    pub live_default_height: i64,
    pub live_default_fps: i64,
    pub live_default_microphone_enabled: bool,
    pub site_icon_key: Option<String>,
    pub site_icon_content_type: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            home_recent_limit: 5,
            home_favorite_limit: 5,
            home_popular_limit: 5,
            home_intro_markdown: String::new(),
            home_recent_visible: true,
            home_favorite_visible: true,
            home_popular_visible: true,
            home_recent_position: 1,
            home_favorite_position: 2,
            home_popular_position: 3,
            search_results_per_page: 20,
            session_timeout_minutes: 1440,
            default_new_resource_is_private: false,
            media_webp_quality: 82,
            site_name: "kjxlkj".to_string(),
            site_description: "Markdown-first resource system for LLM-operated workflows."
                .to_string(),
            public_base_url: String::new(),
            nostr_names: json!({}),
            nostr_relays: json!([]),
            live_ice_servers: json!([{ "urls": ["stun:stun.l.google.com:19302"] }]),
            live_default_source: "screen".to_string(),
            live_default_height: 1080,
            live_default_fps: 60,
            live_default_microphone_enabled: false,
            site_icon_key: None,
            site_icon_content_type: None,
        }
    }
}
