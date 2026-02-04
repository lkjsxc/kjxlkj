//! Position types for cursor and text locations.

use serde::{Deserialize, Serialize};

/// A position in a text buffer (0-indexed line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed, in grapheme clusters).
    pub col: usize,
}

impl Position {
    /// Create a new position.
    pub const fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Create a position at the start of a line.
    pub const fn line_start(line: usize) -> Self {
        Self { line, col: 0 }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_ordering() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 5);
        let p3 = Position::new(1, 0);
        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p1 < p3);
    }

    #[test]
    fn position_equality() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(1, 2);
        assert_eq!(p1, p2);
    }

    #[test]
    fn position_default() {
        let p = Position::default();
        assert_eq!(p.line, 0);
        assert_eq!(p.col, 0);
    }

    #[test]
    fn position_line_start() {
        let p = Position::line_start(5);
        assert_eq!(p.line, 5);
        assert_eq!(p.col, 0);
    }

    #[test]
    fn position_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Position::new(1, 2));
        assert!(set.contains(&Position::new(1, 2)));
    }

    #[test]
    fn position_clone() {
        let p = Position::new(5, 10);
        let cloned = p.clone();
        assert_eq!(p, cloned);
    }

    #[test]
    fn position_debug_format() {
        let p = Position::new(3, 7);
        let debug = format!("{:?}", p);
        assert!(debug.contains("3"));
        assert!(debug.contains("7"));
    }

    #[test]
    fn position_ord_same_line() {
        let p1 = Position::new(5, 2);
        let p2 = Position::new(5, 8);
        assert!(p1 < p2);
    }

    #[test]
    fn position_ord_different_lines() {
        let p1 = Position::new(3, 100);
        let p2 = Position::new(4, 0);
        assert!(p1 < p2);
    }

    #[test]
    fn position_hash_different() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Position::new(1, 2));
        set.insert(Position::new(2, 1));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn position_copy_trait() {
        let p = Position::new(1, 2);
        let copied: Position = p;
        assert_eq!(p, copied);
    }

    #[test]
    fn position_partial_eq() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 0);
        let p3 = Position::new(0, 1);
        assert!(p1 == p2);
        assert!(p1 != p3);
    }

    #[test]
    fn position_origin() {
        let p = Position::default();
        assert_eq!(p.line, 0);
        assert_eq!(p.col, 0);
    }

    #[test]
    fn position_display_format() {
        let p = Position::new(5, 10);
        let display = format!("{:?}", p);
        assert!(display.contains("5"));
        assert!(display.contains("10"));
    }

    #[test]
    fn position_max_values() {
        let p = Position::new(usize::MAX, usize::MAX);
        assert_eq!(p.line, usize::MAX);
        assert_eq!(p.col, usize::MAX);
    }

    #[test]
    fn position_equal_to_self() {
        let p = Position::new(7, 3);
        assert_eq!(p, p);
    }

    #[test]
    fn position_not_equal() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(3, 4);
        assert_ne!(p1, p2);
    }

    #[test]
    fn position_cloneable() {
        let p = Position::new(5, 6);
        let cloned = p.clone();
        assert_eq!(p, cloned);
    }

    #[test]
    fn position_copyable() {
        let p = Position::new(7, 8);
        let copied: Position = p;
        assert_eq!(p, copied);
    }

    #[test]
    fn position_col_zero() {
        let p = Position::line_start(10);
        assert_eq!(p.col, 0);
    }

    #[test]
    fn position_serialize() {
        let p = Position::new(1, 2);
        let display = format!("{:?}", p);
        assert!(display.contains("line"));
    }
}
