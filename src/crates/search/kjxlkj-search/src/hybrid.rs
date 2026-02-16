use kjxlkj_domain::search::{SearchMode, SearchQuery, SearchResult};
use kjxlkj_db::repo_search;
use sqlx::PgPool;
use tracing::warn;
use crate::embedding;

/// Execute hybrid search pipeline per docs/spec/domain/search.md.
/// 1. lexical retrieval via tsvector
/// 2. semantic retrieval via embeddings (if available)
/// 3. deterministic reranking
pub async fn search(
    pool: &PgPool,
    query: &SearchQuery,
    embedding_base_url: &str,
    embedding_model: &str,
    semantic_enabled: bool,
) -> Result<Vec<SearchResult>, SearchError> {
    let mode = query.mode.unwrap_or(SearchMode::Hybrid);
    let limit = query.limit.unwrap_or(20);

    let mut results = Vec::new();

    // Lexical retrieval
    if mode == SearchMode::Hybrid || mode == SearchMode::Lexical {
        let lexical_rows = repo_search::lexical_search(
            pool,
            query.workspace_id,
            &query.q,
            limit,
        )
        .await
        .map_err(SearchError::Db)?;

        for row in lexical_rows {
            results.push(SearchResult {
                note_id: row.note_id,
                title: row.title,
                snippet: row.snippet,
                score_lexical: row.rank as f64,
                score_semantic: 0.0,
                score_final: row.rank as f64,
            });
        }
    }

    // Semantic retrieval
    if semantic_enabled && (mode == SearchMode::Hybrid || mode == SearchMode::Semantic) {
        match embedding::query_embeddings(
            pool,
            &query.q,
            query.workspace_id,
            limit,
            embedding_base_url,
            embedding_model,
        )
        .await
        {
            Ok(semantic_rows) => {
                for (note_id, title, score) in semantic_rows {
                    // Merge or add
                    if let Some(existing) = results.iter_mut().find(|r| r.note_id == note_id) {
                        existing.score_semantic = score;
                        existing.score_final = existing.score_lexical + score;
                    } else {
                        results.push(SearchResult {
                            note_id,
                            title,
                            snippet: String::new(),
                            score_lexical: 0.0,
                            score_semantic: score,
                            score_final: score,
                        });
                    }
                }
            }
            Err(e) => {
                // Fallback: lexical continues; log degradation
                warn!("semantic search degraded: {e}");
            }
        }
    }

    // Deterministic sort: score_final desc, then note_id for stability
    results.sort_by(|a, b| {
        b.score_final
            .partial_cmp(&a.score_final)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.note_id.cmp(&b.note_id))
    });

    results.truncate(limit as usize);
    Ok(results)
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("database error: {0}")]
    Db(sqlx::Error),
    #[error("embedding error: {0}")]
    Embedding(String),
}
