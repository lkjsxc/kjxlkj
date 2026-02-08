//! Cursor position type and operations.

/// A grapheme-based cursor position within a buffer.
///
/// Per /docs/spec/editing/cursor/README.md, the cursor is always
/// on a grapheme boundary: `(line, grapheme_offset)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorPosition {
    /// Zero-based line number.
    pub line: usize,
    /// Zero-based grapheme offset within the line.
    pub grapheme_offset: usize,
    /// Desired display column for vertical movement (sticky column).
    /// This preserves the column when moving through lines of varying length.
    pub desired_col: Option<usize>,
}

impl CursorPosition {
    /// Create a cursor at (0, 0).
    pub fn zero() -> Self {
        Self {
            line: 0,
            grapheme_offset: 0,
            desired_col: None,
        }
    }

    /// Create a cursor at a specific position.
    pub fn new(line: usize, grapheme_offset: usize) -> Self {
        Self {
            line,
            grapheme_offset,
            desired_col: None,
        }
    }

    /// Create with a desired column.
    pub fn with_desired_col(mut self, col: usize) -> Self {
        self.desired_col = Some(col);
        self
    }

    /// Clamp the cursor to end-exclusive range for normal mode.
    ///
    /// For a line with `g` graphemes (g > 0): valid offsets are 0..g-1.
    /// For an empty line: only offset 0 is valid.
    pub fn clamp_exclusive(&mut self, grapheme_count: usize) {
        if grapheme_count == 0 {
            self.grapheme_offset = 0;
        } else {
            self.grapheme_offset =
                self.grapheme_offset.min(grapheme_count - 1);
        }
    }

    /// Clamp the cursor to end-inclusive range for insert mode.
    ///
    /// Valid offsets are 0..g (cursor can be past last grapheme).
    pub fn clamp_inclusive(&mut self, grapheme_count: usize) {
        self.grapheme_offset = self.grapheme_offset.min(grapheme_count);
    }

    /// Clamp line to valid range.
    pub fn clamp_line(&mut self, line_count: usize) {
        if line_count == 0 {
            self.line = 0;
        } else {
            self.line = self.line.min(line_count - 1);
        }
    }

    /// Clear the desired column (for horizontal movements).
    pub fn clear_desired_col(&mut self) {
        self.desired_col = None;
    }

    /// Update the desired column from the current position.
    pub fn update_desired_col(&mut self, display_col: usize) {
        self.desired_col = Some(display_col);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_exclusive_normal() {
        let mut c = CursorPosition::new(0, 5);
        c.clamp_exclusive(3); // line has 3 graphemes
        assert_eq!(c.grapheme_offset, 2);
    }

    #[test]
    fn clamp_exclusive_empty_line() {
        let mut c = CursorPosition::new(0, 5);
        c.clamp_exclusive(0);
        assert_eq!(c.grapheme_offset, 0);
    }

    #[test]
    fn clamp_inclusive() {
        let mut c = CursorPosition::new(0, 5);
        c.clamp_inclusive(3); // can be at 3 (after last)
        assert_eq!(c.grapheme_offset, 3);
    }

    #[test]
    fn desired_col_sticky() {
        let c = CursorPosition::new(0, 10)
            .with_desired_col(20);
        assert_eq!(c.desired_col, Some(20));
    }
}
