//! Terminal state and parsing.

/// Terminal cell.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    /// Character.
    pub c: char,
    /// Foreground color.
    pub fg: u8,
    /// Background color.
    pub bg: u8,
    /// Attributes (bold, underline, etc).
    pub attrs: u8,
}

/// Terminal state.
pub struct Terminal {
    /// Width in columns.
    cols: u16,
    /// Height in rows.
    rows: u16,
    /// Cell buffer.
    cells: Vec<Cell>,
    /// Cursor column.
    cursor_col: u16,
    /// Cursor row.
    cursor_row: u16,
    /// Scroll region top.
    scroll_top: u16,
    /// Scroll region bottom.
    scroll_bottom: u16,
}

impl Terminal {
    /// Creates a new terminal.
    pub fn new(cols: u16, rows: u16) -> Self {
        let size = (cols as usize) * (rows as usize);
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); size],
            cursor_col: 0,
            cursor_row: 0,
            scroll_top: 0,
            scroll_bottom: rows.saturating_sub(1),
        }
    }

    /// Returns the size.
    pub fn size(&self) -> (u16, u16) {
        (self.cols, self.rows)
    }

    /// Returns the cursor position.
    pub fn cursor(&self) -> (u16, u16) {
        (self.cursor_col, self.cursor_row)
    }

    /// Gets a cell.
    pub fn cell(&self, col: u16, row: u16) -> Option<&Cell> {
        if col < self.cols && row < self.rows {
            Some(&self.cells[(row as usize) * (self.cols as usize) + (col as usize)])
        } else {
            None
        }
    }

    /// Gets a mutable cell.
    pub fn cell_mut(&mut self, col: u16, row: u16) -> Option<&mut Cell> {
        if col < self.cols && row < self.rows {
            Some(&mut self.cells[(row as usize) * (self.cols as usize) + (col as usize)])
        } else {
            None
        }
    }

    /// Writes a character at the cursor position.
    pub fn write_char(&mut self, c: char) {
        if let Some(cell) = self.cell_mut(self.cursor_col, self.cursor_row) {
            cell.c = c;
        }
        self.cursor_col += 1;
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row > self.scroll_bottom {
                self.scroll_up();
                self.cursor_row = self.scroll_bottom;
            }
        }
    }

    /// Moves cursor.
    pub fn move_cursor(&mut self, col: u16, row: u16) {
        self.cursor_col = col.min(self.cols.saturating_sub(1));
        self.cursor_row = row.min(self.rows.saturating_sub(1));
    }

    /// Scrolls the terminal up.
    fn scroll_up(&mut self) {
        let cols = self.cols as usize;
        let top = self.scroll_top as usize;
        let bottom = self.scroll_bottom as usize;

        for row in top..bottom {
            let src_start = (row + 1) * cols;
            let dst_start = row * cols;
            for i in 0..cols {
                self.cells[dst_start + i] = self.cells[src_start + i].clone();
            }
        }

        // Clear bottom line
        let start = bottom * cols;
        for i in 0..cols {
            self.cells[start + i] = Cell::default();
        }
    }

    /// Clears the terminal.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    /// Resizes the terminal.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let new_size = (cols as usize) * (rows as usize);
        let mut new_cells = vec![Cell::default(); new_size];

        let min_cols = self.cols.min(cols) as usize;
        let min_rows = self.rows.min(rows) as usize;

        for row in 0..min_rows {
            for col in 0..min_cols {
                let old_idx = row * (self.cols as usize) + col;
                let new_idx = row * (cols as usize) + col;
                new_cells[new_idx] = self.cells[old_idx].clone();
            }
        }

        self.cols = cols;
        self.rows = rows;
        self.cells = new_cells;
        self.scroll_bottom = rows.saturating_sub(1);
        self.cursor_col = self.cursor_col.min(cols.saturating_sub(1));
        self.cursor_row = self.cursor_row.min(rows.saturating_sub(1));
    }
}
