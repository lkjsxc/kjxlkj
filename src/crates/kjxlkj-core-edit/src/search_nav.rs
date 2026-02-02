//! Search state navigation helpers.

use crate::search_types::SearchMatch;

/// Finds the match at or after a position.
pub fn find_at_or_after(matches: &[SearchMatch], line: usize, col: usize) -> Option<usize> {
    matches
        .iter()
        .position(|m| m.line > line || (m.line == line && m.start >= col))
}

/// Finds the match before a position.
pub fn find_before(matches: &[SearchMatch], line: usize, col: usize) -> Option<usize> {
    matches
        .iter()
        .rposition(|m| m.line < line || (m.line == line && m.end <= col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_at_or_after() {
        let matches = vec![
            SearchMatch::new(0, 0, 1),
            SearchMatch::new(1, 5, 6),
            SearchMatch::new(2, 10, 11),
        ];
        assert_eq!(find_at_or_after(&matches, 0, 5), Some(1));
        assert_eq!(find_at_or_after(&matches, 1, 6), Some(2));
    }

    #[test]
    fn test_find_before() {
        let matches = vec![
            SearchMatch::new(0, 0, 1),
            SearchMatch::new(1, 5, 6),
            SearchMatch::new(2, 10, 11),
        ];
        assert_eq!(find_before(&matches, 2, 10), Some(1));
        assert_eq!(find_before(&matches, 1, 5), Some(0));
    }
}
