//! Cursor representation.

use crate::Position;
use serde::{Deserialize, Serialize};

/// A cursor in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Current position.
    pub position: Position,
    /// Preferred column for vertical movement.
    pub preferred_col: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at a position.
    pub fn new(position: Position) -> Self {
        Self {
            position,
            preferred_col: None,
        }
    }

    /// Create a cursor at (0, 0).
    pub fn origin() -> Self {
        Self::new(Position::default())
    }

    /// Set the preferred column.
    pub fn with_preferred_col(mut self, col: usize) -> Self {
        self.preferred_col = Some(col);
        self
    }

    /// Clear the preferred column.
    pub fn clear_preferred_col(&mut self) {
        self.preferred_col = None;
    }

    /// Get the line number.
    pub fn line(&self) -> usize {
        self.position.line
    }

    /// Get the column number.
    pub fn col(&self) -> usize {
        self.position.col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_creation() {
        let c = Cursor::new(Position::new(1, 2));
        assert_eq!(c.line(), 1);
        assert_eq!(c.col(), 2);
        assert!(c.preferred_col.is_none());
    }

    #[test]
    fn cursor_preferred_col() {
        let c = Cursor::origin().with_preferred_col(10);
        assert_eq!(c.preferred_col, Some(10));
    }
}
