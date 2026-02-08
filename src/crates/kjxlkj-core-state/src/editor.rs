//! Editor state: aggregates buffers, windows, mode, and snapshot production.

use std::collections::HashMap;

use kjxlkj_core_mode::{
    CommandModeState, InsertModeState, NormalModeState, VisualModeState,
};
use kjxlkj_core_types::{
    BufferId, Mode, WindowId,
};
use kjxlkj_core_ui::{
    BufferSnapshot, CmdlineState, EditorSnapshot, WindowLayout,
};
use kjxlkj_core_ui::Rect;

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
}

impl EditorState {
    /// Create editor state with a single empty buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(1);
        let win_id = WindowId(1);

        let buf = BufferState::new(buf_id);
        let mut win = WindowState::new_buffer(win_id, buf_id);
        win.viewport.set_size(cols, rows.saturating_sub(2));

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
        }
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

    /// Get the focused window.
    pub fn focused_window(&self) -> Option<&WindowState> {
        self.windows.get(&self.focused_window)
    }

    /// Get the focused window mutably.
    pub fn focused_window_mut(&mut self) -> Option<&mut WindowState> {
        self.windows.get_mut(&self.focused_window)
    }

    /// Get the buffer displayed in the focused window.
    pub fn active_buffer_id(&self) -> Option<BufferId> {
        self.focused_window()
            .and_then(|w| w.buffer_id())
    }

    /// Get the active buffer.
    pub fn active_buffer(&self) -> Option<&BufferState> {
        self.active_buffer_id()
            .and_then(|id| self.buffers.get(&id))
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let id = self.active_buffer_id()?;
        self.buffers.get_mut(&id)
    }

    /// Produce an immutable snapshot for the render task.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.sequence += 1;
        let (cols, rows) = self.terminal_size;

        let mut buf_snaps = HashMap::new();
        for (id, buf) in &self.buffers {
            let snap = self.build_buffer_snapshot(buf);
            buf_snaps.insert(*id, snap);
        }

        let layout = WindowLayout::single(
            self.focused_window,
            Rect::new(0, 0, cols, rows.saturating_sub(1)),
        );

        let cmdline = if let Some(ref cs) = self.command_state {
            CmdlineState {
                active: true,
                prompt: cs.prompt_char(),
                content: cs.content().to_string(),
                cursor: cs.cursor,
                completions: Vec::new(),
                completion_index: None,
            }
        } else {
            CmdlineState::inactive()
        };

        EditorSnapshot {
            sequence: self.sequence,
            layout,
            buffers: buf_snaps,
            terminals: HashMap::new(),
            mode: self.mode.clone(),
            cmdline,
            notifications: Vec::new(),
            search: Default::default(),
            theme: Default::default(),
            terminal_size: self.terminal_size,
        }
    }

    fn build_buffer_snapshot(
        &self,
        buf: &BufferState,
    ) -> BufferSnapshot {
        let win = self.windows.get(&self.focused_window);
        let (cursor_line, cursor_col) = win
            .map(|w| (w.cursor.line, w.cursor.grapheme_offset))
            .unwrap_or((0, 0));
        let top_line = win
            .map(|w| w.viewport.top_line)
            .unwrap_or(0);

        let height = win
            .map(|w| w.viewport.height as usize)
            .unwrap_or(24);

        let mut visible_lines = Vec::with_capacity(height);
        for line_idx in top_line..top_line + height {
            if line_idx < buf.line_count() {
                visible_lines.push(buf.content.line_content(line_idx));
            } else {
                visible_lines.push(String::from("~"));
            }
        }

        BufferSnapshot {
            id: buf.id,
            version: buf.version,
            line_count: buf.line_count(),
            path: buf.path.clone(),
            name: buf.name.clone(),
            modified: buf.modified,
            readonly: buf.readonly,
            visible_lines,
            top_line,
            cursor_line,
            cursor_col,
            file_type: buf.file_type.clone(),
            line_ending: buf.line_ending.as_str().to_string(),
            encoding: String::from("utf-8"),
        }
    }

    /// Handle a resize event.
    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.terminal_size = (cols, rows);
        for win in self.windows.values_mut() {
            win.viewport.set_size(cols, rows.saturating_sub(2));
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
