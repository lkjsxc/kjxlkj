//! Buffer list management.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::BufferId;
use std::collections::HashMap;
use std::path::PathBuf;

/// Manages the ordered list of all open buffers.
pub struct BufferList {
    buffers: HashMap<BufferId, TextBuffer>,
    order: Vec<BufferId>,
    next_id: u64,
}

impl BufferList {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            order: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a new scratch buffer and return its id.
    pub fn create_scratch(&mut self) -> BufferId {
        let id = BufferId(self.next_id);
        self.next_id += 1;
        let buf = TextBuffer::new_scratch(id);
        self.buffers.insert(id, buf);
        self.order.push(id);
        id
    }

    /// Open a file into a new buffer, or return existing
    /// buffer id if already loaded.
    pub fn open_file(&mut self, path: PathBuf, content: String) -> BufferId {
        // Check if already open
        for (id, buf) in &self.buffers {
            if buf.path.as_ref() == Some(&path) {
                return *id;
            }
        }
        let id = BufferId(self.next_id);
        self.next_id += 1;
        let buf = TextBuffer::from_file(id, path, content);
        self.buffers.insert(id, buf);
        self.order.push(id);
        id
    }

    /// Get a buffer by id.
    pub fn get(&self, id: &BufferId) -> Option<&TextBuffer> {
        self.buffers.get(id)
    }

    /// Get a mutable buffer by id.
    pub fn get_mut(&mut self, id: &BufferId) -> Option<&mut TextBuffer> {
        self.buffers.get_mut(id)
    }

    /// Delete a buffer.
    pub fn delete(&mut self, id: &BufferId) -> bool {
        if self.buffers.remove(id).is_some() {
            self.order.retain(|i| i != id);
            true
        } else {
            false
        }
    }

    /// Get the next buffer id in order.
    pub fn next(&self, current: &BufferId) -> Option<BufferId> {
        let pos = self.order.iter().position(|i| i == current)?;
        let next = (pos + 1) % self.order.len();
        Some(self.order[next])
    }

    /// Get the previous buffer id in order.
    pub fn prev(&self, current: &BufferId) -> Option<BufferId> {
        let pos = self.order.iter().position(|i| i == current)?;
        let prev = if pos == 0 {
            self.order.len() - 1
        } else {
            pos - 1
        };
        Some(self.order[prev])
    }

    /// List all buffer ids in order.
    pub fn ids(&self) -> &[BufferId] {
        &self.order
    }

    /// Count of buffers.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// Check if any buffer is modified.
    pub fn any_modified(&self) -> bool {
        self.buffers.values().any(|b| b.modified)
    }

    /// Iterator over all buffers.
    pub fn iter(&self) -> impl Iterator<Item = (&BufferId, &TextBuffer)> {
        self.buffers.iter()
    }
}

impl Default for BufferList {
    fn default() -> Self {
        Self::new()
    }
}
