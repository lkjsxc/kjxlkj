//! Search functionality.

pub use crate::search_types::{SearchDirection, SearchMatch};

/// Search state.
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    /// Current search pattern.
    pattern: Option<String>,
    /// Search direction.
    direction: SearchDirection,
    /// Current matches.
    matches: Vec<SearchMatch>,
    /// Current match index.
    current_match: Option<usize>,
    /// Case sensitivity.
    case_sensitive: bool,
    /// Regex mode.
    regex: bool,
}

impl SearchState {
    /// Creates a new search state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search pattern.
    pub fn set_pattern(&mut self, pattern: String, direction: SearchDirection) {
        self.pattern = Some(pattern);
        self.direction = direction;
        self.matches.clear();
        self.current_match = None;
    }

    /// Returns the current pattern.
    pub fn pattern(&self) -> Option<&str> {
        self.pattern.as_deref()
    }

    /// Returns the direction.
    pub fn direction(&self) -> SearchDirection {
        self.direction
    }

    /// Returns if a search is active.
    pub fn is_active(&self) -> bool {
        self.pattern.is_some()
    }

    /// Sets matches for the current buffer.
    pub fn set_matches(&mut self, matches: Vec<SearchMatch>) {
        self.matches = matches;
        self.current_match = if self.matches.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    /// Returns all matches.
    pub fn matches(&self) -> &[SearchMatch] {
        &self.matches
    }

    /// Returns the current match.
    pub fn current(&self) -> Option<&SearchMatch> {
        self.current_match.and_then(|i| self.matches.get(i))
    }

    /// Returns the current match index.
    pub fn current_index(&self) -> Option<usize> {
        self.current_match
    }

    /// Returns match count.
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Moves to the next match.
    pub fn next_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        let idx = match self.current_match {
            Some(i) => (i + 1) % self.matches.len(),
            None => 0,
        };
        self.current_match = Some(idx);
        self.matches.get(idx)
    }

    /// Moves to the previous match.
    pub fn prev_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        let idx = match self.current_match {
            Some(i) if i > 0 => i - 1,
            Some(_) => self.matches.len() - 1,
            None => self.matches.len() - 1,
        };
        self.current_match = Some(idx);
        self.matches.get(idx)
    }

    /// Clears the search.
    pub fn clear(&mut self) {
        self.pattern = None;
        self.matches.clear();
        self.current_match = None;
    }

    /// Sets case sensitivity.
    pub fn set_case_sensitive(&mut self, sensitive: bool) {
        self.case_sensitive = sensitive;
    }

    /// Returns if case sensitive.
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Finds the match at or after a position.
    pub fn find_at_or_after(&self, line: usize, col: usize) -> Option<usize> {
        self.matches.iter().position(|m| {
            m.line > line || (m.line == line && m.start >= col)
        })
    }

    /// Finds the match before a position.
    pub fn find_before(&self, line: usize, col: usize) -> Option<usize> {
        self.matches.iter().rposition(|m| {
            m.line < line || (m.line == line && m.end <= col)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_state_new() {
        let state = SearchState::new();
        assert!(!state.is_active());
        assert_eq!(state.match_count(), 0);
    }

    #[test]
    fn test_set_pattern() {
        let mut state = SearchState::new();
        state.set_pattern("test".to_string(), SearchDirection::Forward);
        assert!(state.is_active());
        assert_eq!(state.pattern(), Some("test"));
    }

    #[test]
    fn test_set_matches() {
        let mut state = SearchState::new();
        state.set_pattern("x".to_string(), SearchDirection::Forward);
        state.set_matches(vec![
            SearchMatch::new(0, 0, 1),
            SearchMatch::new(1, 5, 6),
        ]);
        assert_eq!(state.match_count(), 2);
        assert_eq!(state.current_index(), Some(0));
    }

    #[test]
    fn test_next_prev_match() {
        let mut state = SearchState::new();
        state.set_pattern("x".to_string(), SearchDirection::Forward);
        state.set_matches(vec![
            SearchMatch::new(0, 0, 1),
            SearchMatch::new(1, 5, 6),
            SearchMatch::new(2, 10, 11),
        ]);

        assert_eq!(state.next_match().unwrap().line, 1);
        assert_eq!(state.next_match().unwrap().line, 2);
        assert_eq!(state.next_match().unwrap().line, 0); // Wraps

        assert_eq!(state.prev_match().unwrap().line, 2);
    }

    #[test]
    fn test_clear() {
        let mut state = SearchState::new();
        state.set_pattern("test".to_string(), SearchDirection::Forward);
        state.set_matches(vec![SearchMatch::new(0, 0, 4)]);
        
        state.clear();
        assert!(!state.is_active());
        assert_eq!(state.match_count(), 0);
    }

    #[test]
    fn test_search_match() {
        let m = SearchMatch::new(5, 10, 15);
        assert_eq!(m.line, 5);
        assert_eq!(m.len(), 5);
        assert!(!m.is_empty());
    }
}
