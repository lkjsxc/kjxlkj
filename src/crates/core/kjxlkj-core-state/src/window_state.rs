//! Per-window viewport and cursor state.

use kjxlkj_core_edit::Cursor;
use kjxlkj_core_types::{ContentKind, WindowId};

/// State associated with one window leaf.
#[derive(Debug, Clone)]
pub struct WindowState {
    pub window_id: WindowId,
    pub content: ContentKind,
    pub cursor: Cursor,
    pub top_line: usize,
    pub left_col: usize,
}

impl WindowState {
    pub fn new(window_id: WindowId, content: ContentKind) -> Self {
        Self {
            window_id,
            content,
            cursor: Cursor::default(),
            top_line: 0,
            left_col: 0,
        }
    }
}
