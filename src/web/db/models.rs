//! Database row models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::media::MediaVariants;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Note,
    Media,
}

impl ResourceKind {
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
    File,
}

impl MediaFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::File => "file",
        }
    }

    pub fn from_db(value: Option<String>) -> Option<Self> {
        match value.as_deref() {
            Some("video") => Some(Self::Video),
            Some("image") => Some(Self::Image),
            Some("file") => Some(Self::File),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub kind: ResourceKind,
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
    pub media_variants: Option<MediaVariants>,
    pub owner_note_id: Option<String>,
    pub is_favorite: bool,
    pub favorite_position: Option<i64>,
    pub is_private: bool,
    pub view_count_total: i64,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub id: String,
    pub kind: ResourceKind,
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
    pub media_variants: Option<MediaVariants>,
    pub owner_note_id: Option<String>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ListedResource {
    pub resource: Resource,
    pub preview: String,
    pub popular_views: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub total: i64,
    pub public_count: i64,
    pub private_count: i64,
    pub favorite_count: i64,
    pub updated_this_month: i64,
    pub updated_this_year: i64,
    pub view_count_total: i64,
    pub view_count_1d: i64,
    pub view_count_7d: i64,
    pub view_count_30d: i64,
    pub view_count_90d: i64,
}

#[derive(Debug, Clone)]
pub struct ResourceViewStats {
    pub total: i64,
    pub views_1d: i64,
    pub views_7d: i64,
    pub views_30d: i64,
    pub views_90d: i64,
    pub last_viewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SitemapResource {
    pub id: String,
    pub alias: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ExternalEmbed {
    pub url: String,
    pub provider: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub author_name: Option<String>,
    pub thumbnail_url: Option<String>,
}
