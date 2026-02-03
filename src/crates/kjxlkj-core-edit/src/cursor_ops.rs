//! Cursor movement operations.

use kjxlkj_core_types::Cursor;

/// Trait for cursor movement operations.
pub trait CursorOps {
    /// Returns the line count.
    fn line_count(&self) -> usize;
    /// Returns the length of a line.
    fn line_len(&self, line: usize) -> Option<usize>;
    /// Returns the cursor.
    fn cursor(&self) -> Cursor;
    /// Returns a mutable reference to the cursor.
    fn cursor_mut(&mut self) -> &mut Cursor;

    /// Moves the cursor left.
    fn move_left(&mut self) {
        let cursor = self.cursor_mut();
        if cursor.position.col > 0 {
            cursor.position.col -= 1;
        }
        cursor.clear_preferred_col();
    }

    /// Moves the cursor right.
    fn move_right(&mut self) {
        let cursor = self.cursor();
        let line_len = self
            .line_len(cursor.position.line as usize)
            .unwrap_or(0);
        let max_col = if line_len > 0 { line_len - 1 } else { 0 };
        let cursor = self.cursor_mut();
        if (cursor.position.col as usize) < max_col {
            cursor.position.col += 1;
        }
        cursor.clear_preferred_col();
    }

    /// Moves the cursor up.
    fn move_up(&mut self) {
        let cur = self.cursor();
        if cur.position.line > 0 {
            let pref = cur.preferred_col.unwrap_or(cur.position.col);
            let new_line = cur.position.line - 1;
            let new_len = self.line_len(new_line as usize).unwrap_or(0);
            let max_col = if new_len > 0 { new_len - 1 } else { 0 };
            let cursor = self.cursor_mut();
            cursor.position.line = new_line;
            cursor.position.col = pref.min(max_col as u32);
            cursor.set_preferred_col(pref);
        }
    }

    /// Moves the cursor down.
    fn move_down(&mut self) {
        let line_count = self.line_count();
        let cur = self.cursor();
        if (cur.position.line as usize) < line_count.saturating_sub(1) {
            let pref = cur.preferred_col.unwrap_or(cur.position.col);
            let new_line = cur.position.line + 1;
            let new_len = self.line_len(new_line as usize).unwrap_or(0);
            let max_col = if new_len > 0 { new_len - 1 } else { 0 };
            let cursor = self.cursor_mut();
            cursor.position.line = new_line;
            cursor.position.col = pref.min(max_col as u32);
            cursor.set_preferred_col(pref);
        }
    }

    /// Moves the cursor to the start of the line.
    fn move_line_start(&mut self) {
        self.cursor_mut().position.col = 0;
        self.cursor_mut().clear_preferred_col();
    }

    /// Moves the cursor to the end of the line.
    fn move_line_end(&mut self) {
        let cursor = self.cursor();
        let line_len = self
            .line_len(cursor.position.line as usize)
            .unwrap_or(0);
        let max_col = if line_len > 0 { line_len - 1 } else { 0 };
        self.cursor_mut().position.col = max_col as u32;
        self.cursor_mut().clear_preferred_col();
    }
}

use crate::Buffer;

impl CursorOps for Buffer {
    fn line_count(&self) -> usize {
        Buffer::line_count(self)
    }

    fn line_len(&self, line: usize) -> Option<usize> {
        Buffer::line_len(self, line)
    }

    fn cursor(&self) -> Cursor {
        Buffer::cursor(self)
    }

    fn cursor_mut(&mut self) -> &mut Cursor {
        Buffer::cursor_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, LineCol};

    #[test]
    fn move_down_clamps() {
        let mut buf = Buffer::from_content(
            BufferId::new(1),
            "test".to_string(),
            "short\nlongerline",
        );
        buf.cursor_mut().position = LineCol::new(1, 9);
        buf.move_up();
        assert_eq!(buf.cursor().position.col, 4);
    }
}
