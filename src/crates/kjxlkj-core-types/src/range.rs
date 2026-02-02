//! Range types for text regions.

use crate::Position;
use serde::{Deserialize, Serialize};

/// A range of text defined by start and end positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive).
    pub start: Position,
    /// End position (exclusive).
    pub end: Position,
}

impl Range {
    /// Creates a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Creates a range from line/column coordinates.
    pub fn from_coords(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }

    /// Creates a single-point range (cursor position).
    pub fn point(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns true if the range contains the given position.
    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Returns a normalized range (start <= end).
    pub fn normalized(&self) -> Self {
        if self.start <= self.end {
            *self
        } else {
            Self {
                start: self.end,
                end: self.start,
            }
        }
    }

    /// Returns the number of lines spanned.
    pub fn line_count(&self) -> usize {
        let norm = self.normalized();
        norm.end.line - norm.start.line + 1
    }
}
