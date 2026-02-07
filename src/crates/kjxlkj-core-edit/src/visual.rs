//! Visual mode selection types and operations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};
use serde::{Deserialize, Serialize};

/// The kind of visual selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisualKind {
    Char,
    Line,
    Block,
}

/// Represents a visual selection in the buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualSelection {
    pub kind: VisualKind,
    pub anchor: Position,
    pub cursor: Position,
}

impl VisualSelection {
    /// Create a new visual selection.
    pub fn new(kind: VisualKind, anchor: Position, cursor: Position) -> Self {
        Self {
            kind,
            anchor,
            cursor,
        }
    }

    /// Check whether the given position is within the selection.
    pub fn contains(&self, pos: Position) -> bool {
        match self.kind {
            VisualKind::Char => {
                let r = self.selected_range();
                pos >= r.start && pos < r.end
            }
            VisualKind::Line => {
                let (start_line, end_line) = self.line_bounds();
                pos.line >= start_line && pos.line <= end_line
            }
            VisualKind::Block => {
                let (start_line, end_line) = self.line_bounds();
                let (left, right) = self.block_cols();
                pos.line >= start_line && pos.line <= end_line && pos.col >= left && pos.col < right
            }
        }
    }

    /// Extract the selected text from the buffer.
    pub fn extract_selection(&self, buffer: &TextBuffer) -> Vec<String> {
        match self.kind {
            VisualKind::Char => {
                let r = self.selected_range();
                let mut result = Vec::new();
                for line_idx in r.start.line..=r.end.line {
                    let line = buffer.line(line_idx).unwrap_or_default();
                    let sc = if line_idx == r.start.line {
                        r.start.col
                    } else {
                        0
                    };
                    let ec = if line_idx == r.end.line {
                        r.end.col
                    } else {
                        line.len()
                    };
                    let ec = ec.min(line.len());
                    let sc = sc.min(ec);
                    result.push(line[sc..ec].to_string());
                }
                result
            }
            VisualKind::Line => {
                let (start_line, end_line) = self.line_bounds();
                let mut result = Vec::new();
                for l in start_line..=end_line {
                    result.push(buffer.line(l).unwrap_or_default());
                }
                result
            }
            VisualKind::Block => {
                let (start_line, end_line) = self.line_bounds();
                let (left, right) = self.block_cols();
                let mut result = Vec::new();
                for l in start_line..=end_line {
                    let line = buffer.line(l).unwrap_or_default();
                    let sc = left.min(line.len());
                    let ec = right.min(line.len());
                    result.push(line[sc..ec].to_string());
                }
                result
            }
        }
    }

    /// Swap anchor and cursor.
    pub fn swap_ends(&mut self) {
        std::mem::swap(&mut self.anchor, &mut self.cursor);
    }

    /// Return block column boundaries (left, right) with left < right.
    pub fn block_cols(&self) -> (usize, usize) {
        let left = self.anchor.col.min(self.cursor.col);
        let right = self.anchor.col.max(self.cursor.col) + 1;
        (left, right)
    }

    /// Switch to a different visual kind.
    pub fn switch_kind(&mut self, new_kind: VisualKind) {
        self.kind = new_kind;
    }

    /// Return the normalized range (start <= end) for the selection.
    pub fn selected_range(&self) -> Range {
        let (s, e) = if self.anchor <= self.cursor {
            (
                self.anchor,
                Position::new(self.cursor.line, self.cursor.col + 1),
            )
        } else {
            (
                self.cursor,
                Position::new(self.anchor.line, self.anchor.col + 1),
            )
        };
        match self.kind {
            VisualKind::Line => {
                let start_line = s.line;
                let end_line = e.line;
                Range::new(
                    Position::new(start_line, 0),
                    Position::new(end_line, usize::MAX),
                )
            }
            _ => Range::new(s, e),
        }
    }

    fn line_bounds(&self) -> (usize, usize) {
        let start = self.anchor.line.min(self.cursor.line);
        let end = self.anchor.line.max(self.cursor.line);
        (start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn char_selection() {
        let sel = VisualSelection::new(VisualKind::Char, Position::new(0, 2), Position::new(0, 5));
        assert!(sel.contains(Position::new(0, 3)));
        assert!(!sel.contains(Position::new(0, 6)));
    }

    #[test]
    fn swap_ends() {
        let mut sel =
            VisualSelection::new(VisualKind::Char, Position::new(0, 0), Position::new(0, 5));
        sel.swap_ends();
        assert_eq!(sel.anchor, Position::new(0, 5));
    }

    #[test]
    fn block_cols() {
        let sel = VisualSelection::new(VisualKind::Block, Position::new(0, 5), Position::new(2, 2));
        assert_eq!(sel.block_cols(), (2, 6));
    }

    #[test]
    fn extract_char() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "hello world");
        let sel = VisualSelection::new(VisualKind::Char, Position::new(0, 0), Position::new(0, 4));
        let lines = sel.extract_selection(&buf);
        assert_eq!(lines, vec!["hello"]);
    }
}
