//! Viewport types for window state.

use serde::{Deserialize, Serialize};

/// Viewport state for a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewportState {
    /// First buffer line shown at top.
    pub top_line: usize,
    /// First display column shown (no-wrap only).
    pub left_col: usize,
    /// Soft-wrap long lines.
    pub wrap: bool,
    /// Height of text area in rows.
    pub text_rows: u16,
    /// Width of text area in columns.
    pub text_cols: u16,
    /// Vertical margin around cursor.
    pub scrolloff: u16,
    /// Horizontal margin around cursor.
    pub sidescrolloff: u16,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            top_line: 0,
            left_col: 0,
            wrap: true,
            text_rows: 24,
            text_cols: 80,
            scrolloff: 3,
            sidescrolloff: 5,
        }
    }
}

impl ViewportState {
    /// Create a new viewport state.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            text_rows: rows,
            text_cols: cols,
            ..Default::default()
        }
    }

    /// Clamp top_line to valid range.
    pub fn clamp_top_line(&mut self, line_count: usize) {
        if line_count == 0 {
            self.top_line = 0;
            return;
        }
        let max_top = line_count.saturating_sub(1);
        self.top_line = self.top_line.min(max_top);
    }

    /// Ensure cursor is visible in viewport.
    pub fn ensure_cursor_visible(&mut self, cursor_line: usize, line_count: usize) {
        let rows = self.text_rows as usize;
        let margin = (self.scrolloff as usize).min(rows.saturating_sub(1) / 2);

        let min_visible = self.top_line.saturating_add(margin);
        let max_visible = self
            .top_line
            .saturating_add(rows)
            .saturating_sub(1 + margin);

        if cursor_line < min_visible {
            self.top_line = cursor_line.saturating_sub(margin);
        } else if cursor_line > max_visible && rows > 0 {
            self.top_line = cursor_line.saturating_sub(rows - 1 - margin);
        }

        self.clamp_top_line(line_count);
    }
}
