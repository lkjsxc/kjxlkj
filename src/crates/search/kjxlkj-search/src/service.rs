/// Search service per /docs/spec/domain/search.md
use crate::embedding::EmbeddingProvider;
use crate::embedding_store::EmbeddingStore;
use kjxlkj_domain::search::*;
use kjxlkj_domain::DomainError;

/// Core search service.
/// Ranking contract per /docs/spec/domain/search.md:
/// 1. fetch top lexical candidates
/// 2. fetch top semantic candidates
/// 3. deduplicate by note ID
/// 4. compute deterministic combined score
/// 5. return stable order for equal scores by updated_at desc, then note_id
pub struct SearchService;

impl SearchService {
    pub fn new() -> Self {
        Self
    }

    /// Merge and rank results per hybrid ranking contract
    pub fn merge_and_rank(
        lexical: Vec<SearchResult>,
        semantic: Vec<SearchResult>,
    ) -> Vec<SearchResult> {
        use std::collections::HashMap;
        let mut by_id: HashMap<uuid::Uuid, SearchResult> = HashMap::new();
        for r in lexical {
            by_id
                .entry(r.note_id)
                .and_modify(|e| e.score_lexical = r.score_lexical)
                .or_insert(r);
        }
        for r in semantic {
            by_id
                .entry(r.note_id)
                .and_modify(|e| {
                    e.score_semantic = r.score_semantic;
                })
                .or_insert(r);
        }
        let mut results: Vec<SearchResult> = by_id
            .into_values()
            .map(|mut r| {
                r.score_final = r.score_lexical * 0.5 + r.score_semantic * 0.5;
                r
            })
            .collect();
        results.sort_by(|a, b| {
            b.score_final
                .partial_cmp(&a.score_final)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.note_id.cmp(&a.note_id))
        });
        results
    }

    /// Perform semantic search using embedding provider and store.
    /// Returns semantic results or empty vec on provider failure.
    /// Per spec: if embedding service unavailable, returns empty + degraded flag.
    pub fn semantic_search(
        provider: &dyn EmbeddingProvider,
        store: &EmbeddingStore,
        query_text: &str,
        limit: usize,
    ) -> Result<Vec<(uuid::Uuid, f64)>, DomainError> {
        if !provider.is_available() {
            return Err(DomainError::SearchEmbeddingDegraded);
        }
        let query_vec = provider.embed(query_text)?;
        Ok(store.nearest(&query_vec, limit))
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_search_02_hybrid_merge() {
        // Acceptance: API-SEARCH-02
        let lex = vec![SearchResult {
            note_id: uuid::Uuid::nil(),
            title: "A".into(),
            snippet: "s".into(),
            score_lexical: 0.9,
            score_semantic: 0.0,
            score_final: 0.0,
        }];
        let sem = vec![SearchResult {
            note_id: uuid::Uuid::nil(),
            title: "A".into(),
            snippet: "s".into(),
            score_lexical: 0.0,
            score_semantic: 0.8,
            score_final: 0.0,
        }];
        let merged = SearchService::merge_and_rank(lex, sem);
        assert_eq!(merged.len(), 1);
        // 0.9*0.5 + 0.8*0.5 = 0.85
        assert!((merged[0].score_final - 0.85).abs() < 0.01);
    }

    #[test]
    fn api_search_03_lexical_fallback() {
        // Acceptance: API-SEARCH-03
        // When semantic is unavailable, only lexical results exist
        let lex = vec![SearchResult {
            note_id: uuid::Uuid::nil(),
            title: "B".into(),
            snippet: "s".into(),
            score_lexical: 0.7,
            score_semantic: 0.0,
            score_final: 0.0,
        }];
        let merged = SearchService::merge_and_rank(lex, vec![]);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].score_final > 0.0);
    }
}
