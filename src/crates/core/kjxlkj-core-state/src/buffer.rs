//! Buffer state management.

use kjxlkj_core_text::{Rope, empty_rope, line_content, line_grapheme_count};
use kjxlkj_core_types::{BufferId, BufferMeta, BufferVersion, CursorPosition};
use kjxlkj_core_undo::UndoHistory;
use std::path::PathBuf;

/// Buffer state.
#[derive(Debug)]
pub struct Buffer {
    /// Buffer metadata.
    pub meta: BufferMeta,
    /// Rope content.
    pub content: Rope,
    /// Undo history.
    pub undo: UndoHistory,
}

impl Buffer {
    /// Create a new scratch buffer.
    pub fn scratch(id: BufferId) -> Self {
        Self {
            meta: BufferMeta::scratch(id),
            content: empty_rope(),
            undo: UndoHistory::new(),
        }
    }

    /// Create a buffer from file path.
    pub fn from_path(id: BufferId, path: PathBuf, content: String) -> Self {
        Self {
            meta: BufferMeta::from_path(id, path),
            content: Rope::from_str(&content),
            undo: UndoHistory::new(),
        }
    }

    /// Get line count.
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get line content.
    pub fn line(&self, idx: usize) -> String {
        line_content(&self.content, idx)
    }

    /// Get grapheme count for a line.
    pub fn line_grapheme_count(&self, idx: usize) -> usize {
        line_grapheme_count(&self.content, idx)
    }

    /// Insert text at position.
    pub fn insert(&mut self, pos: CursorPosition, text: &str) {
        let byte_offset = kjxlkj_core_text::position_to_byte(
            &self.content,
            pos.line,
            pos.grapheme,
        );
        self.content.insert(byte_offset, text);
        self.meta.version.increment();
        self.meta.modified = true;
    }

    /// Delete range.
    pub fn delete(&mut self, start: CursorPosition, end: CursorPosition) {
        kjxlkj_core_text::delete_range(
            &mut self.content,
            start.line,
            start.grapheme,
            end.line,
            end.grapheme,
        );
        self.meta.version.increment();
        self.meta.modified = true;
    }

    /// Mark as saved.
    pub fn mark_saved(&mut self) {
        self.meta.modified = false;
    }

    /// Get buffer version.
    pub fn version(&self) -> BufferVersion {
        self.meta.version
    }
}

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
