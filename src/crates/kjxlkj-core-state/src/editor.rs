//! Editor state: aggregates buffers, windows, mode,
//! and snapshot production.

use std::collections::HashMap;

use kjxlkj_core_mode::{
    CommandModeState, InsertModeState,
    NormalModeState, VisualModeState,
};
use kjxlkj_core_types::{BufferId, Mode, WindowId};

use crate::search::SearchState;
use crate::{BufferState, WindowState};

/// Top-level editor state.
pub struct EditorState {
    /// All open buffers.
    pub buffers: HashMap<BufferId, BufferState>,
    /// All open windows.
    pub windows: HashMap<WindowId, WindowState>,
    /// Currently focused window.
    pub focused_window: WindowId,
    /// Current editing mode.
    pub mode: Mode,
    /// Normal mode state.
    pub normal_state: NormalModeState,
    /// Insert mode state.
    pub insert_state: InsertModeState,
    /// Visual mode state.
    pub visual_state: Option<VisualModeState>,
    /// Command mode state.
    pub command_state: Option<CommandModeState>,
    /// Snapshot sequence counter.
    pub sequence: u64,
    /// Terminal dimensions.
    pub terminal_size: (u16, u16),
    /// Next buffer ID.
    next_buffer_id: u64,
    /// Next window ID.
    next_window_id: u64,
    /// Should quit flag.
    pub should_quit: bool,
    /// Search state.
    pub search_state: SearchState,
}

impl EditorState {
    /// Create editor state with a single empty buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(1);
        let win_id = WindowId(1);

        let buf = BufferState::new(buf_id);
        let mut win =
            WindowState::new_buffer(win_id, buf_id);
        win.viewport
            .set_size(cols, rows.saturating_sub(2));

        let mut buffers = HashMap::new();
        buffers.insert(buf_id, buf);

        let mut windows = HashMap::new();
        windows.insert(win_id, win);

        Self {
            buffers,
            windows,
            focused_window: win_id,
            mode: Mode::Normal,
            normal_state: NormalModeState::new(),
            insert_state: InsertModeState::new(),
            visual_state: None,
            command_state: None,
            sequence: 0,
            terminal_size: (cols, rows),
            next_buffer_id: 2,
            next_window_id: 2,
            should_quit: false,
            search_state: SearchState::new(),
        }
    }

    /// Allocate a new buffer ID.
    pub fn alloc_buffer_id(
        &mut self,
    ) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        id
    }

    /// Allocate a new window ID.
    pub fn alloc_window_id(
        &mut self,
    ) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Get the focused window.
    pub fn focused_window(
        &self,
    ) -> Option<&WindowState> {
        self.windows.get(&self.focused_window)
    }

    /// Get the focused window mutably.
    pub fn focused_window_mut(
        &mut self,
    ) -> Option<&mut WindowState> {
        self.windows.get_mut(&self.focused_window)
    }

    /// Get the active buffer ID.
    pub fn active_buffer_id(
        &self,
    ) -> Option<BufferId> {
        self.focused_window()
            .and_then(|w| w.buffer_id())
    }

    /// Get the active buffer.
    pub fn active_buffer(
        &self,
    ) -> Option<&BufferState> {
        self.active_buffer_id()
            .and_then(|id| self.buffers.get(&id))
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(
        &mut self,
    ) -> Option<&mut BufferState> {
        let id = self.active_buffer_id()?;
        self.buffers.get_mut(&id)
    }

    /// Handle a resize event.
    pub fn handle_resize(
        &mut self,
        cols: u16,
        rows: u16,
    ) {
        self.terminal_size = (cols, rows);
        for win in self.windows.values_mut() {
            win.viewport
                .set_size(cols, rows.saturating_sub(2));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_editor() {
        let state = EditorState::new(80, 24);
        assert_eq!(state.mode, Mode::Normal);
        assert_eq!(state.buffers.len(), 1);
        assert_eq!(state.windows.len(), 1);
        assert!(!state.should_quit);
    }

    #[test]
    fn snapshot_production() {
        let mut state = EditorState::new(80, 24);
        let snap = state.snapshot();
        assert_eq!(snap.sequence, 1);
        assert_eq!(snap.mode, Mode::Normal);
        assert_eq!(snap.buffers.len(), 1);
    }
}
