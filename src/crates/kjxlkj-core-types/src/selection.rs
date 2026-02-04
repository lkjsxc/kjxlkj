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

    #[test]
    fn test_selection_kind_default() {
        assert_eq!(SelectionKind::default(), SelectionKind::Char);
    }

    #[test]
    fn test_selection_char_wise() {
        let sel = Selection::char_wise(Position::new(0, 0), Position::new(1, 1));
        assert_eq!(sel.kind, SelectionKind::Char);
    }

    #[test]
    fn test_selection_line_wise() {
        let sel = Selection::line_wise(Position::new(0, 0), Position::new(1, 1));
        assert_eq!(sel.kind, SelectionKind::Line);
    }

    #[test]
    fn test_selection_block_wise() {
        let sel = Selection::block_wise(Position::new(0, 0), Position::new(1, 1));
        assert_eq!(sel.kind, SelectionKind::Block);
    }

    #[test]
    fn test_selection_start_same_line() {
        let sel = Selection::char_wise(Position::new(0, 5), Position::new(0, 2));
        assert_eq!(sel.start(), Position::new(0, 2));
    }

    #[test]
    fn test_selection_end_same_line() {
        let sel = Selection::char_wise(Position::new(0, 2), Position::new(0, 5));
        assert_eq!(sel.end(), Position::new(0, 5));
    }

    #[test]
    fn test_selection_new() {
        let sel = Selection::new(Position::new(1, 2), Position::new(3, 4), SelectionKind::Block);
        assert_eq!(sel.anchor, Position::new(1, 2));
        assert_eq!(sel.cursor, Position::new(3, 4));
        assert_eq!(sel.kind, SelectionKind::Block);
    }

    #[test]
    fn test_selection_clone() {
        let sel = Selection::char_wise(Position::new(0, 0), Position::new(1, 1));
        let cloned = sel.clone();
        assert_eq!(sel, cloned);
    }

    #[test]
    fn test_selection_kind_equality() {
        assert_eq!(SelectionKind::Char, SelectionKind::Char);
        assert_ne!(SelectionKind::Char, SelectionKind::Line);
    }

    #[test]
    fn test_selection_line_range_reversed() {
        let sel = Selection::line_wise(Position::new(7, 0), Position::new(2, 0));
        assert_eq!(sel.line_range(), (2, 7));
    }

    #[test]
    fn test_selection_debug() {
        let sel = Selection::char_wise(Position::new(0, 0), Position::new(1, 1));
        let debug = format!("{:?}", sel);
        assert!(debug.contains("Selection"));
    }

    #[test]
    fn test_selection_kind_debug() {
        let kind = SelectionKind::Block;
        let debug = format!("{:?}", kind);
        assert!(debug.contains("Block"));
    }

    #[test]
    fn test_selection_kind_clone() {
        let kind = SelectionKind::Line;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_selection_same_position() {
        let sel = Selection::char_wise(Position::new(1, 1), Position::new(1, 1));
        assert_eq!(sel.start(), sel.end());
    }

    #[test]
    fn test_selection_swap_preserves_kind() {
        let mut sel = Selection::block_wise(Position::new(0, 0), Position::new(5, 5));
        sel.swap();
        assert_eq!(sel.kind, SelectionKind::Block);
    }
}

