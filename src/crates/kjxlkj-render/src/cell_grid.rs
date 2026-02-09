//! Cell grid for rendering.

use kjxlkj_core_types::{Cell, CellAttrs, Color};

/// A 2D grid of cells for rendering.
#[derive(Debug, Clone)]
pub struct CellGrid {
    /// Width in columns.
    pub cols: u16,
    /// Height in rows.
    pub rows: u16,
    /// Flattened cell data (row-major).
    cells: Vec<Cell>,
}

impl CellGrid {
    /// Create a new cell grid filled with empty cells.
    pub fn new(cols: u16, rows: u16) -> Self {
        let count = cols as usize * rows as usize;
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); count],
        }
    }

    /// Get a cell at (col, row).
    pub fn get(&self, col: u16, row: u16) -> &Cell {
        let idx = row as usize * self.cols as usize + col as usize;
        &self.cells[idx]
    }

    /// Get a mutable cell at (col, row).
    pub fn get_mut(&mut self, col: u16, row: u16) -> &mut Cell {
        let idx = row as usize * self.cols as usize + col as usize;
        &mut self.cells[idx]
    }

    /// Set a cell at (col, row).
    pub fn set(&mut self, col: u16, row: u16, cell: Cell) {
        if col < self.cols && row < self.rows {
            let idx =
                row as usize * self.cols as usize + col as usize;
            self.cells[idx] = cell;
        }
    }

    /// Write a string at (col, row) with given colors/attrs.
    pub fn write_str(
        &mut self,
        col: u16,
        row: u16,
        text: &str,
        fg: Color,
        bg: Color,
        attrs: CellAttrs,
    ) {
        let mut c = col;
        for ch in text.chars() {
            if c >= self.cols {
                break;
            }
            let mut cell = Cell::default();
            cell.grapheme =
                compact_str::CompactString::from(
                    ch.to_string().as_str(),
                );
            cell.width = 1;
            cell.fg = fg;
            cell.bg = bg;
            cell.attrs = attrs;
            self.set(c, row, cell);
            c += 1;
        }
    }

    /// Clear the entire grid.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Clear a single row.
    pub fn clear_row(&mut self, row: u16) {
        if row >= self.rows {
            return;
        }
        let start = row as usize * self.cols as usize;
        let end = start + self.cols as usize;
        for cell in &mut self.cells[start..end] {
            *cell = Cell::default();
        }
    }

    /// Fill a row with a repeated character.
    pub fn fill_row(
        &mut self,
        row: u16,
        ch: char,
        fg: Color,
        bg: Color,
    ) {
        if row >= self.rows {
            return;
        }
        let s = ch.to_string();
        for col in 0..self.cols {
            let mut cell = Cell::default();
            cell.grapheme =
                compact_str::CompactString::from(s.as_str());
            cell.width = 1;
            cell.fg = fg;
            cell.bg = bg;
            self.set(col, row, cell);
        }
    }

    /// Total number of cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid() {
        let grid = CellGrid::new(80, 24);
        assert_eq!(grid.cell_count(), 80 * 24);
    }

    #[test]
    fn write_and_read() {
        let mut grid = CellGrid::new(10, 5);
        grid.write_str(
            0,
            0,
            "hi",
            Color::Default,
            Color::Default,
            CellAttrs::empty(),
        );
        let cell = grid.get(0, 0);
        assert_eq!(cell.grapheme.as_str(), "h");
    }
}
