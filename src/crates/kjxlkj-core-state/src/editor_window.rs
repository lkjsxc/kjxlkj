//! Editor window management methods.

use kjxlkj_core_types::WindowId;

use crate::{EditorState, WindowState};

impl EditorState {
    /// Allocates a new window ID.
    pub fn alloc_window_id(&mut self) -> WindowId {
        let id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;
        id
    }

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
            let ids: Vec<_> = self
                .layout
                .window_ids()
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
