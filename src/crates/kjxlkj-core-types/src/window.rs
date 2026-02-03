//! Window types.

use serde::{Deserialize, Serialize};

use crate::{BufferId, Cursor, WindowId};

/// Window state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowState {
    /// Window identifier.
    pub id: WindowId,
    /// Buffer shown in this window.
    pub buffer_id: BufferId,
    /// Cursor state.
    pub cursor: Cursor,
    /// Viewport state.
    pub viewport: Viewport,
}

impl WindowState {
    /// Create a new window state.
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor: Cursor::default(),
            viewport: Viewport::default(),
        }
    }
}

/// Viewport state (what portion of the buffer is visible).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    /// First visible line (0-indexed).
    pub top_line: u32,
    /// First visible column (0-indexed).
    pub left_col: u32,
    /// Number of visible lines.
    pub height: u16,
    /// Number of visible columns.
    pub width: u16,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            height: 24, // Sensible default for testing
            width: 80,
        }
    }
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(top_line: u32, left_col: u32, height: u16, width: u16) -> Self {
        Self {
            top_line,
            left_col,
            height,
            width,
        }
    }

    /// Get the last visible line.
    pub fn bottom_line(&self) -> u32 {
        self.top_line.saturating_add(self.height as u32).saturating_sub(1)
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: u32) -> bool {
        line >= self.top_line && line <= self.bottom_line()
    }

    /// Scroll to ensure a line is visible.
    pub fn ensure_line_visible(&mut self, line: u32, scrolloff: u32) {
        let effective_top = self.top_line + scrolloff;
        let effective_bottom = self.bottom_line().saturating_sub(scrolloff);

        if line < effective_top {
            self.top_line = line.saturating_sub(scrolloff);
        } else if line > effective_bottom {
            self.top_line = line
                .saturating_sub(self.height as u32)
                .saturating_add(scrolloff)
                .saturating_add(1);
        }
    }
}
