//! Cursor representation.

use serde::{Deserialize, Serialize};

use crate::position::LineCol;

/// A cursor position in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// Line-column position (0-indexed).
    pub position: LineCol,
    /// Preferred column for vertical navigation.
    pub preferred_col: Option<u32>,
}

impl Cursor {
    /// Creates a new cursor at the given position.
    pub fn new(line: u32, col: u32) -> Self {
        Self {
            position: LineCol { line, col },
            preferred_col: None,
        }
    }

    /// Creates a cursor at origin (0, 0).
    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Updates the cursor position.
    pub fn move_to(&mut self, line: u32, col: u32) {
        self.position = LineCol { line, col };
    }

    /// Sets the preferred column for vertical navigation.
    pub fn set_preferred_col(&mut self, col: u32) {
        self.preferred_col = Some(col);
    }

    /// Clears the preferred column.
    pub fn clear_preferred_col(&mut self) {
        self.preferred_col = None;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::origin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_creation() {
        let c = Cursor::new(5, 10);
        assert_eq!(c.position.line, 5);
        assert_eq!(c.position.col, 10);
    }

    #[test]
    fn cursor_movement() {
        let mut c = Cursor::origin();
        c.move_to(3, 7);
        assert_eq!(c.position.line, 3);
        assert_eq!(c.position.col, 7);
    }
}
