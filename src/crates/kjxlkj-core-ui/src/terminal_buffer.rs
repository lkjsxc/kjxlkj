//! Terminal cell and buffer types.

use super::TermSize;

/// A terminal cell.
#[derive(Debug, Clone)]
pub struct TermCell {
    /// Character.
    pub ch: char,
    /// Foreground color.
    pub fg: Option<u8>,
    /// Background color.
    pub bg: Option<u8>,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
}

impl Default for TermCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// Terminal buffer.
#[derive(Debug, Clone)]
pub struct TermBuffer {
    cells: Vec<Vec<TermCell>>,
    size: TermSize,
}

impl TermBuffer {
    /// Creates a new terminal buffer.
    pub fn new(size: TermSize) -> Self {
        let cells = (0..size.rows)
            .map(|_| (0..size.cols).map(|_| TermCell::default()).collect())
            .collect();
        Self { cells, size }
    }

    /// Returns the size.
    pub fn size(&self) -> TermSize {
        self.size
    }

    /// Gets a cell.
    pub fn cell(&self, row: usize, col: usize) -> Option<&TermCell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    /// Sets a cell.
    pub fn set_cell(&mut self, row: usize, col: usize, cell: TermCell) {
        if let Some(r) = self.cells.get_mut(row) {
            if let Some(c) = r.get_mut(col) {
                *c = cell;
            }
        }
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = TermCell::default();
            }
        }
    }

    /// Resizes the buffer.
    pub fn resize(&mut self, new_size: TermSize) {
        self.cells = (0..new_size.rows)
            .map(|_| (0..new_size.cols).map(|_| TermCell::default()).collect())
            .collect();
        self.size = new_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_buffer() {
        let mut buf = TermBuffer::new(TermSize::new(80, 24));
        assert_eq!(buf.size().cols, 80);

        let cell = TermCell { ch: 'X', ..Default::default() };
        buf.set_cell(5, 10, cell);
        assert_eq!(buf.cell(5, 10).unwrap().ch, 'X');
    }

    #[test]
    fn test_term_buffer_clear() {
        let mut buf = TermBuffer::new(TermSize::new(80, 24));
        buf.set_cell(0, 0, TermCell { ch: 'A', ..Default::default() });
        buf.clear();
        assert_eq!(buf.cell(0, 0).unwrap().ch, ' ');
    }

    #[test]
    fn test_term_buffer_resize() {
        let mut buf = TermBuffer::new(TermSize::new(80, 24));
        buf.resize(TermSize::new(120, 40));
        assert_eq!(buf.size().cols, 120);
    }
}
