//! Editor state aggregation — ties together all core sub-crates.

use std::collections::HashMap;

use kjxlkj_core_mode::ModeManager;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Mode, Size, WindowId};
use kjxlkj_core_undo::UndoTree;

/// Per-buffer state.
pub struct BufferState {
    pub id: BufferId,
    pub text: TextBuffer,
    pub undo: UndoTree,
    pub file_path: Option<String>,
    pub modified: bool,
}

impl BufferState {
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            text: TextBuffer::new(),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            text: TextBuffer::from_str(text),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
        }
    }
}

/// Per-window state.
pub struct WindowState {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub top_line: usize,
}

impl WindowState {
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor_line: 0,
            cursor_col: 0,
            top_line: 0,
        }
    }
}

/// Top-level editor state that owns all buffers, windows, and modes.
pub struct EditorState {
    pub buffers: HashMap<BufferId, BufferState>,
    pub windows: HashMap<WindowId, WindowState>,
    pub mode: ModeManager,
    pub size: Size,
    next_buffer_id: u64,
    next_window_id: u64,
}

impl EditorState {
    pub fn new(size: Size) -> Self {
        Self {
            buffers: HashMap::new(),
            windows: HashMap::new(),
            mode: ModeManager::new(),
            size,
            next_buffer_id: 1,
            next_window_id: 1,
        }
    }

    /// Create a new buffer, returning its ID.
    pub fn create_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::new(id));
        tracing::debug!(?id, "created buffer");
        id
    }

    /// Create a new window for a buffer, returning the window ID.
    pub fn create_window(&mut self, buffer_id: BufferId) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        self.windows.insert(id, WindowState::new(id, buffer_id));
        tracing::debug!(?id, ?buffer_id, "created window");
        id
    }

    /// Current mode.
    pub fn current_mode(&self) -> Mode {
        self.mode.current()
    }
}
