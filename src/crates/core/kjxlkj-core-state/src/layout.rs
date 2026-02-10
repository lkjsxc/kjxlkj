//! Layout tree types.

use kjxlkj_core_types::{SplitDirection, WindowId};

/// Layout tree node.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// Leaf window.
    Leaf(WindowId),
    /// Container with split direction.
    Container {
        direction: SplitDirection,
        children: Vec<LayoutNode>,
    },
}

impl LayoutNode {
    /// Create a leaf node.
    pub fn leaf(id: WindowId) -> Self {
        LayoutNode::Leaf(id)
    }

    /// Create a horizontal container.
    pub fn horizontal(children: Vec<LayoutNode>) -> Self {
        LayoutNode::Container {
            direction: SplitDirection::Horizontal,
            children,
        }
    }

    /// Create a vertical container.
    pub fn vertical(children: Vec<LayoutNode>) -> Self {
        LayoutNode::Container {
            direction: SplitDirection::Vertical,
            children,
        }
    }
}
