//! Search highlight types.
//!
//! Type definitions for search highlighting.

use kjxlkj_core_types::Range;

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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Position;

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
    fn test_search_highlight_range() {
        let start = Position::new(1, 5);
        let end = Position::new(1, 10);
        let range = Range::new(start, end);
        let hl = SearchHighlight::new(range);
        assert_eq!(hl.range.start, start);
        assert_eq!(hl.range.end, end);
    }
}
