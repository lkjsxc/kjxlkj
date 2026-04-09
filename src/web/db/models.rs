//! Database row models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecordKind {
    Note,
    Media,
}

impl RecordKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Media => "media",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "media" => Self::Media,
            _ => Self::Note,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaFamily {
    Image,
    Video,
}

impl MediaFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
        }
    }

    pub fn from_db(value: Option<String>) -> Option<Self> {
        match value.as_deref() {
            Some("video") => Some(Self::Video),
            Some("image") => Some(Self::Image),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub kind: RecordKind,
    pub alias: Option<String>,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub media_family: Option<MediaFamily>,
    pub file_key: Option<String>,
    pub content_type: Option<String>,
    pub byte_size: Option<i64>,
    pub sha256_hex: Option<String>,
    pub original_filename: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_ms: Option<i64>,
    pub is_favorite: bool,
    pub favorite_position: Option<i64>,
    pub is_private: bool,
    pub view_count_total: i64,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSnapshot {
    pub id: String,
    pub kind: RecordKind,
    pub snapshot_number: i32,
    pub alias: Option<String>,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub media_family: Option<MediaFamily>,
    pub file_key: Option<String>,
    pub content_type: Option<String>,
    pub byte_size: Option<i64>,
    pub sha256_hex: Option<String>,
    pub original_filename: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_ms: Option<i64>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ListedRecord {
    pub record: Record,
    pub preview: String,
    pub popular_views: Option<i64>,
}

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
    pub site_name: String,
    pub site_description: String,
    pub public_base_url: String,
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
            home_recent_position: 2,
            home_favorite_position: 3,
            home_popular_position: 1,
            search_results_per_page: 20,
            session_timeout_minutes: 1440,
            default_new_resource_is_private: false,
            site_name: "kjxlkj".to_string(),
            site_description: "Markdown-first resource system for LLM-operated workflows."
                .to_string(),
            public_base_url: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteStats {
    pub total: i64,
    pub public_count: i64,
    pub private_count: i64,
    pub favorite_count: i64,
    pub updated_this_month: i64,
    pub updated_this_year: i64,
    pub view_count_total: i64,
    pub view_count_7d: i64,
    pub view_count_30d: i64,
    pub view_count_90d: i64,
}

#[derive(Debug, Clone)]
pub struct NoteViewStats {
    pub total: i64,
    pub views_7d: i64,
    pub views_30d: i64,
    pub views_90d: i64,
    pub last_viewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SitemapRecord {
    pub id: String,
    pub alias: Option<String>,
    pub updated_at: DateTime<Utc>,
}
