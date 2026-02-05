//! Cursor types and semantics.

use serde::{Deserialize, Serialize};

use crate::Position;

/// A cursor position in a buffer.
///
/// The cursor represents the current editing position.
/// In end-exclusive modes (Normal/Visual), the cursor is on a character.
/// In end-inclusive modes (Insert), the cursor is between characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed, character offset).
    pub column: usize,
    /// Target column for vertical movement.
    pub target_column: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at the given position.
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            target_column: None,
        }
    }

    /// Create cursor at origin (0, 0).
    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Set the target column for vertical movement.
    pub fn with_target_column(mut self, target: usize) -> Self {
        self.target_column = Some(target);
        self
    }

    /// Clear the target column.
    pub fn clear_target_column(&mut self) {
        self.target_column = None;
    }

    /// Get the effective target column for vertical movement.
    pub fn effective_target(&self) -> usize {
        self.target_column.unwrap_or(self.column)
    }

    /// Convert to a Position.
    pub fn to_position(self) -> Position {
        Position::new(self.line, self.column)
    }

    /// Clamp cursor to valid range for end-exclusive mode.
    ///
    /// For a line of length N, valid columns are 0..N-1 for N > 0.
    /// For empty lines, only column 0 is valid.
    pub fn clamp_end_exclusive(mut self, line_len: usize) -> Self {
        if line_len == 0 {
            self.column = 0;
        } else {
            self.column = self.column.min(line_len - 1);
        }
        self
    }

    /// Clamp cursor to valid range for end-inclusive mode.
    ///
    /// For a line of length N, valid columns are 0..N.
    pub fn clamp_end_inclusive(mut self, line_len: usize) -> Self {
        self.column = self.column.min(line_len);
        self
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::origin()
    }
}

impl From<Position> for Cursor {
    fn from(pos: Position) -> Self {
        Self::new(pos.line, pos.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_origin() {
        let c = Cursor::origin();
        assert_eq!(c.line, 0);
        assert_eq!(c.column, 0);
    }

    #[test]
    fn cursor_clamp_end_exclusive() {
        let c = Cursor::new(0, 10);
        let clamped = c.clamp_end_exclusive(5);
        assert_eq!(clamped.column, 4);

        let c = Cursor::new(0, 10);
        let clamped = c.clamp_end_exclusive(0);
        assert_eq!(clamped.column, 0);
    }

    #[test]
    fn cursor_clamp_end_inclusive() {
        let c = Cursor::new(0, 10);
        let clamped = c.clamp_end_inclusive(5);
        assert_eq!(clamped.column, 5);
    }

    #[test]
    fn cursor_target_column() {
        let c = Cursor::new(0, 5).with_target_column(10);
        assert_eq!(c.effective_target(), 10);

        let c = Cursor::new(0, 5);
        assert_eq!(c.effective_target(), 5);
    }
}
