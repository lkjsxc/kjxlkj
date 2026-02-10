//! Terminal screen buffer.

/// Terminal screen.
#[derive(Debug)]
pub struct Screen {
    /// Width.
    width: u16,
    /// Height.
    height: u16,
    /// Cells.
    cells: Vec<ScreenCell>,
    /// Cursor X.
    cursor_x: u16,
    /// Cursor Y.
    cursor_y: u16,
}

/// Screen cell.
#[derive(Debug, Clone, Default)]
pub struct ScreenCell {
    /// Character.
    pub ch: char,
    /// Foreground color.
    pub fg: Option<u8>,
    /// Background color.
    pub bg: Option<u8>,
}

impl Screen {
    /// Create a new screen.
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![ScreenCell::default(); (width as usize) * (height as usize)];
        Self {
            width,
            height,
            cells,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Resize the screen.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.cells = vec![ScreenCell::default(); (width as usize) * (height as usize)];
        self.cursor_x = self.cursor_x.min(width.saturating_sub(1));
        self.cursor_y = self.cursor_y.min(height.saturating_sub(1));
    }

    /// Get cell at position.
    pub fn cell(&self, x: u16, y: u16) -> Option<&ScreenCell> {
        if x < self.width && y < self.height {
            self.cells.get((y as usize) * (self.width as usize) + (x as usize))
        } else {
            None
        }
    }

    /// Set cell at position.
    pub fn set_cell(&mut self, x: u16, y: u16, cell: ScreenCell) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            if let Some(c) = self.cells.get_mut(idx) {
                *c = cell;
            }
        }
    }

    /// Print a character at cursor.
    pub fn print(&mut self, ch: char) {
        self.set_cell(
            self.cursor_x,
            self.cursor_y,
            ScreenCell { ch, fg: None, bg: None },
        );
        self.cursor_x += 1;
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
            if self.cursor_y >= self.height {
                self.scroll_up();
                self.cursor_y = self.height.saturating_sub(1);
            }
        }
    }

    /// Newline.
    pub fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y >= self.height {
            self.scroll_up();
            self.cursor_y = self.height.saturating_sub(1);
        }
    }

    /// Carriage return.
    pub fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    /// Scroll up by one line.
    pub fn scroll_up(&mut self) {
        let width = self.width as usize;
        self.cells.drain(0..width);
        self.cells.extend(std::iter::repeat(ScreenCell::default()).take(width));
    }

    /// Get cursor position.
    pub fn cursor(&self) -> (u16, u16) {
        (self.cursor_x, self.cursor_y)
    }

    /// Get dimensions.
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
