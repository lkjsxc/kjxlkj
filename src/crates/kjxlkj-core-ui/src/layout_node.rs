//! Layout node types.

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

    /// Finds a window and replaces it with a split.
    pub fn split_window(
        &mut self,
        target: WindowId,
        new_id: WindowId,
        direction: SplitDirection,
    ) -> bool {
        match self {
            LayoutNode::Window(id) if *id == target => {
                let existing = LayoutNode::Window(target);
                let new_node = LayoutNode::Window(new_id);
                *self = LayoutNode::Split {
                    direction,
                    children: vec![existing, new_node],
                    ratios: vec![0.5, 0.5],
                };
                true
            }
            LayoutNode::Split { children, .. } => children
                .iter_mut()
                .any(|c| c.split_window(target, new_id, direction)),
            _ => false,
        }
    }

    /// Removes a window from the layout.
    pub fn remove_window(&mut self, target: WindowId) -> bool {
        if let LayoutNode::Split {
            children, ratios, ..
        } = self
        {
            // Find and remove the target
            if let Some(pos) = children
                .iter()
                .position(|c| matches!(c, LayoutNode::Window(id) if *id == target))
            {
                children.remove(pos);
                ratios.remove(pos);
                // Normalize ratios
                let total: f32 = ratios.iter().sum();
                if total > 0.0 {
                    for r in ratios.iter_mut() {
                        *r /= total;
                    }
                }
                return true;
            }
            // Recurse
            for child in children.iter_mut() {
                if child.remove_window(target) {
                    return true;
                }
            }
        }
        false
    }

    /// Returns window count.
    pub fn window_count(&self) -> usize {
        self.window_ids().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_node_window() {
        let id = WindowId::new(1);
        let node = LayoutNode::window(id);
        assert_eq!(node.window_ids(), vec![id]);
    }

    #[test]
    fn test_layout_node_hsplit() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let node = LayoutNode::hsplit(vec![LayoutNode::window(id1), LayoutNode::window(id2)]);
        assert_eq!(node.window_count(), 2);
    }

    #[test]
    fn test_layout_node_vsplit() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let node = LayoutNode::vsplit(vec![LayoutNode::window(id1), LayoutNode::window(id2)]);
        assert_eq!(node.window_count(), 2);
    }

    #[test]
    fn test_split_window() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut node = LayoutNode::window(id1);

        assert!(node.split_window(id1, id2, SplitDirection::Horizontal));
        assert_eq!(node.window_count(), 2);
    }

    #[test]
    fn test_remove_window() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(2);
        let mut node = LayoutNode::hsplit(vec![LayoutNode::window(id1), LayoutNode::window(id2)]);

        assert!(node.remove_window(id1));
        assert_eq!(node.window_count(), 1);
    }
}
