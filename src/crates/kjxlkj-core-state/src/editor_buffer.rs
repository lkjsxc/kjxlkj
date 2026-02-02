//! Editor buffer management methods.

use kjxlkj_core_types::BufferId;
use std::path::PathBuf;

use crate::{BufferState, EditorState};

/// Buffer info for listing.
#[derive(Debug, Clone)]
pub struct BufferInfo {
    /// Buffer ID.
    pub id: BufferId,
    /// Display name.
    pub name: String,
    /// File path.
    pub path: Option<PathBuf>,
    /// Modified flag.
    pub modified: bool,
    /// Is this the active buffer?
    pub active: bool,
}

impl EditorState {
    /// Allocates a new buffer ID.
    pub fn alloc_buffer_id(&mut self) -> BufferId {
        let id = BufferId::new(self.next_buffer_id);
        self.next_buffer_id += 1;
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
            return false;
        }
        if !self.buffers.contains_key(&buffer_id) {
            return false;
        }
        let other_id = self.buffers.keys().find(|&&id| id != buffer_id).copied();
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
        self.buffers
            .values()
            .map(|b| BufferInfo {
                id: b.id,
                name: b.name.as_str().to_string(),
                path: b.path.clone(),
                modified: b.modified,
                active: Some(b.id) == active_buf,
            })
            .collect()
    }
}
