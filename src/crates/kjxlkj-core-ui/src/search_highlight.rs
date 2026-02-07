//! Search match highlighting primitives.

use kjxlkj_core_types::Position;
use serde::{Deserialize, Serialize};

/// A single search match in the buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchMatch {
    pub start: Position,
    pub end: Position,
    pub is_current: bool,
}

/// Collection of search matches with navigation state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchHighlights {
    pub matches: Vec<SearchMatch>,
    pub current_idx: Option<usize>,
    pub hl_search: bool,
}

impl SearchHighlights {
    /// Create a new empty highlight set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a match at the given start/end positions.
    pub fn add_match(&mut self, start: Position, end: Position) {
        self.matches.push(SearchMatch {
            start,
            end,
            is_current: false,
        });
    }

    /// Clear all matches and reset state.
    pub fn clear(&mut self) {
        self.matches.clear();
        self.current_idx = None;
    }

    /// Advance to the next match, wrapping around.
    pub fn next_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        let idx = match self.current_idx {
            Some(i) => (i + 1) % self.matches.len(),
            None => 0,
        };
        self.set_current(idx);
    }

    /// Move to the previous match, wrapping around.
    pub fn prev_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        let idx = match self.current_idx {
            Some(0) => self.matches.len() - 1,
            Some(i) => i - 1,
            None => self.matches.len() - 1,
        };
        self.set_current(idx);
    }

    /// Return matches visible between `top_line` and `bottom_line` (inclusive).
    pub fn visible_matches(&self, top_line: usize, bottom_line: usize) -> Vec<&SearchMatch> {
        self.matches
            .iter()
            .filter(|m| m.start.line <= bottom_line && m.end.line >= top_line)
            .collect()
    }

    /// Set the current match index and update `is_current` flags.
    pub fn set_current(&mut self, idx: usize) {
        if idx >= self.matches.len() {
            return;
        }
        // Clear old current
        if let Some(old) = self.current_idx {
            if old < self.matches.len() {
                self.matches[old].is_current = false;
            }
        }
        self.matches[idx].is_current = true;
        self.current_idx = Some(idx);
    }

    /// Toggle the `hlsearch` flag.
    pub fn toggle_hl_search(&mut self) {
        self.hl_search = !self.hl_search;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_clear() {
        let mut hl = SearchHighlights::new();
        hl.add_match(Position::new(0, 0), Position::new(0, 5));
        hl.add_match(Position::new(2, 0), Position::new(2, 3));
        assert_eq!(hl.matches.len(), 2);
        hl.clear();
        assert!(hl.matches.is_empty());
    }

    #[test]
    fn next_prev_match() {
        let mut hl = SearchHighlights::new();
        hl.add_match(Position::new(0, 0), Position::new(0, 1));
        hl.add_match(Position::new(1, 0), Position::new(1, 1));
        hl.add_match(Position::new(2, 0), Position::new(2, 1));

        hl.next_match();
        assert_eq!(hl.current_idx, Some(0));
        hl.next_match();
        assert_eq!(hl.current_idx, Some(1));
        hl.next_match();
        assert_eq!(hl.current_idx, Some(2));
        hl.next_match(); // wraps
        assert_eq!(hl.current_idx, Some(0));

        hl.prev_match();
        assert_eq!(hl.current_idx, Some(2));
    }

    #[test]
    fn visible_matches_filter() {
        let mut hl = SearchHighlights::new();
        hl.add_match(Position::new(0, 0), Position::new(0, 3));
        hl.add_match(Position::new(5, 0), Position::new(5, 3));
        hl.add_match(Position::new(15, 0), Position::new(15, 3));
        let vis = hl.visible_matches(4, 10);
        assert_eq!(vis.len(), 1);
        assert_eq!(vis[0].start.line, 5);
    }

    #[test]
    fn toggle_hlsearch() {
        let mut hl = SearchHighlights::new();
        assert!(!hl.hl_search);
        hl.toggle_hl_search();
        assert!(hl.hl_search);
        hl.toggle_hl_search();
        assert!(!hl.hl_search);
    }
}
