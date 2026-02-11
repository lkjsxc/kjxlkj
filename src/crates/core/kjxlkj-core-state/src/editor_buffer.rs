//! Buffer management operations: switch, next/prev, delete, open, alternate.
//! See /docs/spec/features/buffer/ and /docs/spec/editor/buffers.md.

use std::path::PathBuf;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{BufferId, ContentKind};
use crate::editor::EditorState;

impl EditorState {
    /// Switch the focused window to display a different buffer.
    pub(crate) fn switch_to_buffer(&mut self, buf_id: BufferId) {
        if !self.buffers.contains_key(&buf_id) { return; }
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            // Record current buffer as alternate before switching.
            if let ContentKind::Buffer(old_id) = win.content {
                if old_id != buf_id {
                    self.alternate_buffer = Some(old_id);
                }
            }
            win.content = ContentKind::Buffer(buf_id);
            win.cursor = kjxlkj_core_edit::Cursor::default();
        }
    }

    /// Switch to next buffer in buffer list order.
    pub(crate) fn next_buffer(&mut self) {
        let cur = self.current_buffer_id();
        let ids = self.sorted_buffer_ids();
        if ids.len() <= 1 { return; }
        let pos = ids.iter().position(|&id| id == cur).unwrap_or(0);
        let next = ids[(pos + 1) % ids.len()];
        self.switch_to_buffer(next);
    }

    /// Switch to previous buffer in buffer list order.
    pub(crate) fn prev_buffer(&mut self) {
        let cur = self.current_buffer_id();
        let ids = self.sorted_buffer_ids();
        if ids.len() <= 1 { return; }
        let pos = ids.iter().position(|&id| id == cur).unwrap_or(0);
        let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
        self.switch_to_buffer(ids[prev]);
    }

    /// Delete (unload) current buffer. Switches to alternate or next.
    pub(crate) fn delete_buffer(&mut self) {
        let cur = self.current_buffer_id();
        let ids = self.sorted_buffer_ids();
        if ids.len() <= 1 { return; } // Don't delete the last buffer.
        // Find fallback buffer to switch to.
        let fallback = self.alternate_buffer
            .filter(|id| *id != cur && self.buffers.contains_key(id))
            .unwrap_or_else(|| {
                let pos = ids.iter().position(|&id| id == cur).unwrap_or(0);
                if pos + 1 < ids.len() { ids[pos + 1] } else { ids[0] }
            });
        // Switch all windows showing this buffer to fallback.
        for win in self.windows.values_mut() {
            if win.content == ContentKind::Buffer(cur) {
                win.content = ContentKind::Buffer(fallback);
                win.cursor = kjxlkj_core_edit::Cursor::default();
            }
        }
        self.buffers.remove(&cur);
        if self.alternate_buffer == Some(cur) {
            self.alternate_buffer = None;
        }
    }

    /// Switch to the alternate buffer (Ctrl-^).
    pub(crate) fn switch_alternate(&mut self) {
        if let Some(alt_id) = self.alternate_buffer {
            if self.buffers.contains_key(&alt_id) {
                self.switch_to_buffer(alt_id);
            }
        }
    }

    /// Open a file path into a new buffer and switch to it.
    pub(crate) fn open_file(&mut self, path: &str) {
        // Check if already open.
        let target = PathBuf::from(path);
        for (&id, buf) in &self.buffers {
            if buf.path.as_ref() == Some(&target) {
                self.switch_to_buffer(id);
                return;
            }
        }
        // Create new buffer (load content if possible).
        let buf_id = BufferId(self.next_id());
        let name = target.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let mut buf = Buffer::from_text(buf_id, &name, &content);
        buf.path = Some(target);
        buf.modified = false;
        self.buffers.insert(buf_id, buf);
        self.switch_to_buffer(buf_id);
    }

    /// Get sorted buffer IDs for deterministic ordering.
    fn sorted_buffer_ids(&self) -> Vec<BufferId> {
        let mut ids: Vec<BufferId> = self.buffers.keys().copied().collect();
        ids.sort_by_key(|id| id.0);
        ids
    }

    /// Get current buffer ID from focused window.
    fn current_buffer_id(&self) -> BufferId {
        let wid = self.focus.focused;
        match self.windows.get(&wid).map(|w| w.content) {
            Some(ContentKind::Buffer(id)) => id,
            _ => BufferId(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::EditorState;

    #[test]
    fn next_prev_buffer_cycles() {
        let mut s = EditorState::new(80, 24);
        let id1 = BufferId(s.next_id());
        let buf1 = Buffer::new_scratch(id1);
        s.buffers.insert(id1, buf1);
        // Initially on buffer 0, next goes to id1.
        s.next_buffer();
        assert_eq!(s.current_buffer_id(), id1);
        // Next wraps around to buffer 0.
        s.next_buffer();
        assert_eq!(s.current_buffer_id(), BufferId(0));
        // Prev goes back to id1.
        s.prev_buffer();
        assert_eq!(s.current_buffer_id(), id1);
    }

    #[test]
    fn switch_sets_alternate() {
        let mut s = EditorState::new(80, 24);
        let id1 = BufferId(s.next_id());
        s.buffers.insert(id1, Buffer::new_scratch(id1));
        s.switch_to_buffer(id1);
        assert_eq!(s.alternate_buffer, Some(BufferId(0)));
    }

    #[test]
    fn delete_buffer_switches_to_fallback() {
        let mut s = EditorState::new(80, 24);
        let id1 = BufferId(s.next_id());
        s.buffers.insert(id1, Buffer::new_scratch(id1));
        s.switch_to_buffer(id1);
        s.delete_buffer();
        assert_eq!(s.current_buffer_id(), BufferId(0));
        assert!(!s.buffers.contains_key(&id1));
    }

    #[test]
    fn switch_alternate_toggles() {
        let mut s = EditorState::new(80, 24);
        let id1 = BufferId(s.next_id());
        s.buffers.insert(id1, Buffer::new_scratch(id1));
        s.switch_to_buffer(id1);
        assert_eq!(s.alternate_buffer, Some(BufferId(0)));
        s.switch_alternate();
        assert_eq!(s.current_buffer_id(), BufferId(0));
        // Alternate should now be id1 (the one we just left).
        assert_eq!(s.alternate_buffer, Some(id1));
    }

    #[test]
    fn delete_last_buffer_is_noop() {
        let mut s = EditorState::new(80, 24);
        assert_eq!(s.buffers.len(), 1);
        s.delete_buffer();
        assert_eq!(s.buffers.len(), 1);
    }
}
