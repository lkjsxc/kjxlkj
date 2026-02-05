//! Range in a text buffer.

use crate::Position;
use serde::{Deserialize, Serialize};

/// A range spanning two positions (start inclusive, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive).
    pub start: Position,
    /// End position (exclusive).
    pub end: Position,
}

impl Range {
    /// Create a new range.
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Check if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if a position is within this range.
    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }
}
