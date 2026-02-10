//! Screen erase and line operations.

use crate::cell::Cell;
use crate::screen::Screen;

impl Screen {
    /// Erase in display.
    pub fn erase_display(&mut self, mode: usize) {
        match mode {
            0 => self.erase_below(),
            1 => self.erase_above(),
            2 | 3 => self.erase_all(),
            _ => {}
        }
    }

    fn erase_below(&mut self) {
        for c in self.cursor_col..self.cols {
            self.cells[self.cursor_row][c] = Cell::default();
        }
        for r in (self.cursor_row + 1)..self.rows {
            for c in 0..self.cols {
                self.cells[r][c] = Cell::default();
            }
        }
    }

    fn erase_above(&mut self) {
        for r in 0..self.cursor_row {
            for c in 0..self.cols {
                self.cells[r][c] = Cell::default();
            }
        }
        for c in 0..=self.cursor_col.min(self.cols.saturating_sub(1)) {
            self.cells[self.cursor_row][c] = Cell::default();
        }
    }

    fn erase_all(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.cells[r][c] = Cell::default();
            }
        }
    }

    /// Erase in line.
    pub fn erase_line(&mut self, mode: usize) {
        let row = self.cursor_row;
        if row >= self.rows {
            return;
        }
        match mode {
            0 => {
                for c in self.cursor_col..self.cols {
                    self.cells[row][c] = Cell::default();
                }
            }
            1 => {
                for c in 0..=self.cursor_col.min(self.cols.saturating_sub(1)) {
                    self.cells[row][c] = Cell::default();
                }
            }
            2 => {
                for c in 0..self.cols {
                    self.cells[row][c] = Cell::default();
                }
            }
            _ => {}
        }
    }

    /// Resize the grid.
    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
        self.cells.resize_with(rows, || vec![Cell::default(); cols]);
        for row in &mut self.cells {
            row.resize_with(cols, Cell::default);
        }
        self.scroll_bottom = rows.saturating_sub(1);
        self.cursor_row = self.cursor_row.min(rows.saturating_sub(1));
        self.cursor_col = self.cursor_col.min(cols.saturating_sub(1));
    }

    /// Enter alternate screen buffer.
    pub fn enter_alternate(&mut self) {
        if !self.in_alternate {
            self.saved_primary = Some(self.cells.clone());
            self.erase_display(2);
            self.cursor_row = 0;
            self.cursor_col = 0;
            self.in_alternate = true;
        }
    }

    /// Exit alternate screen buffer, restoring primary.
    pub fn exit_alternate(&mut self) {
        if self.in_alternate {
            if let Some(primary) = self.saved_primary.take() {
                self.cells = primary;
            }
            self.cursor_row = 0;
            self.cursor_col = 0;
            self.in_alternate = false;
        }
    }

    /// Read a cell at (row, col).
    pub fn cell(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }
}
