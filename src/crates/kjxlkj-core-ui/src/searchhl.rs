//! Search highlighting.
//!
//! Manages highlight state for search matches.

/// Search match position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchMatch {
    /// Line number (0-indexed).
    pub line: usize,
    /// Start column (0-indexed).
    pub start: usize,
    /// End column (exclusive).
    pub end: usize,
}

impl SearchMatch {
    /// Creates a new search match.
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
    }

    /// Returns whether a position is within this match.
    pub fn contains(&self, line: usize, col: usize) -> bool {
        line == self.line && col >= self.start && col < self.end
    }

    /// Returns match length.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns whether match is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Search highlight state.
#[derive(Debug, Clone, Default)]
pub struct SearchHighlight {
    /// Whether highlighting is enabled.
    pub enabled: bool,
    /// Current search pattern.
    pub pattern: String,
    /// Current match index (for n/N navigation).
    pub current: Option<usize>,
    /// All matches.
    matches: Vec<SearchMatch>,
}

impl SearchHighlight {
    /// Creates new search highlight.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search pattern and clears matches.
    pub fn set_pattern(&mut self, pattern: &str) {
        self.pattern = pattern.to_string();
        self.matches.clear();
        self.current = None;
        self.enabled = !pattern.is_empty();
    }

    /// Adds a match.
    pub fn add_match(&mut self, m: SearchMatch) {
        self.matches.push(m);
    }

    /// Clears all matches.
    pub fn clear_matches(&mut self) {
        self.matches.clear();
        self.current = None;
    }

    /// Returns matches for a line.
    pub fn matches_on_line(&self, line: usize) -> Vec<&SearchMatch> {
        self.matches.iter().filter(|m| m.line == line).collect()
    }

    /// Returns all matches.
    pub fn all_matches(&self) -> &[SearchMatch] {
        &self.matches
    }

    /// Returns match count.
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Moves to next match.
    pub fn next_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        let next = match self.current {
            Some(i) => (i + 1) % self.matches.len(),
            None => 0,
        };
        self.current = Some(next);
        self.matches.get(next)
    }

    /// Moves to previous match.
    pub fn prev_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        let prev = match self.current {
            Some(0) => self.matches.len() - 1,
            Some(i) => i - 1,
            None => self.matches.len() - 1,
        };
        self.current = Some(prev);
        self.matches.get(prev)
    }

    /// Returns current match.
    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.current.and_then(|i| self.matches.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_match_new() {
        let m = SearchMatch::new(5, 10, 15);
        assert_eq!(m.len(), 5);
    }

    #[test]
    fn test_search_match_contains() {
        let m = SearchMatch::new(5, 10, 15);
        assert!(m.contains(5, 12));
        assert!(!m.contains(5, 15));
    }

    #[test]
    fn test_search_highlight_set_pattern() {
        let mut sh = SearchHighlight::new();
        sh.set_pattern("test");
        assert!(sh.enabled);
        assert_eq!(sh.pattern, "test");
    }

    #[test]
    fn test_search_highlight_add_match() {
        let mut sh = SearchHighlight::new();
        sh.add_match(SearchMatch::new(0, 0, 4));
        assert_eq!(sh.match_count(), 1);
    }

    #[test]
    fn test_search_highlight_navigation() {
        let mut sh = SearchHighlight::new();
        sh.add_match(SearchMatch::new(0, 0, 4));
        sh.add_match(SearchMatch::new(1, 0, 4));
        sh.next_match();
        assert_eq!(sh.current, Some(0));
        sh.next_match();
        assert_eq!(sh.current, Some(1));
    }

    #[test]
    fn test_search_highlight_matches_on_line() {
        let mut sh = SearchHighlight::new();
        sh.add_match(SearchMatch::new(5, 0, 4));
        sh.add_match(SearchMatch::new(5, 10, 14));
        let matches = sh.matches_on_line(5);
        assert_eq!(matches.len(), 2);
    }
}
