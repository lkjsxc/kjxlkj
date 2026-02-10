//! Window tree management.

use crate::layout::LayoutNode;
use crate::window::Window;
use kjxlkj_core_types::{BufferId, ExplorerStateId, TerminalId, WindowId};

/// Window tree for a tab.
#[derive(Debug)]
pub struct WindowTree {
    /// Windows by ID.
    pub(crate) windows: std::collections::HashMap<WindowId, Window>,
    /// Layout tree root.
    pub(crate) root: Option<LayoutNode>,
    /// Focused window ID.
    pub(crate) focused: Option<WindowId>,
    /// Next window ID.
    pub(crate) next_id: u64,
    /// Focus sequence counter.
    pub(crate) focus_seq: u64,
}

impl Default for WindowTree {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowTree {
    /// Create a new window tree.
    pub fn new() -> Self {
        Self {
            windows: std::collections::HashMap::new(),
            root: None,
            focused: None,
            next_id: 0,
            focus_seq: 0,
        }
    }

    /// Add a buffer window.
    pub fn add_buffer_window(&mut self, buffer_id: BufferId) -> WindowId {
        let id = WindowId::new(self.next_id);
        self.next_id += 1;
        let window = Window::buffer(id, buffer_id);
        self.windows.insert(id, window);
        self.set_root_if_empty(id);
        id
    }

    /// Add a terminal window.
    pub fn add_terminal_window(&mut self, terminal_id: TerminalId) -> WindowId {
        let id = WindowId::new(self.next_id);
        self.next_id += 1;
        let window = Window::terminal(id, terminal_id);
        self.windows.insert(id, window);
        self.set_root_if_empty(id);
        id
    }

    /// Add an explorer window.
    pub fn add_explorer_window(&mut self, state_id: ExplorerStateId) -> WindowId {
        let id = WindowId::new(self.next_id);
        self.next_id += 1;
        let window = Window::explorer(id, state_id);
        self.windows.insert(id, window);
        self.set_root_if_empty(id);
        id
    }

    pub(crate) fn set_root_if_empty(&mut self, id: WindowId) {
        if self.root.is_none() {
            self.root = Some(LayoutNode::leaf(id));
            self.focus(id);
        }
    }

    /// Focus a window.
    pub fn focus(&mut self, id: WindowId) {
        if self.windows.contains_key(&id) {
            self.focus_seq += 1;
            if let Some(window) = self.windows.get_mut(&id) {
                window.last_focus_seq = self.focus_seq;
            }
            self.focused = Some(id);
        }
    }

    /// Get focused window ID.
    pub fn focused_id(&self) -> Option<WindowId> {
        self.focused
    }

    /// Get focused window.
    pub fn focused(&self) -> Option<&Window> {
        self.focused.and_then(|id| self.windows.get(&id))
    }

    /// Get focused window mutably.
    pub fn focused_mut(&mut self) -> Option<&mut Window> {
        self.focused.and_then(|id| self.windows.get_mut(&id))
    }

    /// Get window by ID.
    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    /// Get window by ID mutably.
    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    /// Get the layout root.
    pub fn layout(&self) -> Option<&LayoutNode> {
        self.root.as_ref()
    }

    /// Get all window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        self.windows.keys().copied().collect()
    }

    /// Get sorted window IDs.
    pub fn window_ids_sorted(&self) -> Vec<WindowId> {
        let mut ids: Vec<WindowId> = self.windows.keys().copied().collect();
        ids.sort();
        ids
    }

    /// Close a window.
    pub fn close(&mut self, id: WindowId) -> bool {
        if self.windows.remove(&id).is_some() {
            if self.focused == Some(id) {
                self.focused = self.windows.keys().next().copied();
            }
            true
        } else {
            false
        }
    }

    /// Count windows.
    pub fn count(&self) -> usize {
        self.windows.len()
    }

    // Helper methods for split.rs
    pub(crate) fn allocate_id(&self) -> (WindowId, u64) {
        (WindowId::new(self.next_id), self.next_id + 1)
    }

    pub(crate) fn set_next_id(&mut self, id: u64) {
        self.next_id = id;
    }

    pub(crate) fn insert_window(&mut self, id: WindowId, window: Window) {
        self.windows.insert(id, window);
    }

    pub(crate) fn remove_window(&mut self, id: WindowId) -> Option<Window> {
        self.windows.remove(&id)
    }

    pub(crate) fn take_root(&mut self) -> Option<LayoutNode> {
        self.root.take()
    }

    pub(crate) fn set_root(&mut self, root: Option<LayoutNode>) {
        self.root = root;
    }
}
