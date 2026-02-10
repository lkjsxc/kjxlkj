//! Cell types for rendering grid.

use crate::Style;

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// Grapheme cluster to display.
    pub grapheme: String,
    /// Display width (0 for continuation cells).
    pub width: u8,
    /// Cell style.
    pub style: Style,
    /// Whether this is a continuation cell for a wide character.
    pub is_wide_continuation: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            grapheme: " ".to_string(),
            width: 1,
            style: Style::default(),
            is_wide_continuation: false,
        }
    }
}

impl Cell {
    /// Create a new cell with a grapheme.
    pub fn new(grapheme: String, width: u8, style: Style) -> Self {
        Self {
            grapheme,
            width,
            style,
            is_wide_continuation: false,
        }
    }

    /// Create an empty cell.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a continuation cell for wide characters.
    pub fn continuation(style: Style) -> Self {
        Self {
            grapheme: String::new(),
            width: 0,
            style,
            is_wide_continuation: true,
        }
    }

    /// Create a padding cell.
    pub fn padding(style: Style) -> Self {
        Self {
            grapheme: " ".to_string(),
            width: 1,
            style,
            is_wide_continuation: false,
        }
    }
}

/// A grid of cells.
#[derive(Debug, Clone)]
pub struct CellGrid {
    /// Grid width.
    pub width: u16,
    /// Grid height.
    pub height: u16,
    /// Cell data (row-major order).
    cells: Vec<Cell>,
}

impl CellGrid {
    /// Create a new grid filled with empty cells.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::empty(); size],
        }
    }

    /// Get cell at position.
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells.get(idx)
        } else {
            None
        }
    }

    /// Get mutable cell at position.
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells.get_mut(idx)
        } else {
            None
        }
    }

    /// Set cell at position.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(c) = self.get_mut(x, y) {
            *c = cell;
        }
    }

    /// Fill a row with a cell.
    pub fn fill_row(&mut self, y: u16, cell: &Cell) {
        for x in 0..self.width {
            self.set(x, y, cell.clone());
        }
    }

    /// Clear the grid.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::empty();
        }
    }

    /// Iterate over rows.
    pub fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks(self.width as usize)
    }
}
