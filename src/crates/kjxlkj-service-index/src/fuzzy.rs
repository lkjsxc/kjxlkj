//! Fuzzy matching.

/// Result of a fuzzy match.
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// The matched string.
    pub text: String,
    /// Match score (higher is better).
    pub score: i32,
    /// Indices of matched characters.
    pub matches: Vec<usize>,
}

/// Fuzzy matcher.
#[derive(Debug, Default)]
pub struct FuzzyMatcher {
    /// Whether matching is case sensitive.
    case_sensitive: bool,
}

impl FuzzyMatcher {
    /// Creates a new fuzzy matcher.
    pub fn new() -> Self {
        Self {
            case_sensitive: false,
        }
    }

    /// Sets case sensitivity.
    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    /// Matches a pattern against text.
    pub fn match_pattern(&self, pattern: &str, text: &str) -> Option<MatchResult> {
        if pattern.is_empty() {
            return Some(MatchResult {
                text: text.to_string(),
                score: 0,
                matches: vec![],
            });
        }

        let pattern_chars: Vec<char> = if self.case_sensitive {
            pattern.chars().collect()
        } else {
            pattern.to_lowercase().chars().collect()
        };

        let text_chars: Vec<char> = text.chars().collect();
        let text_lower: Vec<char> = if self.case_sensitive {
            text_chars.clone()
        } else {
            text.to_lowercase().chars().collect()
        };

        let mut pattern_idx = 0;
        let mut matches = Vec::new();
        let mut score = 0;
        let mut prev_matched = false;

        for (i, &c) in text_lower.iter().enumerate() {
            if pattern_idx < pattern_chars.len() && c == pattern_chars[pattern_idx] {
                matches.push(i);
                pattern_idx += 1;

                // Scoring
                if i == 0 {
                    score += 10; // Start of string bonus
                } else if !text_chars[i - 1].is_alphanumeric() {
                    score += 8; // Start of word bonus
                } else if prev_matched {
                    score += 5; // Consecutive bonus
                } else {
                    score += 1;
                }

                // Case match bonus
                if text_chars[i] == pattern.chars().nth(pattern_idx - 1).unwrap_or(' ') {
                    score += 1;
                }

                prev_matched = true;
            } else {
                prev_matched = false;
            }
        }

        if pattern_idx == pattern_chars.len() {
            // Bonus for shorter strings
            score += 100 / (text.len() as i32 + 1);

            Some(MatchResult {
                text: text.to_string(),
                score,
                matches,
            })
        } else {
            None
        }
    }

    /// Matches and sorts multiple candidates.
    pub fn match_all<'a>(
        &self,
        pattern: &str,
        candidates: impl Iterator<Item = &'a str>,
    ) -> Vec<MatchResult> {
        let mut results: Vec<_> = candidates
            .filter_map(|c| self.match_pattern(pattern, c))
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }
}
