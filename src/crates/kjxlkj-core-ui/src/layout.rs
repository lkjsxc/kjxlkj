//! Layout types.

use kjxlkj_core_types::WindowId;
use serde::{Deserialize, Serialize};

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Horizontal split (windows stacked vertically).
    Horizontal,
    /// Vertical split (windows side by side).
    Vertical,
}

/// Layout tree node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    /// Leaf window.
    Window(WindowId),
    /// Split container.
    Split {
        /// Split direction.
        direction: SplitDirection,
        /// Child nodes.
        children: Vec<LayoutNode>,
        /// Split ratios (0.0 to 1.0).
        ratios: Vec<f32>,
    },
}

impl LayoutNode {
    /// Creates a window leaf.
    pub fn window(id: WindowId) -> Self {
        Self::Window(id)
    }

    /// Creates a horizontal split.
    pub fn hsplit(children: Vec<LayoutNode>) -> Self {
        let n = children.len();
        Self::Split {
            direction: SplitDirection::Horizontal,
            children,
            ratios: vec![1.0 / n as f32; n],
        }
    }

    /// Creates a vertical split.
    pub fn vsplit(children: Vec<LayoutNode>) -> Self {
        let n = children.len();
        Self::Split {
            direction: SplitDirection::Vertical,
            children,
            ratios: vec![1.0 / n as f32; n],
        }
    }

    /// Collects all window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        match self {
            LayoutNode::Window(id) => vec![*id],
            LayoutNode::Split { children, .. } => {
                children.iter().flat_map(|c| c.window_ids()).collect()
            }
        }
    }
}

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
}

impl Default for Layout {
    fn default() -> Self {
        Self::new(WindowId::default())
    }
}
