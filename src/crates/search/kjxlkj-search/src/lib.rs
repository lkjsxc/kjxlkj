//! Search services for kjxlkj.
//!
//! This crate contains full-text search and backlink extraction.

use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Search query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub workspace_id: Uuid,
    pub text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub note_kind: Option<String>,
    pub project_id: Option<Uuid>,
    pub include_deleted: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Search result.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub note_id: Uuid,
    pub title: String,
    pub snippet: String,
    pub score: f64,
    pub note_kind: String,
    pub updated_at: String,
}

/// Search service.
pub struct SearchService;

impl SearchService {
    /// Extract wiki-links from markdown content.
    pub fn extract_wiki_links(content: &str) -> Vec<String> {
        let mut links = Vec::new();
        let mut start = 0;

        while let Some(begin) = content[start..].find("[[") {
            let begin = start + begin;
            if let Some(end) = content[begin..].find("]]") {
                let end = begin + end;
                let link_text = content[begin + 2..end].to_string();
                links.push(link_text);
                start = end + 2;
            } else {
                break;
            }
        }

        links
    }

    /// Extract hashtags from markdown content.
    pub fn extract_hashtags(content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();

        for word in words {
            if word.starts_with('#') && word.len() > 1 {
                let tag = word[1..].to_string();
                // Remove trailing punctuation
                let tag = tag.trim_end_matches(|c| !char::is_alphanumeric(c)).to_string();
                if !tag.is_empty() {
                    tags.push(tag);
                }
            }
        }

        tags
    }

    /// Generate search snippet from content.
    pub fn generate_snippet(content: &str, query: &str, max_length: usize) -> String {
        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();

        if let Some(pos) = content_lower.find(&query_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            let mut snippet = content[start..end].to_string();
            if start > 0 {
                snippet = format!("...{}", snippet);
            }
            if end < content.len() {
                snippet = format!("{}...", snippet);
            }
            if snippet.len() > max_length {
                snippet = snippet[..max_length].to_string();
            }
            snippet
        } else {
            content.chars().take(max_length).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wiki_links() {
        let content = "This links to [[Note A]] and [[Note B]].";
        let links = SearchService::extract_wiki_links(content);
        assert_eq!(links, vec!["Note A", "Note B"]);
    }

    #[test]
    fn test_extract_hashtags() {
        let content = "This has #tag1 and #tag2 in it.";
        let tags = SearchService::extract_hashtags(content);
        assert_eq!(tags, vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_generate_snippet() {
        let content = "This is a long piece of content with the search term somewhere in the middle.";
        let snippet = SearchService::generate_snippet(content, "search term", 100);
        assert!(snippet.contains("search term"));
    }
}
