//! Reciprocal Rank Fusion

use kjxlkj_domain::SearchResult;

/// Reciprocal Rank Fusion with parameter k
pub fn reciprocal_rank_fusion(
    lexical: &[SearchResult],
    semantic: &[SearchResult],
    k: usize,
) -> Vec<SearchResult> {
    use std::collections::HashMap;
    
    let mut scores: HashMap<uuid::Uuid, (SearchResult, f64)> = HashMap::new();

    // Add lexical scores
    for (rank, result) in lexical.iter().enumerate() {
        let rrf_score = 1.0 / (k + rank + 1) as f64;
        let entry = scores.entry(result.note_id).or_insert_with(|| {
            (result.clone(), 0.0)
        });
        entry.1 += rrf_score;
        entry.0.score_lexical = result.score_lexical;
        entry.0.score_rrf = entry.1;
    }

    // Add semantic scores
    for (rank, result) in semantic.iter().enumerate() {
        let rrf_score = 1.0 / (k + rank + 1) as f64;
        let entry = scores.entry(result.note_id).or_insert_with(|| {
            (result.clone(), 0.0)
        });
        entry.1 += rrf_score;
        entry.0.score_semantic = result.score_semantic;
        entry.0.score_rrf = entry.1;
    }

    // Sort by RRF score
    let mut results: Vec<_> = scores.into_values().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Update final scores
    for (result, score) in &mut results {
        result.score_final = *score;
    }

    results.into_iter().map(|(r, _)| r).collect()
}
