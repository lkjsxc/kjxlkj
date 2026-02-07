//! EditorState: the central aggregate of all editor state.

use std::collections::HashMap;

use kjxlkj_core_edit::VisualSelection;
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, WindowId};

use crate::change_list::ChangeList;
use crate::command_line_state::CommandLineState;
use crate::jump_list::JumpList;
use crate::macro_state::MacroState;
use crate::marks::MarkEntry;
use crate::options::EditorOptions;
use crate::registers::RegisterEntry;
use crate::search::SearchState;
use crate::viewport::ViewportState;
use crate::window_state::WindowState;

/// The main editor state aggregating all subsystems.
pub struct EditorState {
    pub buffers: HashMap<BufferId, TextBuffer>,
    pub windows: Vec<WindowState>,
    pub active_window: usize,
    pub mode: ModeState,
    pub registers: HashMap<char, RegisterEntry>,
    pub marks: HashMap<char, MarkEntry>,
    pub jump_list: JumpList,
    pub change_list: ChangeList,
    pub search: SearchState,
    pub command_line: CommandLineState,
    pub message: Option<String>,
    pub should_quit: bool,
    pub next_buffer_id: u64,
    pub next_window_id: u64,
    pub viewport: ViewportState,
    pub options: EditorOptions,
    pub last_command: Option<String>,
    pub macro_state: MacroState,
    pub visual: Option<VisualSelection>,
    pub terminal_size: (u16, u16),
}

impl EditorState {
    /// Create a new EditorState with one empty buffer and one window.
    pub fn new() -> Self {
        let buf_id = BufferId(1);
        let win_id = WindowId(1);
        let buffer = TextBuffer::new(buf_id, "[No Name]".to_string());
        let window = WindowState::new(win_id, buf_id, 80, 24);
        let mut buffers = HashMap::new();
        buffers.insert(buf_id, buffer);
        Self {
            buffers,
            windows: vec![window],
            active_window: 0,
            mode: ModeState::new(),
            registers: HashMap::new(),
            marks: HashMap::new(),
            jump_list: JumpList::new(),
            change_list: ChangeList::new(),
            search: SearchState::new(),
            command_line: CommandLineState::new(),
            message: None,
            should_quit: false,
            next_buffer_id: 2,
            next_window_id: 2,
            viewport: ViewportState::default(),
            options: EditorOptions::default(),
            last_command: None,
            macro_state: MacroState::new(),
            visual: None,
            terminal_size: (80, 24),
        }
    }

    /// Get a reference to the active buffer.
    pub fn active_buffer(&self) -> &TextBuffer {
        let win = &self.windows[self.active_window];
        self.buffers.get(&win.buffer_id).expect("active buffer missing")
    }

    /// Get a mutable reference to the active buffer.
    pub fn active_buffer_mut(&mut self) -> &mut TextBuffer {
        let buf_id = self.windows[self.active_window].buffer_id;
        self.buffers.get_mut(&buf_id).expect("active buffer missing")
    }

    /// Get the active buffer's id.
    pub fn active_buffer_id(&self) -> BufferId {
        self.windows[self.active_window].buffer_id
    }

    /// Get a reference to the active window.
    pub fn active_window(&self) -> &WindowState {
        &self.windows[self.active_window]
    }

    /// Get a mutable reference to the active window.
    pub fn active_window_mut(&mut self) -> &mut WindowState {
        &mut self.windows[self.active_window]
    }

    /// Allocate a new buffer ID.
    pub fn alloc_buffer_id(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        id
    }

    /// Allocate a new window ID.
    pub fn alloc_window_id(&mut self) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Set a message for the status line.
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_has_buffer_and_window() {
        let state = EditorState::new();
        assert_eq!(state.buffers.len(), 1);
        assert_eq!(state.windows.len(), 1);
        assert_eq!(state.active_window, 0);
    }

    #[test]
    fn active_buffer_accessible() {
        let state = EditorState::new();
        let buf = state.active_buffer();
        assert_eq!(buf.name(), "[No Name]");
    }

    #[test]
    fn alloc_ids() {
        let mut state = EditorState::new();
        let b = state.alloc_buffer_id();
        assert_eq!(b, BufferId(2));
        let w = state.alloc_window_id();
        assert_eq!(w, WindowId(2));
    }
}
