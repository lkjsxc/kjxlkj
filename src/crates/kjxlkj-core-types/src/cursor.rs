//! Cursor state.

use crate::Position;
use serde::{Deserialize, Serialize};

/// Cursor state with position and optional selection anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// Current cursor position.
    pub pos: Position,
    /// Selection anchor (if in visual mode).
    pub anchor: Option<Position>,
}

impl Cursor {
    /// Create a cursor at a position with no selection.
    pub fn new(pos: Position) -> Self {
        Self { pos, anchor: None }
    }

    /// Create a cursor with a selection anchor.
    pub fn with_anchor(pos: Position, anchor: Position) -> Self {
        Self {
            pos,
            anchor: Some(anchor),
        }
    }

    /// Start a selection from current position.
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.pos);
    }

    /// Clear the selection.
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Get the selection range (normalized).
    pub fn selection_range(&self) -> Option<crate::Range> {
        self.anchor
            .map(|anchor| crate::Range::new(anchor, self.pos).normalized())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_selection() {
        let mut c = Cursor::new(Position::new(5, 10));
        c.start_selection();
        c.pos = Position::new(7, 5);
        let range = c.selection_range().unwrap();
        assert_eq!(range.start, Position::new(5, 10));
        assert_eq!(range.end, Position::new(7, 5));
    }
}
