//! Search entities and result types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    Hybrid,
    Lexical,
    Semantic,
}

impl Default for SearchMode {
    fn default() -> Self {
        Self::Hybrid
    }
}

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub mode: SearchMode,
    pub limit: usize,
    pub offset: usize,
    pub note_kind: Option<String>,
    pub sort: SearchSort,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            q: String::new(),
            workspace_id: Uuid::nil(),
            project_id: None,
            mode: SearchMode::Hybrid,
            limit: 20,
            offset: 0,
            note_kind: None,
            sort: SearchSort::Relevance,
        }
    }
}

/// Sort order for search results
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSort {
    Relevance,
    UpdatedAt,
}

impl Default for SearchSort {
    fn default() -> Self {
        Self::Relevance
    }
}

/// Search result entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: Option<String>,
    pub score_lexical: f64,
    pub score_semantic: f64,
    pub score_rrf: f64,
    pub score_final: f64,
    pub backlink_count: usize,
    pub updated_at: DateTime<Utc>,
    pub note_kind: String,
    pub workspace_id: Uuid,
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub mode: String,
    pub degraded: bool,
    pub degraded_reason: Option<String>,
    pub query_normalized: String,
    pub query_expanded: Vec<String>,
    pub timing_ms: SearchTiming,
}

/// Search timing breakdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTiming {
    pub lexical: u64,
    pub semantic: u64,
    pub fusion: u64,
    pub rerank: u64,
    pub total: u64,
}

/// Backlink entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlink {
    pub source_note_id: Uuid,
    pub source_title: String,
    pub link_text: String,
    pub snippet: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Backlink response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklinkResponse {
    pub note_id: Uuid,
    pub backlinks: Vec<Backlink>,
    pub total: usize,
}
