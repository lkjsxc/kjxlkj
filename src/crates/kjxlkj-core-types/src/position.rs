//! Position types for cursor and text locations.

use serde::{Deserialize, Serialize};

/// A position in a text buffer (0-indexed line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed, grapheme cluster based).
    pub col: usize,
}

impl Position {
    /// Create a new position.
    #[inline]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Position at the origin (0, 0).
    pub const ORIGIN: Self = Self { line: 0, col: 0 };
}

impl From<(usize, usize)> for Position {
    fn from((line, col): (usize, usize)) -> Self {
        Self { line, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.col, 10);
    }

    #[test]
    fn test_position_origin() {
        assert_eq!(Position::ORIGIN, Position::new(0, 0));
    }

    #[test]
    fn test_position_from_tuple() {
        let pos: Position = (3, 7).into();
        assert_eq!(pos, Position::new(3, 7));
    }
}
