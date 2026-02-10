//! Tab page: a collection of windows with layout.

use kjxlkj_core_types::WindowId;
use kjxlkj_core_ui::{LayoutChild, LayoutNode};

use crate::focus::{find_focus, FocusDir};
use crate::window_tree::Window;

/// A tab page containing a layout tree.
pub struct TabPage {
    pub windows: Vec<Window>,
    pub layout: LayoutNode,
    pub active_window: usize,
}

impl TabPage {
    pub fn new(window: Window) -> Self {
        let layout = LayoutNode::Leaf(window.id);
        Self {
            windows: vec![window],
            layout,
            active_window: 0,
        }
    }

    /// Get the active window.
    pub fn active(&self) -> &Window {
        &self.windows[self.active_window]
    }

    /// Get the active window mutably.
    pub fn active_mut(&mut self) -> &mut Window {
        &mut self.windows[self.active_window]
    }

    /// Find a window by id.
    pub fn find(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }

    /// Find a window by id mutably.
    pub fn find_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Split the active window horizontally.
    pub fn split_horizontal(&mut self, new_window: Window) {
        let cur_id = self.windows[self.active_window].id;
        let new_id = new_window.id;
        self.windows.push(new_window);
        self.layout = LayoutNode::Horizontal(vec![
            LayoutChild {
                node: LayoutNode::Leaf(cur_id),
                weight: 1.0,
            },
            LayoutChild {
                node: LayoutNode::Leaf(new_id),
                weight: 1.0,
            },
        ]);
        self.active_window = self.windows.len() - 1;
    }

    /// Split the active window vertically.
    pub fn split_vertical(&mut self, new_window: Window) {
        let cur_id = self.windows[self.active_window].id;
        let new_id = new_window.id;
        self.windows.push(new_window);
        self.layout = LayoutNode::Vertical(vec![
            LayoutChild {
                node: LayoutNode::Leaf(cur_id),
                weight: 1.0,
            },
            LayoutChild {
                node: LayoutNode::Leaf(new_id),
                weight: 1.0,
            },
        ]);
        self.active_window = self.windows.len() - 1;
    }

    /// Close the active window, return whether tab empty.
    pub fn close_active(&mut self) -> bool {
        if self.windows.len() <= 1 {
            return true;
        }
        self.windows.remove(self.active_window);
        if self.active_window >= self.windows.len() {
            self.active_window = self.windows.len() - 1;
        }
        self.rebuild_layout();
        false
    }

    /// Navigate to next window.
    pub fn next_window(&mut self) {
        if !self.windows.is_empty() {
            self.active_window = (self.active_window + 1) % self.windows.len();
        }
    }

    /// Navigate to previous window.
    pub fn prev_window(&mut self) {
        if !self.windows.is_empty() {
            self.active_window = if self.active_window == 0 {
                self.windows.len() - 1
            } else {
                self.active_window - 1
            };
        }
    }

    /// Navigate to nearest window in the given direction.
    pub fn focus_direction(&mut self, dir: FocusDir) {
        if self.windows.len() <= 1 {
            return;
        }
        let cur_id = self.windows[self.active_window].id;
        if let Some(target) = find_focus(&self.layout, cur_id, dir) {
            if let Some(idx) = self.windows.iter().position(|w| w.id == target) {
                self.active_window = idx;
            }
        }
    }

    fn rebuild_layout(&mut self) {
        if self.windows.len() == 1 {
            self.layout = LayoutNode::Leaf(self.windows[0].id);
        } else {
            let children: Vec<LayoutChild> = self
                .windows
                .iter()
                .map(|w| LayoutChild {
                    node: LayoutNode::Leaf(w.id),
                    weight: 1.0,
                })
                .collect();
            self.layout = LayoutNode::Horizontal(children);
        }
    }

    /// Public layout rebuild for cross-module access.
    pub fn rebuild_layout_pub(&mut self) {
        self.rebuild_layout();
    }
}
