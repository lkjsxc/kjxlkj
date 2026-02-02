//! Buffer storage and management.

use std::collections::HashMap;
use kjxlkj_core_types::{
    buffer::BufferInfo,
    ids::BufferId,
};
use kjxlkj_core_text::TextBuffer;

/// Stores all open buffers.
#[derive(Debug, Default)]
pub struct BufferStore {
    /// All buffers by ID.
    buffers: HashMap<BufferId, TextBuffer>,
    /// Next buffer ID.
    next_id: u64,
}

impl BufferStore {
    /// Creates a new buffer store.
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            next_id: 1,
        }
    }

    /// Creates a new empty buffer.
    pub fn create(&mut self) -> BufferId {
        let id = BufferId::new(self.next_id);
        self.next_id += 1;
        self.buffers.insert(id, TextBuffer::new(id));
        id
    }

    /// Creates a new buffer with content.
    pub fn create_with_content(&mut self, content: &str) -> BufferId {
        let id = BufferId::new(self.next_id);
        self.next_id += 1;
        self.buffers.insert(id, TextBuffer::from_text(id, content));
        id
    }

    /// Gets a buffer by ID.
    pub fn get(&self, id: BufferId) -> Option<&TextBuffer> {
        self.buffers.get(&id)
    }

    /// Gets a mutable buffer by ID.
    pub fn get_mut(&mut self, id: BufferId) -> Option<&mut TextBuffer> {
        self.buffers.get_mut(&id)
    }

    /// Removes a buffer.
    pub fn remove(&mut self, id: BufferId) -> Option<TextBuffer> {
        self.buffers.remove(&id)
    }

    /// Returns all buffer IDs.
    pub fn ids(&self) -> impl Iterator<Item = BufferId> + '_ {
        self.buffers.keys().copied()
    }

    /// Returns buffer count.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// Returns info for all buffers.
    pub fn all_info(&self) -> Vec<BufferInfo> {
        self.buffers.values().map(|b| b.info()).collect()
    }
}
