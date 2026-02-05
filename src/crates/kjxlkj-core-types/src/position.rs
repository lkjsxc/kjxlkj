//! Position type.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A position in a text buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed).
    pub column: usize,
}

impl Position {
    /// Create a new position.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Origin position (0, 0).
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::origin()
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            ord => ord,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_ordering() {
        let p1 = Position::new(0, 5);
        let p2 = Position::new(0, 10);
        let p3 = Position::new(1, 0);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p1 < p3);
    }

    #[test]
    fn position_origin() {
        let p = Position::origin();
        assert_eq!(p.line, 0);
        assert_eq!(p.column, 0);
    }
}
