//! Database row models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub alias: Option<String>,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub is_favorite: bool,
    pub favorite_position: Option<i64>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRevision {
    pub revision_number: i32,
    pub body: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ListedRecord {
    pub record: Record,
    pub preview: String,
}

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub home_recent_limit: i64,
    pub home_favorite_limit: i64,
    pub search_results_per_page: i64,
    pub default_vim_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            home_recent_limit: 6,
            home_favorite_limit: 6,
            search_results_per_page: 20,
            default_vim_mode: false,
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
}
