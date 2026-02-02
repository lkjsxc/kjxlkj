//! Screen buffer.

use crate::Style;
use kjxlkj_core_ui::Dimensions;

/// A cell in the screen buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// Character content.
    pub content: String,
    /// Style.
    pub style: Style,
}

impl Cell {
    /// Creates a new cell.
    pub fn new(content: impl Into<String>, style: Style) -> Self {
        Self {
            content: content.into(),
            style,
        }
    }

    /// Creates an empty cell.
    pub fn empty() -> Self {
        Self {
            content: String::from(" "),
            style: Style::default(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}

/// Screen buffer for double buffering.
#[derive(Debug, Clone)]
pub struct ScreenBuffer {
    /// Cells.
    cells: Vec<Cell>,
    /// Dimensions.
    dimensions: Dimensions,
}

impl ScreenBuffer {
    /// Creates a new screen buffer.
    pub fn new(dimensions: Dimensions) -> Self {
        let size = dimensions.width as usize * dimensions.height as usize;
        Self {
            cells: vec![Cell::empty(); size],
            dimensions,
        }
    }

    /// Returns the dimensions.
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    /// Gets a cell.
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.dimensions.width && y < self.dimensions.height {
            let idx = y as usize * self.dimensions.width as usize + x as usize;
            self.cells.get(idx)
        } else {
            None
        }
    }

    /// Sets a cell.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.dimensions.width && y < self.dimensions.height {
            let idx = y as usize * self.dimensions.width as usize + x as usize;
            self.cells[idx] = cell;
        }
    }

    /// Sets a character.
    pub fn set_char(&mut self, x: u16, y: u16, c: char, style: Style) {
        self.set(x, y, Cell::new(c.to_string(), style));
    }

    /// Sets a string starting at position.
    pub fn set_string(&mut self, x: u16, y: u16, s: &str, style: Style) {
        let mut col = x;
        for c in s.chars() {
            if col >= self.dimensions.width {
                break;
            }
            self.set_char(col, y, c, style);
            col += 1;
        }
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::empty();
        }
    }

    /// Resizes the buffer.
    pub fn resize(&mut self, dimensions: Dimensions) {
        if dimensions != self.dimensions {
            let size = dimensions.width as usize * dimensions.height as usize;
            self.cells = vec![Cell::empty(); size];
            self.dimensions = dimensions;
        }
    }
}
