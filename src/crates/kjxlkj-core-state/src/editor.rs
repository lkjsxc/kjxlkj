//! Main editor state.

use std::collections::HashMap;
use std::path::PathBuf;

use kjxlkj_core_mode::{process_key, ModeState};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    BufferId, EditorResult, Key, Mode, WindowId, WindowState,
};
use kjxlkj_core_undo::UndoHistory;
use kjxlkj_core_ui::{
    build_status, EditorSnapshot, LineSnapshot, MessageLevel, StatusMessage, WindowSnapshot,
};

use crate::RegisterStore;

/// Complete editor state.
pub struct EditorState {
    /// All buffers.
    pub(crate) buffers: HashMap<BufferId, TextBuffer>,
    /// All windows.
    pub(crate) windows: HashMap<WindowId, WindowState>,
    /// Active window ID.
    pub(crate) active_window: WindowId,
    /// Mode state.
    pub(crate) mode_state: ModeState,
    /// Undo histories per buffer.
    pub(crate) undo_histories: HashMap<BufferId, UndoHistory>,
    /// Register store.
    pub(crate) registers: RegisterStore,
    /// Current status message.
    pub(crate) message: Option<StatusMessage>,
    /// Whether to quit.
    pub(crate) should_quit: bool,
    /// Terminal size.
    terminal_size: (u16, u16),
}

impl EditorState {
    /// Create a new editor state with an empty buffer.
    pub fn new() -> Self {
        let buffer = TextBuffer::new();
        let buffer_id = buffer.id();
        let window_id = WindowId::new();
        let window = WindowState::new(window_id, buffer_id);

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);
        let mut windows = HashMap::new();
        windows.insert(window_id, window);
        let mut undo_histories = HashMap::new();
        undo_histories.insert(buffer_id, UndoHistory::new());

        Self {
            buffers, windows, active_window: window_id, mode_state: ModeState::new(),
            undo_histories, registers: RegisterStore::new(), message: None,
            should_quit: false, terminal_size: (80, 24),
        }
    }

    /// Create editor state with initial file.
    pub fn with_file(path: PathBuf, content: &str) -> Self {
        let buffer = TextBuffer::from_file(path, content);
        let buffer_id = buffer.id();
        let window_id = WindowId::new();
        let window = WindowState::new(window_id, buffer_id);

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);
        let mut windows = HashMap::new();
        windows.insert(window_id, window);
        let mut undo_histories = HashMap::new();
        undo_histories.insert(buffer_id, UndoHistory::new());

        Self {
            buffers, windows, active_window: window_id, mode_state: ModeState::new(),
            undo_histories, registers: RegisterStore::new(), message: None,
            should_quit: false, terminal_size: (80, 24),
        }
    }

    /// Set terminal size.
    pub fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
        for window in self.windows.values_mut() {
            window.viewport.width = width;
            window.viewport.height = height.saturating_sub(2);
        }
    }

    /// Check if editor should quit.
    pub fn should_quit(&self) -> bool { self.should_quit }

    /// Get current mode.
    pub fn mode(&self) -> Mode { self.mode_state.mode }

    /// Get command line content.
    pub fn command_line(&self) -> &str { &self.mode_state.command_line }

    /// Process a key event.
    pub fn handle_key(&mut self, key: Key) -> EditorResult<()> {
        let intents = process_key(&mut self.mode_state, key);
        for intent in intents {
            self.apply_intent(intent)?;
        }
        Ok(())
    }

    /// Set a status message.
    pub fn set_message(&mut self, text: &str, level: MessageLevel) {
        self.message = Some(StatusMessage { text: text.to_string(), level });
    }

    /// Clear the status message.
    pub fn clear_message(&mut self) { self.message = None; }

    /// Create a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let window = self.windows.get(&self.active_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        let meta = buffer.meta();
        let viewport = &window.viewport;

        let mut lines = Vec::new();
        let start_line = viewport.top_line as usize;
        let end_line = (viewport.top_line + viewport.height as u32) as usize;

        for line_idx in start_line..end_line.min(buffer.line_count()) {
            let text = buffer.line(line_idx).unwrap_or_default();
            lines.push(LineSnapshot {
                number: (line_idx + 1) as u32,
                text,
                is_cursor_line: line_idx == window.cursor.line() as usize,
            });
        }

        let status = build_status(self.mode_state.mode, &meta, &window.cursor, buffer.line_count());
        let selection = self.mode_state.visual_range(window.cursor.position);

        EditorSnapshot {
            active_window: WindowSnapshot {
                id: window.id,
                buffer_meta: meta,
                lines,
                cursor: window.cursor,
                cursor_style: self.mode_state.mode.cursor_style(),
                viewport: window.viewport,
                selection,
            },
            mode: self.mode_state.mode,
            status,
            command_line: if self.mode_state.mode == Mode::Command {
                Some(self.mode_state.command_line.clone())
            } else { None },
            message: self.message.clone(),
        }
    }
}

impl Default for EditorState {
    fn default() -> Self { Self::new() }
}
