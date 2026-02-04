//! Selection types for visual mode.

use crate::Position;
use serde::{Deserialize, Serialize};

/// The kind of visual selection.
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

/// A text selection (visual mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Selection {
    /// The anchor position (where selection started).
    pub anchor: Position,
    /// The cursor position (current end of selection).
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

    /// Create a character-wise selection.
    pub fn char_wise(anchor: Position, cursor: Position) -> Self {
        Self::new(anchor, cursor, SelectionKind::Char)
    }

    /// Create a line-wise selection.
    pub fn line_wise(anchor: Position, cursor: Position) -> Self {
        Self::new(anchor, cursor, SelectionKind::Line)
    }

    /// Create a block selection.
    pub fn block_wise(anchor: Position, cursor: Position) -> Self {
        Self::new(anchor, cursor, SelectionKind::Block)
    }

    /// Get the start position (minimum).
    pub fn start(&self) -> Position {
        if self.anchor.line < self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.col <= self.cursor.col)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    /// Get the end position (maximum).
    pub fn end(&self) -> Position {
        if self.anchor.line > self.cursor.line
            || (self.anchor.line == self.cursor.line && self.anchor.col >= self.cursor.col)
        {
            self.anchor
        } else {
            self.cursor
        }
    }

    /// Get the line range (inclusive).
    pub fn line_range(&self) -> (usize, usize) {
        let start = self.start();
        let end = self.end();
        (start.line, end.line)
    }

    /// Swap anchor and cursor (toggle selection end).
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_start_end() {
        let sel = Selection::char_wise(Position::new(5, 10), Position::new(3, 5));
        assert_eq!(sel.start(), Position::new(3, 5));
        assert_eq!(sel.end(), Position::new(5, 10));
    }

    #[test]
    fn test_selection_swap() {
        let mut sel = Selection::char_wise(Position::new(0, 0), Position::new(5, 5));
        sel.swap();
        assert_eq!(sel.anchor, Position::new(5, 5));
        assert_eq!(sel.cursor, Position::new(0, 0));
    }

    #[test]
    fn test_selection_line_range() {
        let sel = Selection::line_wise(Position::new(2, 0), Position::new(7, 0));
        assert_eq!(sel.line_range(), (2, 7));
    }
}
