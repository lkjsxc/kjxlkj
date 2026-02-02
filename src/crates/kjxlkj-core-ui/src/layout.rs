//! Editor layout.

use crate::layout_node::{LayoutNode, SplitDirection};
use kjxlkj_core_types::WindowId;
use serde::{Deserialize, Serialize};

/// Editor layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    /// Root node.
    pub root: LayoutNode,
    /// Active window ID.
    pub active: WindowId,
}

impl Layout {
    /// Creates a new layout with a single window.
    pub fn new(window_id: WindowId) -> Self {
        Self {
            root: LayoutNode::window(window_id),
            active: window_id,
        }
    }

    /// Returns all window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        self.root.window_ids()
    }

    /// Splits the active window horizontally.
    pub fn split_horizontal(&mut self, new_id: WindowId) -> bool {
        self.root
            .split_window(self.active, new_id, SplitDirection::Horizontal)
    }

    /// Splits the active window vertically.
    pub fn split_vertical(&mut self, new_id: WindowId) -> bool {
        self.root
            .split_window(self.active, new_id, SplitDirection::Vertical)
    }

    /// Closes the active window and activates another.
    pub fn close_active(&mut self) -> bool {
        let ids = self.window_ids();
        if ids.len() <= 1 {
            return false;
        }
        let current = self.active;
        if self.root.remove_window(current) {
            // Activate another window
            let remaining = self.window_ids();
            if let Some(&next) = remaining.first() {
                self.active = next;
            }
            true
        } else {
            false
        }
    }

    /// Returns window count.
    pub fn window_count(&self) -> usize {
        self.root.window_count()
    }

    /// Sets active window.
    pub fn set_active(&mut self, id: WindowId) -> bool {
        if self.window_ids().contains(&id) {
            self.active = id;
            true
        } else {
            false
        }
    }

    /// Cycles to the next window.
    pub fn next_window(&mut self) -> bool {
        let ids = self.window_ids();
        if ids.len() <= 1 {
            return false;
        }
        let pos = ids.iter().position(|&id| id == self.active).unwrap_or(0);
        self.active = ids[(pos + 1) % ids.len()];
        true
    }

    /// Cycles to the previous window.
    pub fn prev_window(&mut self) -> bool {
        let ids = self.window_ids();
        if ids.len() <= 1 {
            return false;
        }
        let pos = ids.iter().position(|&id| id == self.active).unwrap_or(0);
        let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
        self.active = ids[prev];
        true
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new(WindowId::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_new() {
        let id = WindowId::new(1);
        let layout = Layout::new(id);
        assert_eq!(layout.active, id);
        assert_eq!(layout.window_count(), 1);
    }

    #[test]
    fn test_split_horizontal() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut layout = Layout::new(id1);

        assert!(layout.split_horizontal(id2));
        assert_eq!(layout.window_count(), 2);
    }

    #[test]
    fn test_split_vertical() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut layout = Layout::new(id1);

        assert!(layout.split_vertical(id2));
        assert_eq!(layout.window_count(), 2);
    }

    #[test]
    fn test_close_active() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut layout = Layout::new(id1);
        layout.split_horizontal(id2);

        assert!(layout.close_active());
        assert_eq!(layout.window_count(), 1);
    }

    #[test]
    fn test_cannot_close_last_window() {
        let id = WindowId::new(1);
        let mut layout = Layout::new(id);
        assert!(!layout.close_active());
    }

    #[test]
    fn test_next_prev_window() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut layout = Layout::new(id1);
        layout.split_horizontal(id2);

        let initial = layout.active;
        assert!(layout.next_window());
        assert_ne!(layout.active, initial);

        assert!(layout.prev_window());
        assert_eq!(layout.active, initial);
    }
}
