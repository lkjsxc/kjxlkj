//! Editor state.

use kjxlkj_core_mode::ModeState;
use kjxlkj_core_types::{BufferId, Mode, WindowId};
use kjxlkj_core_ui::{BufferView, Dimensions, EditorSnapshot, Layout, StatusLine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{BufferState, Registers, WindowState};

/// Complete editor state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    /// Buffers.
    #[serde(skip)]
    pub buffers: HashMap<BufferId, BufferState>,
    /// Windows.
    pub windows: HashMap<WindowId, WindowState>,
    /// Layout.
    pub layout: Layout,
    /// Mode state.
    pub mode: ModeState,
    /// Registers.
    pub registers: Registers,
    /// Terminal dimensions.
    pub dimensions: Dimensions,
    /// Message to display.
    pub message: Option<String>,
    /// Next buffer ID.
    pub(crate) next_buffer_id: u64,
    /// Next window ID.
    pub(crate) next_window_id: u64,
}

impl EditorState {
    /// Creates a new editor state.
    pub fn new() -> Self {
        let buffer_id = BufferId::new(0);
        let window_id = WindowId::new(0);

        let dims = Dimensions::new(80, 24);
        let buffer = BufferState::new(buffer_id);
        let mut window = WindowState::new(window_id, buffer_id);
        // Set initial viewport dimensions
        window.viewport.dimensions.height = dims.height.saturating_sub(2);
        window.viewport.dimensions.width = dims.width;

        let mut buffers = HashMap::new();
        buffers.insert(buffer_id, buffer);

        let mut windows = HashMap::new();
        windows.insert(window_id, window);

        Self {
            buffers,
            windows,
            layout: Layout::new(window_id),
            mode: ModeState::new(),
            registers: Registers::new(),
            dimensions: dims,
            message: None,
            next_buffer_id: 1,
            next_window_id: 1,
        }
    }

    /// Sets a message to display.
    pub fn set_message(&mut self, msg: &str) {
        self.message = Some(msg.to_string());
    }

    /// Clears the message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Returns the current mode.
    pub fn current_mode(&self) -> Mode {
        self.mode.mode
    }

    /// Returns the active window.
    pub fn active_window(&self) -> Option<&WindowState> {
        self.windows.get(&self.layout.active)
    }

    /// Returns the active buffer.
    pub fn active_buffer(&self) -> Option<&BufferState> {
        self.active_window()
            .and_then(|w| self.buffers.get(&w.buffer_id))
    }

    /// Returns the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let buffer_id = self.active_window()?.buffer_id;
        self.buffers.get_mut(&buffer_id)
    }

    /// Creates a new window for a buffer.
    pub fn create_window(&mut self, buffer_id: BufferId) -> WindowId {
        let window_id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;

        let mut window = WindowState::new(window_id, buffer_id);
        // Set viewport dimensions to terminal size minus status/command lines
        window.viewport.dimensions.height = self.dimensions.height.saturating_sub(2);
        window.viewport.dimensions.width = self.dimensions.width;
        self.windows.insert(window_id, window);

        window_id
    }

    /// Creates a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let mut snapshot = EditorSnapshot::new(self.dimensions);
        snapshot.mode = self.mode.mode;
        snapshot.layout = self.layout.clone();

        for window in self.windows.values() {
            if let Some(buffer) = self.buffers.get(&window.buffer_id) {
                let mut view = BufferView::new(window.id, window.buffer_id);
                view.name = buffer.name.clone();
                view.version = buffer.version();
                view.cursor = window.cursor;
                view.viewport = window.viewport;
                view.modified = buffer.modified;
                view.line_numbers = window.line_numbers;
                view.total_lines = buffer.line_count();

                let start = window.viewport.first_line();
                let end = window.viewport.last_line().min(buffer.line_count());
                for i in start..end {
                    view.lines.push(buffer.line(i));
                }

                snapshot.views.push(view);
            }
        }

        if let Some(window) = self.active_window() {
            if let Some(buffer) = self.buffers.get(&window.buffer_id) {
                snapshot.status = StatusLine {
                    mode: self.mode.mode,
                    file_name: buffer.name.as_str().to_string(),
                    modified: buffer.modified,
                    line: window.cursor.line() + 1,
                    col: window.cursor.col() + 1,
                    total_lines: buffer.line_count(),
                    file_type: buffer.filetype.clone(),
                    encoding: String::from("utf-8"),
                    line_ending: String::from("LF"),
                };
            }
        }

        // Include message in snapshot
        snapshot.message = self.message.clone();

        snapshot
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
