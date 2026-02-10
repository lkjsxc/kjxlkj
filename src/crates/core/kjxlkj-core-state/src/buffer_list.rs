//! Buffer list management.

use crate::Buffer;
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::BufferId;
use std::path::PathBuf;

/// Buffer list.
#[derive(Debug, Default)]
pub struct BufferList {
    /// All buffers.
    buffers: Vec<Buffer>,
    /// Next buffer ID.
    next_id: u64,
}

impl BufferList {
    /// Create a new buffer list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a scratch buffer.
    pub fn add_scratch(&mut self) -> BufferId {
        let id = BufferId::new(self.next_id);
        self.next_id += 1;
        self.buffers.push(Buffer::scratch(id));
        id
    }

    /// Add a buffer with content.
    pub fn add_with_content(&mut self, content: &str) -> BufferId {
        let id = BufferId::new(self.next_id);
        self.next_id += 1;
        let mut buffer = Buffer::scratch(id);
        buffer.content = Rope::from_str(content);
        self.buffers.push(buffer);
        id
    }

    /// Add a buffer from file.
    pub fn add_from_path(&mut self, path: PathBuf, content: String) -> BufferId {
        let id = BufferId::new(self.next_id);
        self.next_id += 1;
        self.buffers.push(Buffer::from_path(id, path, content));
        id
    }

    /// Get buffer by ID.
    pub fn get(&self, id: BufferId) -> Option<&Buffer> {
        self.buffers.iter().find(|b| b.meta.id == id)
    }

    /// Get mutable buffer by ID.
    pub fn get_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.meta.id == id)
    }

    /// Remove buffer by ID.
    pub fn remove(&mut self, id: BufferId) -> bool {
        if let Some(pos) = self.buffers.iter().position(|b| b.meta.id == id) {
            self.buffers.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all buffer IDs.
    pub fn ids(&self) -> Vec<BufferId> {
        self.buffers.iter().map(|b| b.meta.id).collect()
    }

    /// Get buffer count.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }
}
