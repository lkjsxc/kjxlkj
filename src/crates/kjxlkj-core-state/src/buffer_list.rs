use kjxlkj_core_types::BufferId;
use std::path::PathBuf;

use crate::buffer::Buffer;

/// Ordered list of all open buffers.
#[derive(Debug)]
pub struct BufferList {
    buffers: Vec<Buffer>,
    next_id: u64,
    current: usize,
    alternate: Option<usize>,
}

impl BufferList {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            next_id: 1,
            current: 0,
            alternate: None,
        }
    }

    /// Create initial scratch buffer.
    pub fn create_scratch(&mut self) -> BufferId {
        let id = BufferId(self.next_id);
        self.next_id += 1;
        self.buffers.push(Buffer::new_scratch(id));
        if self.buffers.len() == 1 {
            self.current = 0;
        }
        id
    }

    /// Open file content into a new buffer.
    pub fn open(&mut self, text: &str, path: PathBuf) -> BufferId {
        // Check if already open
        let existing_idx = self
            .buffers
            .iter()
            .position(|buf| buf.path.as_ref() == Some(&path));
        if let Some(idx) = existing_idx {
            let id = self.buffers[idx].id;
            self.set_current(idx);
            return id;
        }
        let id = BufferId(self.next_id);
        self.next_id += 1;
        self.buffers.push(Buffer::from_text(id, text, Some(path)));
        let idx = self.buffers.len() - 1;
        self.set_current(idx);
        id
    }

    fn set_current(&mut self, idx: usize) {
        if self.current != idx {
            self.alternate = Some(self.current);
        }
        self.current = idx;
    }

    pub fn current(&self) -> &Buffer {
        &self.buffers[self.current]
    }

    pub fn current_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub fn current_id(&self) -> BufferId {
        self.buffers[self.current].id
    }

    pub fn get(&self, id: BufferId) -> Option<&Buffer> {
        self.buffers.iter().find(|b| b.id == id)
    }

    pub fn get_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.id == id)
    }

    pub fn next(&mut self) {
        if !self.buffers.is_empty() {
            let idx = (self.current + 1) % self.buffers.len();
            self.set_current(idx);
        }
    }

    pub fn prev(&mut self) {
        if !self.buffers.is_empty() {
            let idx = if self.current == 0 {
                self.buffers.len() - 1
            } else {
                self.current - 1
            };
            self.set_current(idx);
        }
    }

    pub fn switch_to(&mut self, id: BufferId) -> bool {
        if let Some(idx) = self.buffers.iter().position(|b| b.id == id) {
            self.set_current(idx);
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, id: BufferId) -> Result<(), &'static str> {
        let idx = self
            .buffers
            .iter()
            .position(|b| b.id == id)
            .ok_or("Buffer not found")?;
        if self.buffers[idx].modified {
            return Err("Buffer has unsaved changes");
        }
        self.buffers.remove(idx);
        if self.buffers.is_empty() {
            self.create_scratch();
            self.current = 0;
        } else if self.current >= self.buffers.len() {
            self.current = self.buffers.len() - 1;
        }
        Ok(())
    }

    pub fn all_snapshots(
        &self,
    ) -> std::collections::HashMap<BufferId, kjxlkj_core_ui::BufferSnapshot> {
        self.buffers.iter().map(|b| (b.id, b.snapshot())).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Buffer> {
        self.buffers.iter()
    }
}

impl Default for BufferList {
    fn default() -> Self {
        Self::new()
    }
}
