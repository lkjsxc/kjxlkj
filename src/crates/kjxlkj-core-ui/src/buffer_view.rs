//! Buffer view types.

use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, WindowId};
use serde::{Deserialize, Serialize};

use crate::Viewport;

/// A view of a buffer for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferView {
    /// Window ID.
    pub window_id: WindowId,
    /// Buffer ID.
    pub buffer_id: BufferId,
    /// Buffer name.
    pub name: BufferName,
    /// Buffer version.
    pub version: BufferVersion,
    /// Cursor state.
    pub cursor: Cursor,
    /// Viewport.
    pub viewport: Viewport,
    /// Visible lines content.
    pub lines: Vec<String>,
    /// Modified flag.
    pub modified: bool,
    /// Line numbers enabled.
    pub line_numbers: bool,
    /// Total line count.
    pub total_lines: usize,
}

impl BufferView {
    /// Creates a new buffer view.
    pub fn new(window_id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            window_id,
            buffer_id,
            name: BufferName::default(),
            version: BufferVersion::default(),
            cursor: Cursor::default(),
            viewport: Viewport::default(),
            lines: Vec::new(),
            modified: false,
            line_numbers: true,
            total_lines: 0,
        }
    }

    /// Returns the cursor line (relative to viewport).
    pub fn cursor_row(&self) -> usize {
        self.cursor.line().saturating_sub(self.viewport.first_line())
    }

    /// Returns the cursor column.
    pub fn cursor_col(&self) -> usize {
        self.cursor.col()
    }
}
