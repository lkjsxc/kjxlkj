//! EditorState buffer management methods.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, EditorError};

use crate::editor_state::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    /// Create a new buffer with the given name and text. Returns the new BufferId.
    pub fn create_buffer(&mut self, name: &str, text: &str) -> BufferId {
        let id = self.alloc_buffer_id();
        let buffer = TextBuffer::from_text(id, name.to_string(), text);
        self.buffers.insert(id, buffer);
        id
    }

    /// Open a file, creating a new buffer. Returns the BufferId.
    pub fn open_file(&mut self, path: &str) -> Result<BufferId, EditorError> {
        // Check if already open
        for (id, buf) in &self.buffers {
            if buf.path() == Some(path) {
                return Ok(*id);
            }
        }
        let content = std::fs::read_to_string(path).map_err(EditorError::from)?;
        let name = std::path::Path::new(path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());
        let id = self.alloc_buffer_id();
        let mut buffer = TextBuffer::from_text(id, name, &content);
        buffer.set_path(path.to_string());
        buffer.mark_saved();
        self.buffers.insert(id, buffer);
        Ok(id)
    }

    /// Switch the active window to display the given buffer.
    pub fn switch_buffer(&mut self, id: BufferId) {
        if self.buffers.contains_key(&id) {
            self.windows[self.active_window].buffer_id = id;
            self.windows[self.active_window].cursor = kjxlkj_core_types::Position::ZERO;
            self.windows[self.active_window].viewport_top = 0;
        }
    }

    /// Create a horizontal split with the active buffer.
    pub fn split_horizontal(&mut self) {
        let buf_id = self.active_buffer_id();
        let win_id = self.alloc_window_id();
        let (w, h) = self.terminal_size;
        let new_h = h / 2;
        let window = WindowState::new(win_id, buf_id, w, new_h);
        self.windows.push(window);
    }

    /// Create a vertical split with the active buffer.
    pub fn split_vertical(&mut self) {
        let buf_id = self.active_buffer_id();
        let win_id = self.alloc_window_id();
        let (w, h) = self.terminal_size;
        let new_w = w / 2;
        let window = WindowState::new(win_id, buf_id, new_w, h);
        self.windows.push(window);
    }

    /// Close all windows except the active one.
    pub fn close_other_windows(&mut self) {
        let active = self.windows.remove(self.active_window);
        self.windows = vec![active];
        self.active_window = 0;
    }

    /// Delete a buffer by ID. Returns error if it's the last buffer.
    pub fn delete_buffer(&mut self, id: BufferId, force: bool) -> Result<(), EditorError> {
        if let Some(buf) = self.buffers.get(&id) {
            if buf.is_modified() && !force {
                return Err(EditorError::InvalidCommand(
                    "buffer has unsaved changes (use :bd!)".into(),
                ));
            }
        }
        if self.buffers.len() <= 1 {
            return Err(EditorError::InvalidCommand(
                "cannot delete last buffer".into(),
            ));
        }
        self.buffers.remove(&id);
        // Switch windows that pointed to deleted buffer
        let fallback = *self.buffers.keys().next().unwrap();
        for win in &mut self.windows {
            if win.buffer_id == id {
                win.buffer_id = fallback;
                win.cursor = kjxlkj_core_types::Position::ZERO;
                win.viewport_top = 0;
            }
        }
        Ok(())
    }

    /// Get buffer list as formatted strings.
    pub fn buffer_list(&self) -> Vec<String> {
        let active_id = self.active_buffer_id();
        let mut list: Vec<_> = self.buffers.iter().collect();
        list.sort_by_key(|(id, _)| id.0);
        list.iter()
            .map(|(id, buf)| {
                let active = if **id == active_id { "%" } else { " " };
                let modified = if buf.is_modified() { "+" } else { " " };
                let path = buf.path().unwrap_or(buf.name());
                format!("{:>3}{active}{modified} \"{path}\"", id.0)
            })
            .collect()
    }

    /// Write the active buffer to its file path.
    pub fn write_buffer(&mut self) -> Result<String, EditorError> {
        let buf = self.active_buffer();
        let path = buf.path().map(|s| s.to_string());
        let path = path.ok_or_else(|| EditorError::InvalidCommand("no file name".into()))?;
        let text = buf.text();
        let lines = buf.line_count();
        let bytes = text.len();
        std::fs::write(&path, &text).map_err(EditorError::from)?;
        self.active_buffer_mut().mark_saved();
        Ok(format!("\"{path}\" {lines}L, {bytes}B written"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_switch_buffer() {
        let mut state = EditorState::new();
        let id = state.create_buffer("test", "hello");
        state.switch_buffer(id);
        assert_eq!(state.active_buffer().name(), "test");
    }

    #[test]
    fn buffer_list_format() {
        let state = EditorState::new();
        let list = state.buffer_list();
        assert_eq!(list.len(), 1);
        assert!(list[0].contains("[No Name]"));
    }

    #[test]
    fn delete_last_buffer_fails() {
        let mut state = EditorState::new();
        let id = state.active_buffer_id();
        assert!(state.delete_buffer(id, false).is_err());
    }
}
