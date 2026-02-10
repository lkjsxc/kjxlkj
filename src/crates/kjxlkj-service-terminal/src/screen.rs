//! Terminal screen model: grid of cells with cursor.

use crate::cell::{Cell, CellAttrs};

/// Terminal screen buffer (main or alternate).
pub struct Screen {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Vec<Cell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub cursor_visible: bool,
    pub attrs: CellAttrs,
    pub scroll_top: usize,
    pub scroll_bottom: usize,
    pub auto_wrap: bool,
    pub saved_cursor: Option<(usize, usize)>,
    pub title: String,
    /// Saved primary screen for alternate screen switching.
    pub(crate) saved_primary: Option<Vec<Vec<Cell>>>,
    /// Whether we are in alternate screen mode.
    pub in_alternate: bool,
}

impl Screen {
    pub fn new(cols: usize, rows: usize) -> Self {
        let cells = (0..rows).map(|_| vec![Cell::default(); cols]).collect();
        Self {
            cols,
            rows,
            cells,
            cursor_row: 0,
            cursor_col: 0,
            cursor_visible: true,
            attrs: CellAttrs::default(),
            scroll_top: 0,
            scroll_bottom: rows.saturating_sub(1),
            auto_wrap: true,
            saved_cursor: None,
            title: String::new(),
            saved_primary: None,
            in_alternate: false,
        }
    }

    /// Write a character at the current cursor position.
    pub fn write_char(&mut self, ch: char, width: u8) {
        if self.cursor_row >= self.rows || self.cursor_col >= self.cols {
            return;
        }
        if self.cursor_col + (width as usize) > self.cols && self.auto_wrap {
            self.cursor_col = 0;
            self.advance_row();
        }
        if self.cursor_row >= self.rows {
            return;
        }
        let cell = &mut self.cells[self.cursor_row][self.cursor_col];
        cell.ch = ch;
        cell.width = width;
        cell.is_wide_continuation = false;
        cell.attrs = self.attrs.clone();
        self.cursor_col += 1;
        if width == 2 && self.cursor_col < self.cols {
            let cont = &mut self.cells[self.cursor_row][self.cursor_col];
            cont.ch = ' ';
            cont.width = 0;
            cont.is_wide_continuation = true;
            cont.attrs = self.attrs.clone();
            self.cursor_col += 1;
        }
    }

    /// Advance cursor to next row, scrolling if needed.
    pub(crate) fn advance_row(&mut self) {
        if self.cursor_row == self.scroll_bottom {
            self.scroll_up(1);
        } else if self.cursor_row < self.rows - 1 {
            self.cursor_row += 1;
        }
    }

    /// Scroll up N rows in the scroll region.
    pub fn scroll_up(&mut self, n: usize) {
        for _ in 0..n {
            if self.scroll_top < self.scroll_bottom {
                self.cells.remove(self.scroll_top);
                let blank = vec![Cell::default(); self.cols];
                self.cells.insert(self.scroll_bottom, blank);
            }
        }
    }

    /// Scroll down N rows in the scroll region.
    pub fn scroll_down(&mut self, n: usize) {
        for _ in 0..n {
            if self.scroll_top < self.scroll_bottom && self.scroll_bottom < self.cells.len() {
                self.cells.remove(self.scroll_bottom);
                let blank = vec![Cell::default(); self.cols];
                self.cells.insert(self.scroll_top, blank);
            }
        }
    }

    /// Move cursor to position (0-based).
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.rows.saturating_sub(1));
        self.cursor_col = col.min(self.cols.saturating_sub(1));
    }

    /// Process a newline.
    pub fn newline(&mut self) {
        self.cursor_col = 0;
        self.advance_row();
    }

    /// Process a carriage return.
    pub fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    /// Process a backspace.
    pub fn backspace(&mut self) {
        self.cursor_col = self.cursor_col.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_char() {
        let mut s = Screen::new(80, 24);
        s.write_char('A', 1);
        assert_eq!(s.cells[0][0].ch, 'A');
        assert_eq!(s.cursor_col, 1);
    }

    #[test]
    fn test_erase_display() {
        let mut s = Screen::new(10, 5);
        s.write_char('X', 1);
        s.erase_display(2);
        assert_eq!(s.cells[0][0].ch, ' ');
    }

    #[test]
    fn test_wide_char() {
        let mut s = Screen::new(80, 24);
        s.write_char('あ', 2);
        assert_eq!(s.cells[0][0].ch, 'あ');
        assert_eq!(s.cells[0][0].width, 2);
        assert!(s.cells[0][1].is_wide_continuation);
        assert_eq!(s.cursor_col, 2);
    }
}
