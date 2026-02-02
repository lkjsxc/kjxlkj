//! Double-buffered rendering for efficient screen updates.

use crossterm::style::{Color, Attribute, Attributes};

/// A single cell on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The character in this cell.
    pub ch: char,
    /// Foreground color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text attributes.
    pub attrs: Attributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
            attrs: Attributes::default(),
        }
    }
}

impl Cell {
    /// Creates a new cell with a character.
    pub fn new(ch: char) -> Self {
        Self { ch, ..Default::default() }
    }

    /// Sets the foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Adds a text attribute.
    pub fn attr(mut self, attr: Attribute) -> Self {
        self.attrs.set(attr);
        self
    }
}

/// A buffer of cells representing the screen.
#[derive(Debug, Clone)]
pub struct ScreenBuffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

impl ScreenBuffer {
    /// Creates a new buffer.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            cells: vec![Cell::default(); size],
            width,
            height,
        }
    }

    /// Resizes the buffer.
    pub fn resize(&mut self, width: u16, height: u16) {
        let size = (width as usize) * (height as usize);
        self.cells.resize(size, Cell::default());
        self.width = width;
        self.height = height;
        self.clear();
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    /// Gets a cell at position.
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells.get(idx)
        } else {
            None
        }
    }

    /// Sets a cell at position.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx] = cell;
        }
    }

    /// Writes a string at position.
    pub fn write_str(&mut self, x: u16, y: u16, s: &str, fg: Color, bg: Color) {
        let mut col = x;
        for ch in s.chars() {
            if col >= self.width {
                break;
            }
            self.set(col, y, Cell { ch, fg, bg, attrs: Attributes::default() });
            col += 1;
        }
    }

    /// Returns width.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns height.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Iterates over cells that differ from another buffer.
    pub fn diff<'a>(&'a self, other: &'a ScreenBuffer) -> impl Iterator<Item = (u16, u16, &'a Cell)> {
        self.cells.iter().enumerate().filter_map(move |(i, cell)| {
            let x = (i % self.width as usize) as u16;
            let y = (i / self.width as usize) as u16;
            let other_cell = other.get(x, y)?;
            if cell != other_cell {
                Some((x, y, cell))
            } else {
                None
            }
        })
    }
}
