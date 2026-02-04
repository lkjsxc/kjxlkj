//! Position types for buffer locations.

use serde::{Deserialize, Serialize};

/// A byte position in the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position(pub usize);

impl Position {
    /// Create a new position.
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Get the byte offset.
    pub fn offset(&self) -> usize {
        self.0
    }
}

impl From<usize> for Position {
    fn from(offset: usize) -> Self {
        Self(offset)
    }
}

impl From<Position> for usize {
    fn from(pos: Position) -> Self {
        pos.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_ordering() {
        let a = Position::new(5);
        let b = Position::new(10);
        assert!(a < b);
    }
}
