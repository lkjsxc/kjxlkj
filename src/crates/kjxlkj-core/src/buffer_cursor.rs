//! Cursor movement methods for Buffer.

use crate::buffer::Buffer;

impl Buffer {
    /// Moves cursor left.
    pub fn cursor_left(&mut self) {
        let col = self.cursor_col();
        if col > 0 {
            self.move_cursor(self.cursor_line(), col - 1);
        }
    }

    /// Moves cursor right.
    pub fn cursor_right(&mut self) {
        let line = self.cursor_line();
        let col = self.cursor_col();
        let max_col = self.line_len(line).saturating_sub(1);
        if col < max_col {
            self.move_cursor(line, col + 1);
        }
    }

    /// Moves cursor up.
    pub fn cursor_up(&mut self) {
        let line = self.cursor_line();
        if line > 0 {
            self.move_cursor(line - 1, self.cursor_col());
        }
    }

    /// Moves cursor down.
    pub fn cursor_down(&mut self) {
        let line = self.cursor_line();
        if line < self.line_count().saturating_sub(1) {
            self.move_cursor(line + 1, self.cursor_col());
        }
    }
}
