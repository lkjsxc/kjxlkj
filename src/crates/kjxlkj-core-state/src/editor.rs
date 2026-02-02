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
    /// Next buffer ID.
    next_buffer_id: u64,
    /// Next window ID.
    next_window_id: u64,
}

impl EditorState {
    /// Creates a new editor state.
    pub fn new() -> Self {
        let buffer_id = BufferId::new(0);
        let window_id = WindowId::new(0);

        let buffer = BufferState::new(buffer_id);
        let window = WindowState::new(window_id, buffer_id);

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
            dimensions: Dimensions::new(80, 24),
            next_buffer_id: 1,
            next_window_id: 1,
        }
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

        snapshot
    }

    /// Allocates a new buffer ID.
    pub fn alloc_buffer_id(&mut self) -> BufferId {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;
        id
    }

    /// Allocates a new window ID.
    pub fn alloc_window_id(&mut self) -> WindowId {
        let id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Creates a new buffer and returns its ID.
    pub fn create_buffer(&mut self) -> BufferId {
        let id = self.alloc_buffer_id();
        let buffer = BufferState::new(id);
        self.buffers.insert(id, buffer);
        id
    }

    /// Creates a buffer from content.
    pub fn create_buffer_from(&mut self, content: &str) -> BufferId {
        let id = self.alloc_buffer_id();
        let buffer = BufferState::from_content(id, content);
        self.buffers.insert(id, buffer);
        id
    }

    /// Returns a buffer by ID.
    pub fn buffer(&self, id: BufferId) -> Option<&BufferState> {
        self.buffers.get(&id)
    }

    /// Returns a mutable buffer by ID.
    pub fn buffer_mut(&mut self, id: BufferId) -> Option<&mut BufferState> {
        self.buffers.get_mut(&id)
    }

    /// Returns all buffer IDs.
    pub fn buffer_ids(&self) -> Vec<BufferId> {
        self.buffers.keys().copied().collect()
    }

    /// Returns buffer count.
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    /// Switches the active window to a buffer.
    pub fn switch_buffer(&mut self, buffer_id: BufferId) -> bool {
        if !self.buffers.contains_key(&buffer_id) {
            return false;
        }
        if let Some(window) = self.windows.get_mut(&self.layout.active) {
            window.buffer_id = buffer_id;
            true
        } else {
            false
        }
    }

    /// Goes to the next buffer.
    pub fn next_buffer(&mut self) -> bool {
        let ids: Vec<_> = self.buffer_ids();
        if ids.len() <= 1 {
            return false;
        }
        if let Some(window) = self.active_window() {
            let current = window.buffer_id;
            let pos = ids.iter().position(|&id| id == current).unwrap_or(0);
            let next = ids[(pos + 1) % ids.len()];
            return self.switch_buffer(next);
        }
        false
    }

    /// Goes to the previous buffer.
    pub fn prev_buffer(&mut self) -> bool {
        let ids: Vec<_> = self.buffer_ids();
        if ids.len() <= 1 {
            return false;
        }
        if let Some(window) = self.active_window() {
            let current = window.buffer_id;
            let pos = ids.iter().position(|&id| id == current).unwrap_or(0);
            let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
            return self.switch_buffer(ids[prev]);
        }
        false
    }

    /// Deletes a buffer.
    pub fn delete_buffer(&mut self, buffer_id: BufferId) -> bool {
        if self.buffers.len() <= 1 {
            return false; // Can't delete last buffer
        }
        if !self.buffers.contains_key(&buffer_id) {
            return false;
        }
        // Switch windows using this buffer to another
        let other_id = self.buffers.keys()
            .find(|&&id| id != buffer_id)
            .copied();
        if let Some(other) = other_id {
            for window in self.windows.values_mut() {
                if window.buffer_id == buffer_id {
                    window.buffer_id = other;
                }
            }
        }
        self.buffers.remove(&buffer_id);
        true
    }

    /// Returns buffer list info for :ls command.
    pub fn buffer_list(&self) -> Vec<BufferInfo> {
        let active_buf = self.active_window().map(|w| w.buffer_id);
        self.buffers.values().map(|b| BufferInfo {
            id: b.id,
            name: b.name.as_str().to_string(),
            path: b.path.clone(),
            modified: b.modified,
            active: Some(b.id) == active_buf,
        }).collect()
    }

    // Window management

    /// Returns window count.
    pub fn window_count(&self) -> usize {
        self.layout.window_count()
    }

    /// Splits the active window horizontally.
    pub fn split_horizontal(&mut self) -> Option<WindowId> {
        let buffer_id = self.active_window()?.buffer_id;
        let window_id = self.alloc_window_id();
        let window = WindowState::new(window_id, buffer_id);
        self.windows.insert(window_id, window);
        if self.layout.split_horizontal(window_id) {
            Some(window_id)
        } else {
            self.windows.remove(&window_id);
            None
        }
    }

    /// Splits the active window vertically.
    pub fn split_vertical(&mut self) -> Option<WindowId> {
        let buffer_id = self.active_window()?.buffer_id;
        let window_id = self.alloc_window_id();
        let window = WindowState::new(window_id, buffer_id);
        self.windows.insert(window_id, window);
        if self.layout.split_vertical(window_id) {
            Some(window_id)
        } else {
            self.windows.remove(&window_id);
            None
        }
    }

    /// Closes the active window.
    pub fn close_window(&mut self) -> bool {
        if self.window_count() <= 1 {
            return false;
        }
        let active_id = self.layout.active;
        if self.layout.close_active() {
            self.windows.remove(&active_id);
            true
        } else {
            false
        }
    }

    /// Navigates to the next window.
    pub fn next_window(&mut self) -> bool {
        self.layout.next_window()
    }

    /// Navigates to the previous window.
    pub fn prev_window(&mut self) -> bool {
        self.layout.prev_window()
    }

    /// Keeps only the active window.
    pub fn only_window(&mut self) -> bool {
        while self.window_count() > 1 {
            let ids: Vec<_> = self.layout.window_ids()
                .into_iter()
                .filter(|&id| id != self.layout.active)
                .collect();
            for id in ids {
                self.windows.remove(&id);
            }
            self.layout.root = kjxlkj_core_ui::LayoutNode::window(self.layout.active);
        }
        true
    }
}

/// Buffer info for listing.
#[derive(Debug, Clone)]
pub struct BufferInfo {
    /// Buffer ID.
    pub id: BufferId,
    /// Display name.
    pub name: String,
    /// File path.
    pub path: Option<std::path::PathBuf>,
    /// Modified flag.
    pub modified: bool,
    /// Is this the active buffer?
    pub active: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
