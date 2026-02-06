//! Visual block operation model for block insert/append/change.

use kjxlkj_core_types::Position;

/// A visual block selection defined by two corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockSelection {
    pub start: Position,
    pub end: Position,
}

impl BlockSelection {
    pub fn new(start: Position, end: Position) -> Self { Self { start, end } }

    pub fn top_left(&self) -> Position {
        Position::new(self.start.line.min(self.end.line), self.start.col.min(self.end.col))
    }
    pub fn bottom_right(&self) -> Position {
        Position::new(self.start.line.max(self.end.line), self.start.col.max(self.end.col))
    }
    pub fn line_range(&self) -> (usize, usize) {
        (self.start.line.min(self.end.line), self.start.line.max(self.end.line))
    }
    pub fn col_range(&self) -> (usize, usize) {
        (self.start.col.min(self.end.col), self.start.col.max(self.end.col))
    }
    pub fn height(&self) -> usize { let (t, b) = self.line_range(); b - t + 1 }
    pub fn width(&self) -> usize { let (l, r) = self.col_range(); r - l + 1 }
}

/// Block operation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockOp { Insert, Append, Change, Delete }

/// A pending block edit operation.
#[derive(Debug, Clone)]
pub struct BlockEdit {
    pub selection: BlockSelection,
    pub op: BlockOp,
    pub text: String,
}

impl BlockEdit {
    pub fn new(sel: BlockSelection, op: BlockOp) -> Self {
        Self { selection: sel, op, text: String::new() }
    }

    /// Generate per-line edit positions for applying the block edit.
    pub fn line_edits(&self) -> Vec<(usize, usize, &str)> {
        let (top, bot) = self.selection.line_range();
        let (left, right) = self.selection.col_range();
        (top..=bot).map(|line| {
            match self.op {
                BlockOp::Insert => (line, left, self.text.as_str()),
                BlockOp::Append => (line, right + 1, self.text.as_str()),
                BlockOp::Change | BlockOp::Delete => (line, left, self.text.as_str()),
            }
        }).collect()
    }

    /// Columns to delete per line for Change/Delete operations.
    pub fn delete_range(&self) -> Option<(usize, usize)> {
        match self.op {
            BlockOp::Change | BlockOp::Delete => Some(self.selection.col_range()),
            _ => None,
        }
    }
}

/// Extend a block selection with `$` (to end-of-line per row).
pub fn extend_to_eol(sel: &BlockSelection, line_lengths: &[usize]) -> Vec<(usize, usize)> {
    let (top, bot) = sel.line_range();
    let left = sel.col_range().0;
    (top..=bot).map(|line| {
        let len = line_lengths.get(line).copied().unwrap_or(0);
        (left, len.saturating_sub(1).max(left))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pos(l: usize, c: usize) -> Position { Position::new(l, c) }

    #[test]
    fn block_dimensions() {
        let b = BlockSelection::new(pos(1, 5), pos(4, 10));
        assert_eq!(b.height(), 4);
        assert_eq!(b.width(), 6);
        assert_eq!(b.top_left(), pos(1, 5));
        assert_eq!(b.bottom_right(), pos(4, 10));
    }

    #[test]
    fn block_inverted_corners() {
        let b = BlockSelection::new(pos(5, 10), pos(2, 3));
        assert_eq!(b.top_left(), pos(2, 3));
        assert_eq!(b.bottom_right(), pos(5, 10));
        assert_eq!(b.height(), 4);
    }

    #[test]
    fn insert_line_edits() {
        let sel = BlockSelection::new(pos(1, 5), pos(3, 5));
        let mut edit = BlockEdit::new(sel, BlockOp::Insert);
        edit.text = "// ".into();
        let edits = edit.line_edits();
        assert_eq!(edits.len(), 3);
        assert_eq!(edits[0], (1, 5, "// "));
    }

    #[test]
    fn append_line_edits() {
        let sel = BlockSelection::new(pos(0, 3), pos(2, 5));
        let mut edit = BlockEdit::new(sel, BlockOp::Append);
        edit.text = ";".into();
        let edits = edit.line_edits();
        assert_eq!(edits[0].1, 6); // right + 1
    }

    #[test]
    fn delete_range() {
        let sel = BlockSelection::new(pos(0, 2), pos(3, 8));
        let edit = BlockEdit::new(sel, BlockOp::Delete);
        assert_eq!(edit.delete_range(), Some((2, 8)));
        let edit2 = BlockEdit::new(sel, BlockOp::Insert);
        assert!(edit2.delete_range().is_none());
    }

    #[test]
    fn extend_eol() {
        let sel = BlockSelection::new(pos(0, 2), pos(2, 5));
        let lens = vec![10, 8, 15];
        let ranges = extend_to_eol(&sel, &lens);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], (2, 9));
        assert_eq!(ranges[1], (2, 7));
        assert_eq!(ranges[2], (2, 14));
    }

    #[test]
    fn block_ops_distinct() {
        assert_ne!(BlockOp::Insert, BlockOp::Append);
        assert_ne!(BlockOp::Change, BlockOp::Delete);
    }
}
