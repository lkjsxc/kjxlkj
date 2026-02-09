use kjxlkj_core_ui::Style;

/// A single cell in the terminal display grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub grapheme: String,
    pub width: u8,
    pub style: Style,
    pub is_wide_continuation: bool,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            grapheme: " ".to_string(),
            width: 1,
            style: Style::default(),
            is_wide_continuation: false,
        }
    }

    pub fn with_char(c: char, style: Style) -> Self {
        let width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1) as u8;
        Self {
            grapheme: c.to_string(),
            width,
            style,
            is_wide_continuation: false,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}

/// 2D grid of cells for rendering.
#[derive(Debug, Clone)]
pub struct CellGrid {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

impl CellGrid {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            cells: vec![Cell::empty(); size],
            width,
            height,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, col: u16, row: u16) -> &Cell {
        let idx = (row as usize) * (self.width as usize) + (col as usize);
        &self.cells[idx]
    }

    pub fn set(&mut self, col: u16, row: u16, cell: Cell) {
        let idx = (row as usize) * (self.width as usize) + (col as usize);
        if idx < self.cells.len() {
            self.cells[idx] = cell;
        }
    }

    pub fn set_str(&mut self, col: u16, row: u16, s: &str, style: Style) {
        let mut c = col;
        for ch in s.chars() {
            if c >= self.width {
                break;
            }
            let w = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u8;
            self.set(
                c,
                row,
                Cell {
                    grapheme: ch.to_string(),
                    width: w,
                    style,
                    is_wide_continuation: false,
                },
            );
            if w == 2 && c + 1 < self.width {
                self.set(
                    c + 1,
                    row,
                    Cell {
                        grapheme: String::new(),
                        width: 0,
                        style,
                        is_wide_continuation: true,
                    },
                );
            }
            c += w as u16;
        }
    }

    /// Compare two grids and return list of dirty cells.
    pub fn diff(&self, other: &CellGrid) -> Vec<(u16, u16)> {
        let mut dirty = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get(col, row) != other.get(col, row) {
                    dirty.push((col, row));
                }
            }
        }
        dirty
    }
}
