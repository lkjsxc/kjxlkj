//! Selection types for visual mode.

use crate::Cursor;
use serde::{Deserialize, Serialize};

/// Kind of selection.
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

/// A text selection with anchor and cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// The anchor point (where selection started).
    pub anchor: Cursor,
    /// The cursor point (where selection ends).
    pub cursor: Cursor,
    /// The kind of selection.
    pub kind: SelectionKind,
}

impl Selection {
    /// Create a new selection.
    pub fn new(anchor: Cursor, cursor: Cursor, kind: SelectionKind) -> Self {
        Self { anchor, cursor, kind }
    }

    /// Create a charwise selection.
    pub fn charwise(anchor: Cursor, cursor: Cursor) -> Self {
        Self::new(anchor, cursor, SelectionKind::Char)
    }

    /// Create a linewise selection.
    pub fn linewise(anchor: Cursor, cursor: Cursor) -> Self {
        Self::new(anchor, cursor, SelectionKind::Line)
    }

    /// Create a blockwise selection.
    pub fn blockwise(anchor: Cursor, cursor: Cursor) -> Self {
        Self::new(anchor, cursor, SelectionKind::Block)
    }

    /// Get the start position (min of anchor and cursor).
    pub fn start(&self) -> Cursor {
        if (self.anchor.line, self.anchor.col) <= (self.cursor.line, self.cursor.col) {
            self.anchor
        } else {
            self.cursor
        }
    }

    /// Get the end position (max of anchor and cursor).
    pub fn end(&self) -> Cursor {
        if (self.anchor.line, self.anchor.col) <= (self.cursor.line, self.cursor.col) {
            self.cursor
        } else {
            self.anchor
        }
    }

    /// Swap anchor and cursor.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_start_end() {
        let sel = Selection::charwise(Cursor::new(5, 3), Cursor::new(2, 1));
        assert_eq!(sel.start(), Cursor::new(2, 1));
        assert_eq!(sel.end(), Cursor::new(5, 3));
    }

    #[test]
    fn selection_swap() {
        let mut sel = Selection::charwise(Cursor::new(1, 0), Cursor::new(2, 0));
        sel.swap();
        assert_eq!(sel.anchor, Cursor::new(2, 0));
        assert_eq!(sel.cursor, Cursor::new(1, 0));
    }
}
