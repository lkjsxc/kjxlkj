//! Fuzzy matching algorithm.

/// Score a candidate string against a query using fuzzy matching.
///
/// Returns a score (higher is better) or None if no match.
pub fn fuzzy_match(query: &str, candidate: &str) -> Option<i32> {
    if query.is_empty() {
        return Some(0);
    }

    let query_lower = query.to_lowercase();
    let candidate_lower = candidate.to_lowercase();

    let query_chars: Vec<char> = query_lower.chars().collect();
    let candidate_chars: Vec<char> =
        candidate_lower.chars().collect();

    let mut qi = 0;
    let mut score: i32 = 0;
    let mut last_match: Option<usize> = None;

    for (ci, &cc) in candidate_chars.iter().enumerate() {
        if qi < query_chars.len() && cc == query_chars[qi] {
            score += 1;

            // Bonus for consecutive matches.
            if let Some(last) = last_match {
                if ci == last + 1 {
                    score += 2;
                }
            }

            // Bonus for match at start.
            if ci == 0 {
                score += 3;
            }

            // Bonus for match after separator.
            if ci > 0 {
                let prev = candidate_chars[ci - 1];
                if prev == '/' || prev == '\\' || prev == '_'
                    || prev == '-' || prev == '.'
                {
                    score += 2;
                }
            }

            // Bonus for case match.
            let orig_char: Vec<char> =
                candidate.chars().collect();
            if ci < orig_char.len()
                && query.chars().nth(qi) == Some(orig_char[ci])
            {
                score += 1;
            }

            last_match = Some(ci);
            qi += 1;
        }
    }

    if qi == query_chars.len() {
        // Penalize longer candidates.
        score -= (candidate_chars.len() as i32) / 10;
        Some(score)
    } else {
        None
    }
}

/// Sort candidates by fuzzy match score.
pub fn fuzzy_sort(
    query: &str,
    candidates: &[String],
) -> Vec<(String, i32)> {
    let mut scored: Vec<(String, i32)> = candidates
        .iter()
        .filter_map(|c| {
            fuzzy_match(query, c).map(|s| (c.clone(), s))
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match_scores_high() {
        let score = fuzzy_match("abc", "abc").unwrap();
        assert!(score > 0);
    }

    #[test]
    fn no_match_returns_none() {
        assert!(fuzzy_match("xyz", "abc").is_none());
    }

    #[test]
    fn empty_query_matches_all() {
        assert!(fuzzy_match("", "anything").is_some());
    }

    #[test]
    fn partial_match() {
        let score = fuzzy_match("abc", "a_b_c_d").unwrap();
        assert!(score > 0);
    }

    #[test]
    fn sort_by_score() {
        let candidates = vec![
            "apple".into(),
            "application".into(),
            "app".into(),
        ];
        let sorted = fuzzy_sort("app", &candidates);
        assert!(!sorted.is_empty());
        // All three should match.
        assert_eq!(sorted.len(), 3);
        // "app" or "apple" should score higher than "application".
        let last = &sorted[sorted.len() - 1];
        assert_eq!(last.0, "application");
    }
}
