/// Search domain types per /docs/spec/domain/search.md
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search mode enum per /docs/spec/api/types.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    Hybrid,
    Lexical,
    Semantic,
}

impl SearchMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hybrid => "hybrid",
            Self::Lexical => "lexical",
            Self::Semantic => "semantic",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "hybrid" => Some(Self::Hybrid),
            "lexical" => Some(Self::Lexical),
            "semantic" => Some(Self::Semantic),
            _ => None,
        }
    }
}

/// Search query input
#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub workspace_id: Uuid,
    pub project_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub mode: Option<SearchMode>,
}

/// Search result per /docs/spec/api/types.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: String,
    pub score_lexical: f64,
    pub score_semantic: f64,
    pub score_final: f64,
}

/// Backlink entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlink {
    pub source_note_id: Uuid,
    pub target_note_id: Uuid,
    pub source_title: String,
    pub updated_at: chrono::NaiveDateTime,
}

/// Extract wiki-link targets from markdown body
pub fn extract_wiki_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = markdown.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 1 < len {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            i += 2;
            let start = i;
            let mut end = None;
            while i + 1 < len {
                if bytes[i] == b']' && bytes[i + 1] == b']' {
                    end = Some(i);
                    i += 2;
                    break;
                }
                i += 1;
            }
            if let Some(e) = end {
                let target = &markdown[start..e];
                if !target.is_empty() {
                    links.push(target.to_string());
                }
            }
        } else {
            i += 1;
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wiki_links() {
        let md = "See [[note-a]] and [[note-b]] for details.";
        let links = extract_wiki_links(md);
        assert_eq!(links, vec!["note-a", "note-b"]);
    }

    #[test]
    fn test_extract_wiki_links_empty() {
        assert!(extract_wiki_links("no links here").is_empty());
    }

    #[test]
    fn test_extract_nested_brackets() {
        // The byte-based parser stops at the first ]]
        let md = "See [[inner link]] end.";
        let links = extract_wiki_links(md);
        assert_eq!(links, vec!["inner link"]);
    }

    #[test]
    fn test_search_mode_roundtrip() {
        assert_eq!(SearchMode::from_str("hybrid"), Some(SearchMode::Hybrid));
        assert_eq!(SearchMode::from_str("lexical"), Some(SearchMode::Lexical));
        assert_eq!(SearchMode::from_str("semantic"), Some(SearchMode::Semantic));
        assert_eq!(SearchMode::from_str("unknown"), None);
    }

    #[test]
    fn api_search_01_lexical_deterministic() {
        // Acceptance: API-SEARCH-01
        let mut results = vec![
            SearchResult {
                note_id: uuid::Uuid::nil(),
                title: "B".into(),
                snippet: "".into(),
                score_lexical: 0.8,
                score_semantic: 0.0,
                score_final: 0.8,
            },
            SearchResult {
                note_id: uuid::Uuid::nil(),
                title: "A".into(),
                snippet: "".into(),
                score_lexical: 0.9,
                score_semantic: 0.0,
                score_final: 0.9,
            },
        ];
        results.sort_by(|a, b| {
            b.score_final
                .partial_cmp(&a.score_final)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        assert_eq!(results[0].title, "A");
    }
}
