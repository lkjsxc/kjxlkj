//! Range type for text regions.

use serde::{Deserialize, Serialize};

use crate::Position;

/// A range of text in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
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

    /// Create a range covering a single position.
    pub fn point(pos: Position) -> Self {
        Self {
            start: pos,
            end: Position::new(pos.line, pos.column + 1),
        }
    }

    /// Create a range covering a single line.
    pub fn line(line: usize) -> Self {
        Self {
            start: Position::new(line, 0),
            end: Position::new(line + 1, 0),
        }
    }

    /// Create a range from line/column values.
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

    /// Check if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Check if the range spans multiple lines.
    pub fn is_multiline(&self) -> bool {
        self.start.line != self.end.line
    }

    /// Check if a position is contained in this range.
    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Get range in normalized form (start <= end).
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_point() {
        let r = Range::point(Position::new(0, 5));
        assert_eq!(r.start.column, 5);
        assert_eq!(r.end.column, 6);
    }

    #[test]
    fn range_line() {
        let r = Range::line(3);
        assert_eq!(r.start.line, 3);
        assert_eq!(r.start.column, 0);
        assert_eq!(r.end.line, 4);
    }

    #[test]
    fn range_is_empty() {
        let empty = Range::new(Position::new(0, 5), Position::new(0, 5));
        assert!(empty.is_empty());

        let not_empty = Range::new(Position::new(0, 5), Position::new(0, 6));
        assert!(!not_empty.is_empty());
    }

    #[test]
    fn range_contains() {
        let r = Range::from_coords(0, 5, 0, 10);
        assert!(r.contains(Position::new(0, 5)));
        assert!(r.contains(Position::new(0, 9)));
        assert!(!r.contains(Position::new(0, 10)));
        assert!(!r.contains(Position::new(0, 4)));
    }

    #[test]
    fn range_normalized() {
        let r = Range::new(Position::new(1, 0), Position::new(0, 0));
        let n = r.normalized();
        assert_eq!(n.start, Position::new(0, 0));
        assert_eq!(n.end, Position::new(1, 0));
    }
}
