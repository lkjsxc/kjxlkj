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
}
