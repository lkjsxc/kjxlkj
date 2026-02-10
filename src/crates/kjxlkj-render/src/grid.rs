//! Cell grid: 2D array of render cells.

/// A single cell in the render grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub grapheme: String,
    pub width: u8,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub is_wide_continuation: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            grapheme: " ".to_string(),
            width: 1,
            fg: (255, 255, 255),
            bg: (0, 0, 0),
            bold: false,
            italic: false,
            underline: false,
            is_wide_continuation: false,
        }
    }
}

/// A 2D cell grid.
pub struct CellGrid {
    pub cells: Vec<Vec<Cell>>,
    pub rows: usize,
    pub cols: usize,
}

impl CellGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Cell::default(); cols]; rows];
        Self { cells, rows, cols }
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = cell;
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }

    /// Clear the entire grid.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::default();
            }
        }
    }
}
