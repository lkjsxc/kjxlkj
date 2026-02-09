use kjxlkj_core_types::{BufferId, ContentSource, CursorPosition, LayoutNode, WindowId};
use kjxlkj_core_ui::{WindowArea, WindowSnapshot};
use std::collections::HashMap;

use crate::window_tree_layout::{compute_areas, remove_leaf, replace_leaf};

/// Per-window mutable state.
#[derive(Debug)]
pub struct WindowState {
    pub window_id: WindowId,
    pub content: ContentSource,
    pub cursor: CursorPosition,
    pub top_line: usize,
    pub left_col: usize,
    pub show_line_numbers: bool,
    pub wrap: bool,
    pub scrolloff: usize,
}

impl WindowState {
    pub fn new(window_id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            window_id,
            content: ContentSource::Buffer(buffer_id),
            cursor: CursorPosition::origin(),
            top_line: 0,
            left_col: 0,
            show_line_numbers: true,
            wrap: false,
            scrolloff: 5,
        }
    }

    pub fn snapshot(&self, area: WindowArea) -> WindowSnapshot {
        WindowSnapshot {
            window_id: self.window_id,
            content: self.content.clone(),
            cursor: self.cursor,
            top_line: self.top_line,
            left_col: self.left_col,
            area,
            show_line_numbers: self.show_line_numbers,
            wrap: self.wrap,
        }
    }
}

/// Window tree managing layout and window states.
#[derive(Debug)]
pub struct WindowTree {
    windows: HashMap<WindowId, WindowState>,
    layout: LayoutNode,
    focused: WindowId,
    next_id: u64,
}

impl WindowTree {
    pub fn new(initial_buffer: BufferId) -> Self {
        let wid = WindowId(1);
        let ws = WindowState::new(wid, initial_buffer);
        let mut windows = HashMap::new();
        windows.insert(wid, ws);

        Self {
            windows,
            layout: LayoutNode::Leaf(wid),
            focused: wid,
            next_id: 2,
        }
    }

    pub fn focused_id(&self) -> WindowId {
        self.focused
    }

    pub fn focused(&self) -> &WindowState {
        self.windows.get(&self.focused).expect("focused window")
    }

    pub fn focused_mut(&mut self) -> &mut WindowState {
        self.windows.get_mut(&self.focused).expect("focused window")
    }

    pub fn get(&self, id: WindowId) -> Option<&WindowState> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut WindowState> {
        self.windows.get_mut(&id)
    }

    pub fn layout(&self) -> &LayoutNode {
        &self.layout
    }

    /// Split the focused window horizontally (new window below).
    pub fn split_horizontal(&mut self, buffer_id: BufferId) -> WindowId {
        let new_id = WindowId(self.next_id);
        self.next_id += 1;
        let ws = WindowState::new(new_id, buffer_id);
        self.windows.insert(new_id, ws);

        let focused_leaf = LayoutNode::Leaf(self.focused);
        let new_leaf = LayoutNode::Leaf(new_id);
        self.layout = replace_leaf(
            &self.layout,
            self.focused,
            LayoutNode::HorizontalSplit {
                children: vec![focused_leaf, new_leaf],
                weights: vec![0.5, 0.5],
            },
        );
        self.focused = new_id;
        new_id
    }

    /// Split the focused window vertically (new window right).
    pub fn split_vertical(&mut self, buffer_id: BufferId) -> WindowId {
        let new_id = WindowId(self.next_id);
        self.next_id += 1;
        let ws = WindowState::new(new_id, buffer_id);
        self.windows.insert(new_id, ws);

        let focused_leaf = LayoutNode::Leaf(self.focused);
        let new_leaf = LayoutNode::Leaf(new_id);
        self.layout = replace_leaf(
            &self.layout,
            self.focused,
            LayoutNode::VerticalSplit {
                children: vec![focused_leaf, new_leaf],
                weights: vec![0.5, 0.5],
            },
        );
        self.focused = new_id;
        new_id
    }

    /// Close the focused window.
    pub fn close_focused(&mut self) -> bool {
        let ids = self.layout.window_ids();
        if ids.len() <= 1 {
            return false; // Cannot close last window
        }
        self.windows.remove(&self.focused);
        self.layout = remove_leaf(&self.layout, self.focused);
        let remaining = self.layout.window_ids();
        self.focused = remaining[0];
        true
    }

    /// Focus next window in the list.
    pub fn focus_next(&mut self) {
        let ids = self.layout.window_ids();
        if let Some(pos) = ids.iter().position(|id| *id == self.focused) {
            let next = (pos + 1) % ids.len();
            self.focused = ids[next];
        }
    }

    /// Focus previous window.
    pub fn focus_prev(&mut self) {
        let ids = self.layout.window_ids();
        if let Some(pos) = ids.iter().position(|id| *id == self.focused) {
            let prev = if pos == 0 { ids.len() - 1 } else { pos - 1 };
            self.focused = ids[prev];
        }
    }

    /// Build window snapshots with computed areas.
    pub fn snapshots(&self, total_cols: u16, total_rows: u16) -> HashMap<WindowId, WindowSnapshot> {
        let mut result = HashMap::new();
        compute_areas(
            &self.layout,
            0,
            0,
            total_cols,
            total_rows,
            &self.windows,
            &mut result,
        );
        result
    }
}
