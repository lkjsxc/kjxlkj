//! Window types.

use super::{BufferId, CursorState, Selection, ViewportState, WindowId};
use serde::{Deserialize, Serialize};

/// Window state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window identifier.
    pub id: WindowId,
    /// Buffer being displayed.
    pub buffer_id: BufferId,
    /// Cursor state.
    pub cursor: CursorState,
    /// Viewport state.
    pub viewport: ViewportState,
    /// Visual selection (if in visual mode).
    pub selection: Option<Selection>,
    /// Show line numbers.
    pub number: bool,
    /// Show relative line numbers.
    pub relative_number: bool,
}

impl WindowState {
    /// Create a new window state.
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor: CursorState::default(),
            viewport: ViewportState::default(),
            selection: None,
            number: true,
            relative_number: false,
        }
    }
}

/// Window layout node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    /// Leaf window.
    Window(WindowId),
    /// Horizontal split.
    Horizontal(Vec<LayoutNode>),
    /// Vertical split.
    Vertical(Vec<LayoutNode>),
}

impl LayoutNode {
    /// Find all window IDs in this layout.
    pub fn window_ids(&self) -> Vec<WindowId> {
        match self {
            LayoutNode::Window(id) => vec![*id],
            LayoutNode::Horizontal(children) | LayoutNode::Vertical(children) => {
                children.iter().flat_map(|c| c.window_ids()).collect()
            }
        }
    }
}
