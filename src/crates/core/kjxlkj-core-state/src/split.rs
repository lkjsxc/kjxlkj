//! Window tree split and navigation operations.

use crate::layout::LayoutNode;
use crate::tree::WindowTree;
use crate::window::Window;
use kjxlkj_core_types::{BufferId, WindowId};

impl WindowTree {
    /// Split the focused window horizontally.
    pub fn split_horizontal(&mut self, buffer_id: BufferId) -> WindowId {
        let (new_id, next_id) = self.allocate_id();
        self.set_next_id(next_id);
        let window = Window::buffer(new_id, buffer_id);
        self.insert_window(new_id, window);

        if let Some(focused_id) = self.focused_id() {
            let old_root = self.take_root();
            self.set_root(Some(LayoutNode::horizontal(vec![
                old_root.unwrap_or_else(|| LayoutNode::leaf(focused_id)),
                LayoutNode::leaf(new_id),
            ])));
        } else {
            self.set_root(Some(LayoutNode::leaf(new_id)));
        }
        self.focus(new_id);
        new_id
    }

    /// Split the focused window vertically.
    pub fn split_vertical(&mut self, buffer_id: BufferId) -> WindowId {
        let (new_id, next_id) = self.allocate_id();
        self.set_next_id(next_id);
        let window = Window::buffer(new_id, buffer_id);
        self.insert_window(new_id, window);

        if let Some(focused_id) = self.focused_id() {
            let old_root = self.take_root();
            self.set_root(Some(LayoutNode::vertical(vec![
                old_root.unwrap_or_else(|| LayoutNode::leaf(focused_id)),
                LayoutNode::leaf(new_id),
            ])));
        } else {
            self.set_root(Some(LayoutNode::leaf(new_id)));
        }
        self.focus(new_id);
        new_id
    }

    /// Close the focused window.
    pub fn close_focused(&mut self) -> bool {
        if let Some(id) = self.focused_id() {
            self.close(id)
        } else {
            false
        }
    }

    /// Close all windows except the focused one.
    pub fn close_others(&mut self) {
        if let Some(focused_id) = self.focused_id() {
            let to_close: Vec<WindowId> = self
                .window_ids()
                .into_iter()
                .filter(|&id| id != focused_id)
                .collect();
            for id in to_close {
                self.remove_window(id);
            }
            self.set_root(Some(LayoutNode::leaf(focused_id)));
        }
    }

    /// Focus the next window in ID order.
    pub fn focus_next(&mut self) {
        let ids = self.window_ids_sorted();
        if ids.is_empty() {
            return;
        }
        if let Some(focused_id) = self.focused_id() {
            let pos = ids.iter().position(|&id| id == focused_id).unwrap_or(0);
            let next = ids[(pos + 1) % ids.len()];
            self.focus(next);
        } else if let Some(&first) = ids.first() {
            self.focus(first);
        }
    }

    /// Focus the previous window in ID order.
    pub fn focus_prev(&mut self) {
        let ids = self.window_ids_sorted();
        if ids.is_empty() {
            return;
        }
        if let Some(focused_id) = self.focused_id() {
            let pos = ids.iter().position(|&id| id == focused_id).unwrap_or(0);
            let prev = ids[(pos + ids.len() - 1) % ids.len()];
            self.focus(prev);
        } else if let Some(&first) = ids.first() {
            self.focus(first);
        }
    }
}
