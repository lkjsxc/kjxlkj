//! Buffer position (line + column).

use serde::{Deserialize, Serialize};

/// A position in a text buffer.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Position {
    /// Zero-based line index.
    pub line: usize,
    /// Zero-based column index (character offset within the line).
    pub col: usize,
}

impl Position {
    /// Create a new position.
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Start of the buffer.
    pub fn zero() -> Self {
        Self { line: 0, col: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_ordering() {
        let a = Position::new(0, 5);
        let b = Position::new(1, 0);
        let c = Position::new(1, 3);
        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn test_position_equality() {
        let a = Position::new(10, 20);
        let b = Position::new(10, 20);
        assert_eq!(a, b);
    }
}
