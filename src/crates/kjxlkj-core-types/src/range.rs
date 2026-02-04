//! Text range (start + end position).

use crate::Position;
use serde::{Deserialize, Serialize};

/// A range in a text buffer (start inclusive, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive).
    pub start: Position,
    /// End position (exclusive).
    pub end: Position,
}

impl Range {
    /// Create a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a range spanning a single position.
    pub fn point(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    /// Check if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Normalize so start <= end.
    pub fn normalized(self) -> Self {
        if self.start <= self.end {
            self
        } else {
            Self {
                start: self.end,
                end: self.start,
            }
        }
    }

    /// Check if a position is within this range.
    pub fn contains(&self, pos: Position) -> bool {
        let norm = self.normalized();
        pos >= norm.start && pos < norm.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_normalized() {
        let r = Range::new(Position::new(5, 0), Position::new(2, 0));
        let n = r.normalized();
        assert_eq!(n.start, Position::new(2, 0));
        assert_eq!(n.end, Position::new(5, 0));
    }

    #[test]
    fn test_range_contains() {
        let r = Range::new(Position::new(1, 0), Position::new(3, 0));
        assert!(r.contains(Position::new(2, 5)));
        assert!(!r.contains(Position::new(0, 0)));
        assert!(!r.contains(Position::new(3, 0)));
    }
}
