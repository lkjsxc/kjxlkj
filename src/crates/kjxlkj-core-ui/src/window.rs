//! Window management.

use kjxlkj_core_types::{
    cursor::Cursor,
    ids::{BufferId, WindowId},
    snapshot::WindowDimensions,
};

/// A window displaying a buffer.
#[derive(Debug, Clone)]
pub struct Window {
    /// Window identifier.
    id: WindowId,
    /// Buffer displayed in this window.
    buffer_id: BufferId,
    /// Cursor position.
    cursor: Cursor,
    /// First visible line (scroll offset).
    top_line: usize,
    /// Window dimensions.
    dimensions: WindowDimensions,
}

impl Window {
    /// Creates a new window.
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor: Cursor::default(),
            top_line: 0,
            dimensions: WindowDimensions::default(),
        }
    }

    /// Returns the window ID.
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Returns the buffer ID.
    pub fn buffer_id(&self) -> BufferId {
        self.buffer_id
    }

    /// Sets the buffer ID.
    pub fn set_buffer(&mut self, buffer_id: BufferId) {
        self.buffer_id = buffer_id;
        self.cursor = Cursor::default();
        self.top_line = 0;
    }

    /// Returns the cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Returns a mutable reference to the cursor.
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Returns the top visible line.
    pub fn top_line(&self) -> usize {
        self.top_line
    }

    /// Sets the top visible line.
    pub fn set_top_line(&mut self, line: usize) {
        self.top_line = line;
    }

    /// Returns the window dimensions.
    pub fn dimensions(&self) -> WindowDimensions {
        self.dimensions
    }

    /// Sets the window dimensions.
    pub fn set_dimensions(&mut self, dimensions: WindowDimensions) {
        self.dimensions = dimensions;
    }

    /// Returns the number of visible lines.
    pub fn visible_lines(&self) -> usize {
        self.dimensions.text_height() as usize
    }

    /// Scrolls to ensure the cursor is visible.
    pub fn scroll_to_cursor(&mut self) {
        let cursor_line = self.cursor.line();
        let visible = self.visible_lines();
        
        if cursor_line < self.top_line {
            self.top_line = cursor_line;
        } else if cursor_line >= self.top_line + visible {
            self.top_line = cursor_line.saturating_sub(visible - 1);
        }
    }
}
