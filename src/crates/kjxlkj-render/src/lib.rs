//! Rendering pipeline — diff-based terminal output.

mod viewport_wrap;

use kjxlkj_core_types::Size;
use kjxlkj_core_ui::UiModel;

/// A single cell in the terminal grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Default,
            bg: Color::Default,
            bold: false,
            underline: false,
            reverse: false,
        }
    }
}

/// Terminal color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

/// A double-buffered render frame for diff-based updates.
pub struct RenderFrame {
    size: Size,
    current: Vec<Cell>,
    previous: Vec<Cell>,
}

impl RenderFrame {
    pub fn new(size: Size) -> Self {
        let total = size.width as usize * size.height as usize;
        Self {
            size,
            current: vec![Cell::default(); total],
            previous: vec![Cell::default(); total],
        }
    }

    /// Resize the frame, clearing both buffers.
    pub fn resize(&mut self, size: Size) {
        let total = size.width as usize * size.height as usize;
        self.size = size;
        self.current = vec![Cell::default(); total];
        self.previous = vec![Cell::default(); total];
    }

    /// Swap current and previous buffers after flushing.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.previous);
        for cell in &mut self.current {
            *cell = Cell::default();
        }
    }
}

/// The renderer that converts a `UiModel` into terminal output.
pub struct Renderer {
    frame: RenderFrame,
}

impl Renderer {
    pub fn new(size: Size) -> Self {
        Self {
            frame: RenderFrame::new(size),
        }
    }

    /// Render the given UI model to the terminal.
    pub fn render(&mut self, _model: &UiModel) -> anyhow::Result<()> {
        // Future: populate self.frame.current from model, diff against previous, emit crossterm commands
        self.frame.swap();
        Ok(())
    }

    /// Handle a terminal resize.
    pub fn resize(&mut self, size: Size) {
        self.frame.resize(size);
        tracing::debug!(width = size.width, height = size.height, "renderer resized");
    }
}
