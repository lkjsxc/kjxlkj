//! Cursor position tracking.
//!
//! Cursor is in grapheme-space on a line.

/// Cursor position within a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    /// Zero-based line number.
    pub line: usize,
    /// Zero-based grapheme offset on the line.
    pub col: usize,
    /// Desired column for vertical movement.
    pub desired_col: usize,
}

impl Cursor {
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            line,
            col,
            desired_col: col,
        }
    }

    /// Clamp cursor to valid buffer bounds.
    pub fn clamp(
        &mut self,
        line_count: usize,
        line_grapheme_count: usize,
    ) {
        if line_count == 0 {
            self.line = 0;
            self.col = 0;
            return;
        }
        if self.line >= line_count {
            self.line = line_count - 1;
        }
        let max_col = if line_grapheme_count > 0 {
            line_grapheme_count - 1
        } else {
            0
        };
        if self.col > max_col {
            self.col = max_col;
        }
    }

    /// Clamp for insert mode (allows cursor past last char).
    pub fn clamp_insert(
        &mut self,
        line_count: usize,
        line_grapheme_count: usize,
    ) {
        if line_count == 0 {
            self.line = 0;
            self.col = 0;
            return;
        }
        if self.line >= line_count {
            self.line = line_count - 1;
        }
        if self.col > line_grapheme_count {
            self.col = line_grapheme_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_clamp_basic() {
        let mut c = Cursor::new(100, 50);
        c.clamp(10, 5);
        assert_eq!(c.line, 9);
        assert_eq!(c.col, 4);
    }

    #[test]
    fn cursor_clamp_empty() {
        let mut c = Cursor::new(5, 5);
        c.clamp(0, 0);
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 0);
    }

    #[test]
    fn cursor_insert_allows_past_end() {
        let mut c = Cursor::new(0, 5);
        c.clamp_insert(1, 5);
        assert_eq!(c.col, 5); // one past last char
    }
}
