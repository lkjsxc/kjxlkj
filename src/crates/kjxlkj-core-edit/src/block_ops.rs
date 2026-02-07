//! Visual block (column) editing operations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};
use serde::{Deserialize, Serialize};

/// A rectangular block selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockSelection {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl BlockSelection {
    pub fn new(top_left: Position, bottom_right: Position) -> Self {
        Self { top_left, bottom_right }
    }

    /// Number of lines in the selection.
    pub fn height(&self) -> usize {
        self.bottom_right.line - self.top_left.line + 1
    }

    /// Column width of the selection.
    pub fn width(&self) -> usize {
        if self.bottom_right.col >= self.top_left.col {
            self.bottom_right.col - self.top_left.col + 1
        } else {
            0
        }
    }
}

/// Kind of block editing operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockOp {
    Insert,
    Append,
    Change,
    Delete,
}

/// A block edit specification.
#[derive(Debug, Clone)]
pub struct BlockEdit {
    pub op: BlockOp,
    pub lines: Vec<(usize, Range)>,
}

/// Build per-line edit ranges for a block selection.
pub fn build_block_edits(sel: &BlockSelection, buffer: &TextBuffer) -> Vec<(usize, Range)> {
    let mut edits = Vec::new();
    for line_idx in sel.top_left.line..=sel.bottom_right.line {
        let line_len = buffer.line_len(line_idx);
        let start_col = sel.top_left.col.min(line_len);
        let end_col = (sel.bottom_right.col + 1).min(line_len);
        edits.push((
            line_idx,
            Range::new(
                Position::new(line_idx, start_col),
                Position::new(line_idx, end_col),
            ),
        ));
    }
    edits
}

/// Extend the block selection to end-of-line on every selected line.
pub fn extend_to_eol(sel: &mut BlockSelection, buffer: &TextBuffer) {
    let mut max_col = sel.bottom_right.col;
    for line_idx in sel.top_left.line..=sel.bottom_right.line {
        let ll = buffer.line_len(line_idx);
        if ll > max_col {
            max_col = ll;
        }
    }
    sel.bottom_right.col = max_col;
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn block_dimensions() {
        let sel = BlockSelection::new(Position::new(1, 2), Position::new(4, 5));
        assert_eq!(sel.height(), 4);
        assert_eq!(sel.width(), 4);
    }

    #[test]
    fn build_edits() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "abcdef\nghijkl\nmnopqr\n");
        let sel = BlockSelection::new(Position::new(0, 1), Position::new(2, 3));
        let edits = build_block_edits(&sel, &buf);
        assert_eq!(edits.len(), 3);
        assert_eq!(edits[0].1.start.col, 1);
        assert_eq!(edits[0].1.end.col, 4);
    }

    #[test]
    fn extend_eol() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "short\nlonger line\nhi\n");
        let mut sel = BlockSelection::new(Position::new(0, 0), Position::new(2, 2));
        extend_to_eol(&mut sel, &buf);
        assert!(sel.bottom_right.col >= 11);
    }
}
