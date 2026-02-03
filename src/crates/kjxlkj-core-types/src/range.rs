//! Range types for text regions.

use serde::{Deserialize, Serialize};

use crate::Position;

/// A range in a buffer (start inclusive, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// Create a range from line/column coordinates.
    pub fn from_coords(
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }

    /// Create an empty range at a position.
    pub fn empty(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    /// Check if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if the range spans multiple lines.
    pub fn is_multiline(&self) -> bool {
        self.start.line != self.end.line
    }

    /// Get the normalized range (start <= end).
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

    /// Extend range to include another position.
    pub fn extend_to(&self, pos: Position) -> Self {
        let norm = self.normalized();
        Self {
            start: if pos < norm.start { pos } else { norm.start },
            end: if pos >= norm.end {
                Position::new(pos.line, pos.col + 1)
            } else {
                norm.end
            },
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::empty(Position::default())
    }
}
