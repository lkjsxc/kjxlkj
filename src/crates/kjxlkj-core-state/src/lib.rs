//! Editor state aggregation — ties together all core sub-crates.

mod buffer_state;
mod dispatch;
mod registers;
mod window_state;

pub use buffer_state::BufferState;
pub use dispatch::dispatch_intent;
pub use registers::RegisterFile;
pub use window_state::WindowState;

use std::collections::HashMap;

use kjxlkj_core_mode::{KeyParser, ModeState};
use kjxlkj_core_types::{BufferId, Mode, Position, Size, WindowId};

/// Top-level editor state that owns all buffers, windows, modes, registers.
pub struct EditorState {
    pub buffers: HashMap<BufferId, BufferState>,
    pub windows: HashMap<WindowId, WindowState>,
    pub mode: ModeState,
    pub parser: KeyParser,
    pub registers: RegisterFile,
    pub size: Size,
    pub active_window: Option<WindowId>,
    pub message: Option<String>,
    pub should_quit: bool,
    next_buffer_id: u64,
    next_window_id: u64,
}

impl EditorState {
    pub fn new(size: Size) -> Self {
        Self {
            buffers: HashMap::new(),
            windows: HashMap::new(),
            mode: ModeState::new(),
            parser: KeyParser::new(),
            registers: RegisterFile::new(),
            size,
            active_window: None,
            message: None,
            should_quit: false,
            next_buffer_id: 1,
            next_window_id: 1,
        }
    }

    /// Create a new empty buffer, returning its ID.
    pub fn create_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::new(id));
        tracing::debug!(?id, "created buffer");
        id
    }

    /// Create a buffer from text content.
    pub fn create_buffer_from_text(&mut self, text: &str) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::from_text(id, text));
        tracing::debug!(?id, "created buffer from text");
        id
    }

    /// Create a new window for a buffer, returning the window ID.
    pub fn create_window(&mut self, buffer_id: BufferId) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        self.windows.insert(id, WindowState::new(id, buffer_id));
        if self.active_window.is_none() {
            self.active_window = Some(id);
        }
        tracing::debug!(?id, ?buffer_id, "created window");
        id
    }

    /// Current mode.
    pub fn current_mode(&self) -> Mode {
        self.mode.current()
    }

    /// Get cursor position in the active window.
    pub fn cursor(&self) -> Position {
        self.active_window
            .and_then(|wid| self.windows.get(&wid))
            .map(|w| Position::new(w.cursor_line, w.cursor_col))
            .unwrap_or_default()
    }

    /// Get the active buffer, if any.
    pub fn active_buffer(&self) -> Option<&BufferState> {
        let wid = self.active_window?;
        let win = self.windows.get(&wid)?;
        self.buffers.get(&win.buffer_id)
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let wid = self.active_window?;
        let win = self.windows.get(&wid)?;
        let bid = win.buffer_id;
        self.buffers.get_mut(&bid)
    }

    /// Get the active window, if any.
    pub fn active_window_state(&self) -> Option<&WindowState> {
        self.active_window.and_then(|id| self.windows.get(&id))
    }

    /// Get the active window mutably.
    pub fn active_window_mut(&mut self) -> Option<&mut WindowState> {
        let wid = self.active_window?;
        self.windows.get_mut(&wid)
    }
}
