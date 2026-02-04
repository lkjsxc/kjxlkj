//! Selection types for visual mode.

use crate::Position;
use serde::{Deserialize, Serialize};

/// The kind of selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SelectionKind {
    /// Character-wise selection.
    #[default]
    Char,
    /// Line-wise selection.
    Line,
    /// Block (rectangular) selection.
    Block,
}

/// A selection range in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// The anchor position (where selection started).
    pub anchor: Position,
    /// The cursor position (where selection ends).
    pub cursor: Position,
    /// The kind of selection.
    pub kind: SelectionKind,
}

impl Selection {
    /// Create a new selection.
    pub fn new(anchor: Position, cursor: Position, kind: SelectionKind) -> Self {
        Self {
            anchor,
            cursor,
            kind,
        }
    }

    /// Create a character selection at a position.
    pub fn char_at(pos: Position) -> Self {
        Self::new(pos, pos, SelectionKind::Char)
    }

    /// Get the start position (minimum of anchor and cursor).
    pub fn start(&self) -> Position {
        std::cmp::min(self.anchor, self.cursor)
    }

    /// Get the end position (maximum of anchor and cursor).
    pub fn end(&self) -> Position {
        std::cmp::max(self.anchor, self.cursor)
    }

    /// Check if the selection is empty (anchor == cursor).
    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }

    /// Swap anchor and cursor.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::char_at(Position::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_start_end() {
        let sel = Selection::new(
            Position::new(2, 5),
            Position::new(1, 3),
            SelectionKind::Char,
        );
        assert_eq!(sel.start(), Position::new(1, 3));
        assert_eq!(sel.end(), Position::new(2, 5));
    }

    #[test]
    fn selection_is_empty() {
        let sel = Selection::char_at(Position::new(1, 1));
        assert!(sel.is_empty());
    }

    #[test]
    fn selection_not_empty() {
        let sel = Selection::new(
            Position::new(0, 0),
            Position::new(0, 5),
            SelectionKind::Char,
        );
        assert!(!sel.is_empty());
    }

    #[test]
    fn selection_swap() {
        let mut sel = Selection::new(
            Position::new(0, 0),
            Position::new(1, 5),
            SelectionKind::Char,
        );
        sel.swap();
        assert_eq!(sel.anchor, Position::new(1, 5));
        assert_eq!(sel.cursor, Position::new(0, 0));
    }

    #[test]
    fn selection_line_kind() {
        let sel = Selection::new(
            Position::new(0, 0),
            Position::new(2, 0),
            SelectionKind::Line,
        );
        assert_eq!(sel.kind, SelectionKind::Line);
    }

    #[test]
    fn selection_block_kind() {
        let sel = Selection::new(
            Position::new(0, 5),
            Position::new(3, 10),
            SelectionKind::Block,
        );
        assert_eq!(sel.kind, SelectionKind::Block);
    }

    #[test]
    fn selection_kind_default() {
        assert_eq!(SelectionKind::default(), SelectionKind::Char);
    }

    #[test]
    fn selection_default() {
        let sel = Selection::default();
        assert!(sel.is_empty());
        assert_eq!(sel.kind, SelectionKind::Char);
        assert_eq!(sel.anchor, Position::new(0, 0));
    }

    #[test]
    fn selection_start_end_same_line() {
        let sel = Selection::new(
            Position::new(5, 10),
            Position::new(5, 2),
            SelectionKind::Char,
        );
        assert_eq!(sel.start(), Position::new(5, 2));
        assert_eq!(sel.end(), Position::new(5, 10));
    }

    #[test]
    fn selection_clone() {
        let sel = Selection::char_at(Position::new(3, 4));
        let cloned = sel.clone();
        assert_eq!(sel, cloned);
    }

    #[test]
    fn selection_debug_format() {
        let sel = Selection::char_at(Position::new(1, 2));
        let debug = format!("{:?}", sel);
        assert!(debug.contains("anchor"));
        assert!(debug.contains("cursor"));
    }
}
