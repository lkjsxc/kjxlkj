//! Search highlighting integration.
//!
//! Combines search state with buffer highlighting.

use kjxlkj_core_types::{Position, Range};

/// A highlighted search match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHighlight {
    /// Range of the match.
    pub range: Range,
    /// Whether this is the current match.
    pub is_current: bool,
}

impl SearchHighlight {
    /// Creates a new search highlight.
    pub fn new(range: Range) -> Self {
        Self {
            range,
            is_current: false,
        }
    }

    /// Marks this as the current match.
    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }
}

/// Search result with highlights.
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    /// Pattern searched.
    pub pattern: String,
    /// All matches.
    pub matches: Vec<SearchHighlight>,
    /// Current match index.
    pub current_index: Option<usize>,
}

impl SearchResult {
    /// Creates an empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pattern and finds matches.
    pub fn set_pattern(&mut self, pattern: &str, text: &str) {
        self.pattern = pattern.to_string();
        self.matches.clear();
        self.current_index = None;

        if pattern.is_empty() {
            return;
        }

        // Find all matches.
        let mut line = 0;
        let mut col = 0;

        for (i, ch) in text.char_indices() {
            if text[i..].starts_with(pattern) {
                let start = Position::new(line, col);
                let end_col = col + pattern.chars().count();
                let end = Position::new(line, end_col);
                self.matches.push(SearchHighlight::new(Range::new(start, end)));
            }

            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        if !self.matches.is_empty() {
            self.current_index = Some(0);
        }
    }

    /// Returns the current match.
    pub fn current_match(&self) -> Option<&SearchHighlight> {
        self.current_index.and_then(|i| self.matches.get(i))
    }

    /// Moves to the next match.
    pub fn next(&mut self) -> Option<&SearchHighlight> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_index = Some(match self.current_index {
            Some(i) => (i + 1) % self.matches.len(),
            None => 0,
        });
        self.current_match()
    }

    /// Moves to the previous match.
    pub fn prev(&mut self) -> Option<&SearchHighlight> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_index = Some(match self.current_index {
            Some(0) => self.matches.len() - 1,
            Some(i) => i - 1,
            None => self.matches.len() - 1,
        });
        self.current_match()
    }

    /// Returns match count.
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Returns matches on a specific line.
    pub fn matches_on_line(&self, line: usize) -> Vec<&SearchHighlight> {
        self.matches
            .iter()
            .filter(|m| m.range.start.line == line || m.range.end.line == line)
            .collect()
    }

    /// Finds the nearest match to a position.
    pub fn find_nearest(&mut self, pos: Position, forward: bool) {
        if self.matches.is_empty() {
            return;
        }

        let idx = if forward {
            self.matches
                .iter()
                .position(|m| m.range.start > pos)
                .unwrap_or(0)
        } else {
            self.matches
                .iter()
                .rposition(|m| m.range.start < pos)
                .unwrap_or(self.matches.len() - 1)
        };

        self.current_index = Some(idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_highlight_new() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let hl = SearchHighlight::new(range);
        assert!(!hl.is_current);
    }

    #[test]
    fn test_search_highlight_current() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let hl = SearchHighlight::new(range).current();
        assert!(hl.is_current);
    }

    #[test]
    fn test_search_result_set_pattern() {
        let mut result = SearchResult::new();
        result.set_pattern("foo", "foo bar foo");
        assert_eq!(result.match_count(), 2);
    }

    #[test]
    fn test_search_result_next() {
        let mut result = SearchResult::new();
        result.set_pattern("foo", "foo foo foo");
        assert_eq!(result.current_index, Some(0));
        result.next();
        assert_eq!(result.current_index, Some(1));
        result.next();
        assert_eq!(result.current_index, Some(2));
        result.next();
        assert_eq!(result.current_index, Some(0)); // Wrap
    }

    #[test]
    fn test_search_result_prev() {
        let mut result = SearchResult::new();
        result.set_pattern("foo", "foo foo foo");
        result.prev();
        assert_eq!(result.current_index, Some(2)); // Wrap backwards
    }

    #[test]
    fn test_matches_on_line() {
        let mut result = SearchResult::new();
        result.set_pattern("x", "x\nx\nx");
        let line1 = result.matches_on_line(1);
        assert_eq!(line1.len(), 1);
    }
}
