//! Central editor state.

use kjxlkj_core_types::{
    ids::{BufferId, WindowId},
    snapshot::{EditorSnapshot, StatusSnapshot, WindowSnapshot},
};
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_ui::{Layout, Window};
use crate::buffers::BufferStore;
use crate::registers::RegisterStore;
use std::collections::HashMap;

/// The central editor state.
#[derive(Debug)]
pub struct EditorState {
    /// Buffer storage.
    pub buffers: BufferStore,
    /// Register storage.
    pub registers: RegisterStore,
    /// Mode state.
    pub mode: ModeState,
    /// Window storage.
    windows: HashMap<WindowId, Window>,
    /// Active window ID.
    active_window: WindowId,
    /// Layout.
    layout: Option<Layout>,
    /// Next window ID.
    next_window_id: u64,
    /// Command line content.
    command_line: String,
    /// Last message.
    message: Option<String>,
}

impl EditorState {
    /// Creates a new editor state.
    pub fn new() -> Self {
        let mut buffers = BufferStore::new();
        let buffer_id = buffers.create();

        let window_id = WindowId::new(1);
        let window = Window::new(window_id, buffer_id);
        let mut windows = HashMap::new();
        windows.insert(window_id, window);

        Self {
            buffers,
            registers: RegisterStore::new(),
            mode: ModeState::new(),
            windows,
            active_window: window_id,
            layout: None,
            next_window_id: 2,
            command_line: String::new(),
            message: None,
        }
    }

    /// Returns the active window.
    pub fn active_window(&self) -> Option<&Window> {
        self.windows.get(&self.active_window)
    }

    /// Returns a mutable reference to the active window.
    pub fn active_window_mut(&mut self) -> Option<&mut Window> {
        self.windows.get_mut(&self.active_window)
    }

    /// Returns the active buffer ID.
    pub fn active_buffer_id(&self) -> Option<BufferId> {
        self.active_window().map(|w| w.buffer_id())
    }

    /// Sets the active window.
    pub fn set_active_window(&mut self, id: WindowId) {
        if self.windows.contains_key(&id) {
            self.active_window = id;
        }
    }

    /// Creates a snapshot for rendering.
    pub fn snapshot(&self) -> EditorSnapshot {
        let windows: Vec<WindowSnapshot> = self
            .windows
            .values()
            .map(|w| {
                let buffer = self.buffers.get(w.buffer_id());
                WindowSnapshot {
                    id: w.id(),
                    buffer_id: w.buffer_id(),
                    buffer_info: buffer.map(|b| b.info()).unwrap_or_else(|| {
                        kjxlkj_core_types::buffer::BufferInfo::new(
                            w.buffer_id(),
                            kjxlkj_core_types::buffer::BufferName::Unnamed,
                        )
                    }),
                    cursor: *w.cursor(),
                    selection: None,
                    top_line: w.top_line(),
                    dimensions: w.dimensions(),
                    visible_range: (w.top_line(), w.top_line() + w.visible_lines()),
                }
            })
            .collect();

        EditorSnapshot {
            mode: self.mode.mode(),
            active_window: self.active_window,
            windows,
            command_line: None,
            status: StatusSnapshot {
                mode_text: self.mode.mode().to_string(),
                ..Default::default()
            },
            message: self
                .message
                .as_ref()
                .map(kjxlkj_core_types::snapshot::MessageSnapshot::info),
        }
    }

    /// Sets a message.
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }

    /// Clears the message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
