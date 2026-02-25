//! Search service - hybrid lexical/semantic retrieval

pub mod lexical;
pub mod semantic;
pub mod fusion;
pub mod embedding;

use chrono::Utc;
use kjxlkj_domain::{SearchQuery, SearchResponse, SearchResult, SearchMode, SearchTiming};
use uuid::Uuid;

pub use lexical::*;
pub use semantic::*;
pub use fusion::*;
pub use embedding::*;

use kjxlkj_db::NoteRepo;

/// Search service
#[derive(Debug, Clone)]
pub struct SearchService {
    note_repo: NoteRepo,
    lexical: LexicalSearch,
    semantic: Option<SemanticSearch>,
}

impl SearchService {
    pub fn new(note_repo: NoteRepo, embedding_config: Option<EmbeddingConfig>) -> Self {
        Self {
            note_repo,
            lexical: LexicalSearch::new(),
            semantic: embedding_config.map(SemanticSearch::new),
        }
    }

    pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let start = Utc::now();
        let mut timing = SearchTiming::default();

        // Normalize query
        let normalized = normalize_query(&query.q);

        // Parallel retrieval
        let lexical_start = Utc::now();
        let lexical_results = self.lexical.search(&normalized, &query).await?;
        timing.lexical = (Utc::now() - lexical_start).num_milliseconds() as u64;

        let mut semantic_results = Vec::new();
        let mut degraded = false;
        let mut degraded_reason = None;

        if query.mode != SearchMode::Lexical {
            if let Some(ref semantic) = self.semantic {
                let semantic_start = Utc::now();
                match semantic.search(&normalized, &query).await {
                    Ok(results) => {
                        semantic_results = results;
                        timing.semantic = (Utc::now() - semantic_start).num_milliseconds() as u64;
                    }
                    Err(e) => {
                        degraded = true;
                        degraded_reason = Some(format!("Embedding unavailable: {}", e));
                    }
                }
            } else {
                degraded = true;
                degraded_reason = Some("Semantic search not configured".into());
            }
        }

        // Fusion
        let fusion_start = Utc::now();
        let fused = if query.mode == SearchMode::Hybrid && !semantic_results.is_empty() {
            reciprocal_rank_fusion(&lexical_results, &semantic_results, 60)
        } else if query.mode == SearchMode::Semantic && !semantic_results.is_empty() {
            semantic_results
        } else {
            lexical_results
        };
        timing.fusion = (Utc::now() - fusion_start).num_milliseconds() as u64;

        // Apply pagination
        let total = fused.len();
        let paginated: Vec<_> = fused
            .into_iter()
            .skip(query.offset)
            .take(query.limit)
            .collect();

        timing.total = (Utc::now() - start).num_milliseconds() as u64;

        let mode_str = match query.mode {
            SearchMode::Hybrid => "hybrid",
            SearchMode::Lexical => "lexical",
            SearchMode::Semantic => "semantic",
        };

        Ok(SearchResponse {
            results: paginated,
            total,
            mode: mode_str.to_string(),
            degraded,
            degraded_reason,
            query_normalized: normalized.clone(),
            query_expanded: vec![normalized.clone()],
            timing_ms: timing,
        })
    }
}

/// Normalize query string
fn normalize_query(q: &str) -> String {
    q.trim().to_lowercase()
}

/// Search error types
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Query validation failed: {0}")]
    ValidationError(String),

    #[error("Lexical search failed: {0}")]
    LexicalError(String),

    #[error("Semantic search failed: {0}")]
    SemanticError(String),

    #[error("Embedding provider error: {0}")]
    EmbeddingError(String),

    #[error("Database error: {0}")]
    DbError(String),
}
