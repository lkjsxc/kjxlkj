//! Window state management.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_types::{BufferId, TerminalId, WindowId};

use crate::ViewportState;

/// What a window displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowContent {
    Buffer(BufferId),
    Terminal(TerminalId),
}

/// Per-window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub wrap: bool,
    pub number: bool,
    pub relative_number: bool,
    pub scroll_off: u16,
    pub side_scroll_off: u16,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            wrap: true,
            number: true,
            relative_number: false,
            scroll_off: 5,
            side_scroll_off: 0,
        }
    }
}

/// State for a single window (viewport over a buffer or terminal).
pub struct WindowState {
    /// Stable unique identifier.
    pub id: WindowId,
    /// What this window displays.
    pub content: WindowContent,
    /// Cursor position (for buffer windows).
    pub cursor: CursorPosition,
    /// Viewport state.
    pub viewport: ViewportState,
    /// Per-window options.
    pub options: WindowOptions,
}

impl WindowState {
    /// Create a new buffer window.
    pub fn new_buffer(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            content: WindowContent::Buffer(buffer_id),
            cursor: CursorPosition::zero(),
            viewport: ViewportState::new(),
            options: WindowOptions::default(),
        }
    }

    /// Create a new terminal window.
    pub fn new_terminal(id: WindowId, terminal_id: TerminalId) -> Self {
        Self {
            id,
            content: WindowContent::Terminal(terminal_id),
            cursor: CursorPosition::zero(),
            viewport: ViewportState::new(),
            options: WindowOptions::default(),
        }
    }

    /// Get the buffer ID if this is a buffer window.
    pub fn buffer_id(&self) -> Option<BufferId> {
        match self.content {
            WindowContent::Buffer(id) => Some(id),
            WindowContent::Terminal(_) => None,
        }
    }

    /// Set this window to display a different buffer.
    pub fn set_buffer(&mut self, id: BufferId) {
        self.content = WindowContent::Buffer(id);
    }

    /// Get the terminal ID if this is a terminal window.
    pub fn terminal_id(&self) -> Option<TerminalId> {
        match self.content {
            WindowContent::Terminal(id) => Some(id),
            WindowContent::Buffer(_) => None,
        }
    }

    /// Number of gutter columns (line numbers + sign column).
    pub fn gutter_width(&self, line_count: usize) -> u16 {
        if !self.options.number && !self.options.relative_number {
            return 0;
        }
        // Width = digits for max line number + 1 space padding
        let digit_count = if line_count == 0 {
            1
        } else {
            ((line_count as f64).log10().floor() as u16) + 1
        };
        digit_count.max(2) + 1 // min 2 digits + space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_window() {
        let w = WindowState::new_buffer(
            WindowId(1),
            BufferId(1),
        );
        assert_eq!(w.buffer_id(), Some(BufferId(1)));
        assert_eq!(w.terminal_id(), None);
    }

    #[test]
    fn gutter_width_small() {
        let w = WindowState::new_buffer(
            WindowId(1),
            BufferId(1),
        );
        assert_eq!(w.gutter_width(10), 3);
    }

    #[test]
    fn gutter_width_large() {
        let w = WindowState::new_buffer(
            WindowId(1),
            BufferId(1),
        );
        assert_eq!(w.gutter_width(1000), 5);
    }
}
