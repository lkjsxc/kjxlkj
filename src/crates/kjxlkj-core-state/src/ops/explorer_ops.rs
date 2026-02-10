//! Explorer operations on EditorState: open, split-open, file CRUD, dispatch.

use kjxlkj_core_types::Mode;
use std::path::PathBuf;

use crate::editor::EditorState;
use crate::window_tree::Window;

impl EditorState {
    /// Read selected file path (if not a directory).
    fn explorer_selected_file(&self) -> Option<PathBuf> {
        self.explorer
            .selected_entry()
            .and_then(|e| if e.is_dir { None } else { Some(e.path.clone()) })
    }
    /// Open selected explorer file in current window.
    pub fn do_explorer_open_file(&mut self) {
        let Some(path) = self.explorer_selected_file() else {
            return;
        };
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        self.open_file(path, content);
        self.mode = Mode::Normal;
    }
    /// Open in horizontal split.
    pub fn do_explorer_open_split_h(&mut self) {
        let Some(path) = self.explorer_selected_file() else {
            return;
        };
        let c = std::fs::read_to_string(&path).unwrap_or_default();
        let bid = self.buffers.open_file(path, c);
        let w = Window::new_buffer(self.windows.next_window_id(), bid);
        self.windows.active_tab_mut().split_horizontal(w);
        self.mode = Mode::Normal;
    }
    /// Open in vertical split.
    pub fn do_explorer_open_split_v(&mut self) {
        let Some(path) = self.explorer_selected_file() else {
            return;
        };
        let c = std::fs::read_to_string(&path).unwrap_or_default();
        let bid = self.buffers.open_file(path, c);
        let w = Window::new_buffer(self.windows.next_window_id(), bid);
        self.windows.active_tab_mut().split_vertical(w);
        self.mode = Mode::Normal;
    }
    /// Create a new file in the selected directory or parent.
    pub fn do_explorer_create_file(&mut self, name: &str) {
        let parent = self
            .explorer
            .selected_entry()
            .map(|e| {
                if e.is_dir {
                    e.path.clone()
                } else {
                    e.path.parent().unwrap_or(&self.explorer.root).to_path_buf()
                }
            })
            .unwrap_or_else(|| self.explorer.root.clone());
        let target = parent.join(name);
        let _ = std::fs::write(&target, "");
        self.explorer.refresh();
        self.notify_info(&format!("Created: {}", target.display()));
    }
    /// Rename selected entry.
    pub fn do_explorer_rename(&mut self, new_name: &str) {
        let Some(e) = self.explorer.selected_entry() else {
            return;
        };
        let path = e.path.clone();
        let new_path = path
            .parent()
            .map(|p| p.join(new_name))
            .unwrap_or_else(|| PathBuf::from(new_name));
        match std::fs::rename(&path, &new_path) {
            Ok(()) => {
                self.explorer.refresh();
                self.notify_info(&format!("Renamed → {}", new_path.display()));
            }
            Err(e) => self.notify_error(&format!("Rename failed: {e}")),
        }
    }
    /// Delete selected entry.
    pub fn do_explorer_delete(&mut self) {
        let Some(entry) = self.explorer.selected_entry().cloned() else {
            return;
        };
        let r = if entry.is_dir {
            std::fs::remove_dir_all(&entry.path)
        } else {
            std::fs::remove_file(&entry.path)
        };
        match r {
            Ok(()) => {
                self.explorer.refresh();
                self.notify_info(&format!("Deleted: {}", entry.path.display()));
            }
            Err(e) => self.notify_error(&format!("Delete failed: {e}")),
        }
    }
    /// Handle a key press when the explorer window is focused.
    pub fn dispatch_explorer_key(&mut self, key: &kjxlkj_core_types::Key) {
        use kjxlkj_core_types::KeyCode;
        match &key.code {
            KeyCode::Char('j') | KeyCode::Down => self.explorer.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.explorer.move_up(),
            KeyCode::Char('h') | KeyCode::Left => self.explorer.collapse_or_parent(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                if let Some(path) = self.explorer.expand_or_open() {
                    let c = std::fs::read_to_string(&path).unwrap_or_default();
                    self.open_file(path, c);
                    self.mode = Mode::Normal;
                }
            }
            KeyCode::Char('q') => self.do_explorer_toggle(),
            KeyCode::Char('R') => self.explorer.refresh(),
            _ => {}
        }
    }
}
