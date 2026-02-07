//! Fuzzy finder scoring algorithm.

use serde::{Deserialize, Serialize};

/// Result of a fuzzy match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyMatch {
    /// Higher is better.
    pub score: i64,
    /// Indices of matched characters in the candidate.
    pub indices: Vec<usize>,
}

/// Attempt a fuzzy match of `pattern` against `candidate`.
///
/// Returns `None` if the pattern cannot be matched.
pub fn fuzzy_match(pattern: &str, candidate: &str) -> Option<FuzzyMatch> {
    if pattern.is_empty() {
        return Some(FuzzyMatch { score: 0, indices: Vec::new() });
    }

    let pat_lower: Vec<char> = pattern.chars().map(|c| c.to_ascii_lowercase()).collect();
    let cand_chars: Vec<char> = candidate.chars().collect();
    let cand_lower: Vec<char> = cand_chars.iter().map(|c| c.to_ascii_lowercase()).collect();

    let mut indices = Vec::with_capacity(pat_lower.len());
    let mut score: i64 = 0;
    let mut pi = 0;
    let mut prev_matched = false;
    let mut prev_idx: Option<usize> = None;

    for (ci, &cc) in cand_lower.iter().enumerate() {
        if pi < pat_lower.len() && cc == pat_lower[pi] {
            indices.push(ci);
            // Exact case match bonus.
            if cand_chars[ci] == pattern.chars().nth(pi).unwrap_or(' ') {
                score += 2;
            } else {
                score += 1;
            }
            // Consecutive bonus.
            if prev_matched {
                score += 4;
            }
            // Word boundary bonus.
            if ci == 0 || !cand_chars[ci - 1].is_alphanumeric() {
                score += 3;
            }
            // Penalize distance from previous match.
            if let Some(prev) = prev_idx {
                let gap = ci - prev - 1;
                score -= gap as i64;
            }
            pi += 1;
            prev_matched = true;
            prev_idx = Some(ci);
        } else {
            prev_matched = false;
        }
    }

    if pi < pat_lower.len() {
        None
    } else {
        Some(FuzzyMatch { score, indices })
    }
}

/// Rank candidates by fuzzy match score (descending). Returns (index, match) pairs.
pub fn rank_candidates(pattern: &str, candidates: &[&str]) -> Vec<(usize, FuzzyMatch)> {
    let mut results: Vec<(usize, FuzzyMatch)> = candidates
        .iter()
        .enumerate()
        .filter_map(|(i, c)| fuzzy_match(pattern, c).map(|m| (i, m)))
        .collect();
    results.sort_by(|a, b| b.1.score.cmp(&a.1.score));
    results
}

/// Normalize a raw score to [0.0, 1.0] based on candidate length.
pub fn normalize_score(score: i64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    let max_possible = (len as i64) * 10; // rough upper bound
    let clamped = score.max(0).min(max_possible);
    clamped as f64 / max_possible as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let m = fuzzy_match("abc", "abc").unwrap();
        assert_eq!(m.indices, vec![0, 1, 2]);
        assert!(m.score > 0);
    }

    #[test]
    fn no_match() {
        assert!(fuzzy_match("xyz", "abc").is_none());
    }

    #[test]
    fn empty_pattern_matches_all() {
        let m = fuzzy_match("", "anything").unwrap();
        assert_eq!(m.score, 0);
        assert!(m.indices.is_empty());
    }

    #[test]
    fn rank_order() {
        let candidates = vec!["foo_bar", "foobar", "fb", "xfb"];
        let ranked = rank_candidates("fb", &candidates);
        assert!(!ranked.is_empty());
        // "fb" should be the best match
        assert_eq!(ranked[0].0, 2);
    }

    #[test]
    fn normalize() {
        let n = normalize_score(5, 10);
        assert!(n > 0.0 && n <= 1.0);
        assert_eq!(normalize_score(0, 0), 0.0);
    }
}
