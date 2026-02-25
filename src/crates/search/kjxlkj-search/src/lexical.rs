//! Lexical search using BM25-style scoring

use kjxlkj_domain::{SearchQuery, SearchResult};
use uuid::Uuid;

use super::SearchError;

/// Lexical search engine
#[derive(Debug, Clone, Default)]
pub struct LexicalSearch;

impl LexicalSearch {
    pub fn new() -> Self {
        Self
    }

    pub async fn search(
        &self,
        query: &str,
        _search_query: &SearchQuery,
    ) -> Result<Vec<SearchResult>, SearchError> {
        // Simplified in-memory implementation
        // In production, this uses PostgreSQL tsvector + GIN index
        
        let query_terms: Vec<_> = query.split_whitespace().collect();
        
        // Return empty results for now (will be populated from DB in production)
        Ok(vec![])
    }
}
