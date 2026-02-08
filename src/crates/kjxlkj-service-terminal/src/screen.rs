//! Terminal screen buffer.

use kjxlkj_core_types::{Cell, CellAttrs, Color, TerminalId};

/// Internal screen buffer for a terminal emulator.
pub struct ScreenBuffer {
    /// Terminal ID.
    pub id: TerminalId,
    /// Buffer width.
    pub cols: u16,
    /// Buffer height.
    pub rows: u16,
    /// Cell grid (row-major).
    cells: Vec<Cell>,
    /// Cursor position (col, row).
    pub cursor_col: u16,
    pub cursor_row: u16,
    /// Current attributes for new text.
    pub current_fg: Color,
    pub current_bg: Color,
    pub current_attrs: CellAttrs,
    /// Window title.
    pub title: String,
}

impl ScreenBuffer {
    /// Create a new screen buffer.
    pub fn new(
        id: TerminalId,
        cols: u16,
        rows: u16,
    ) -> Self {
        let count = cols as usize * rows as usize;
        Self {
            id,
            cols,
            rows,
            cells: vec![Cell::default(); count],
            cursor_col: 0,
            cursor_row: 0,
            current_fg: Color::Default,
            current_bg: Color::Default,
            current_attrs: CellAttrs::empty(),
            title: String::new(),
        }
    }

    /// Put a character at the cursor position.
    pub fn put_char(&mut self, ch: char) {
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row >= self.rows {
                self.scroll_up(1);
                self.cursor_row = self.rows - 1;
            }
        }

        let idx = self.cursor_row as usize * self.cols as usize
            + self.cursor_col as usize;
        if idx < self.cells.len() {
            self.cells[idx].grapheme =
                compact_str::CompactString::from(
                    ch.to_string().as_str(),
                );
            self.cells[idx].width = 1;
            self.cells[idx].fg = self.current_fg;
            self.cells[idx].bg = self.current_bg;
            self.cells[idx].attrs = self.current_attrs;
        }
        self.cursor_col += 1;
    }

    /// Scroll up by n lines.
    pub fn scroll_up(&mut self, n: u16) {
        let row_len = self.cols as usize;
        let n = n as usize;
        if n >= self.rows as usize {
            self.clear();
            return;
        }
        // Rotate rows up: first n rows go to end, then clear them.
        let shift = n * row_len;
        self.cells.rotate_left(shift);
        let total = self.cells.len();
        let clear_start = total - shift;
        for cell in &mut self.cells[clear_start..] {
            *cell = Cell::default();
        }
    }

    /// Clear the entire screen.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    /// Move cursor to position.
    pub fn move_cursor(&mut self, col: u16, row: u16) {
        self.cursor_col = col.min(self.cols - 1);
        self.cursor_row = row.min(self.rows - 1);
    }

    /// Get the cells as a slice.
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Resize the screen buffer.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let count = cols as usize * rows as usize;
        self.cells = vec![Cell::default(); count];
        self.cols = cols;
        self.rows = rows;
        self.cursor_col = self.cursor_col.min(cols - 1);
        self.cursor_row = self.cursor_row.min(rows - 1);
    }

    /// Execute a newline.
    pub fn newline(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;
        if self.cursor_row >= self.rows {
            self.scroll_up(1);
            self.cursor_row = self.rows - 1;
        }
    }

    /// Carriage return.
    pub fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    /// Backspace.
    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_char_advances_cursor() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.put_char('A');
        assert_eq!(buf.cursor_col, 1);
    }

    #[test]
    fn newline_wraps() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.cursor_row = 23;
        buf.newline();
        assert_eq!(buf.cursor_row, 23);
    }

    #[test]
    fn clear_resets() {
        let mut buf =
            ScreenBuffer::new(TerminalId(1), 80, 24);
        buf.put_char('X');
        buf.clear();
        assert_eq!(buf.cursor_col, 0);
        assert_eq!(buf.cursor_row, 0);
    }
}
