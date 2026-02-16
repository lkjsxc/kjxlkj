use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search mode per docs/spec/domain/search.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    Hybrid,
    Lexical,
    Semantic,
}

impl SearchMode {
    pub fn from_str_checked(s: &str) -> Option<Self> {
        match s {
            "hybrid" => Some(Self::Hybrid),
            "lexical" => Some(Self::Lexical),
            "semantic" => Some(Self::Semantic),
            _ => None,
        }
    }
}

/// Search query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub mode: Option<SearchMode>,
}

/// Single search result per docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: String,
    pub score_lexical: f64,
    pub score_semantic: f64,
    pub score_final: f64,
}

/// Backlink entry.
#[derive(Debug, Clone, Serialize)]
pub struct BacklinkEntry {
    pub source_note_id: Uuid,
    pub source_title: String,
}
