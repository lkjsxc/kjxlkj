//! Layout tree for tiled window management.
//!
//! See /docs/spec/editor/windows.md for tree model spec.

use kjxlkj_core_types::{ContentKind, Rect, WindowId};
use serde::{Deserialize, Serialize};

/// A node in the layout tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    Leaf {
        window_id: WindowId,
        content: ContentKind,
    },
    Horizontal {
        children: Vec<LayoutNode>,
    },
    Vertical {
        children: Vec<LayoutNode>,
    },
}

/// The layout tree owns the full tiled structure.
#[derive(Debug, Clone)]
pub struct LayoutTree {
    pub root: LayoutNode,
}

impl LayoutTree {
    /// Create a tree with a single leaf.
    pub fn single(
        window_id: WindowId,
        content: ContentKind,
    ) -> Self {
        Self {
            root: LayoutNode::Leaf { window_id, content },
        }
    }

    /// Compute rectangles for all leaves given total area.
    pub fn compute_rects(
        &self,
        area: Rect,
    ) -> Vec<(WindowId, ContentKind, Rect)> {
        let mut results = Vec::new();
        Self::layout_node(&self.root, area, &mut results);
        results
    }

    fn layout_node(
        node: &LayoutNode,
        area: Rect,
        out: &mut Vec<(WindowId, ContentKind, Rect)>,
    ) {
        match node {
            LayoutNode::Leaf { window_id, content } => {
                out.push((*window_id, *content, area));
            }
            LayoutNode::Horizontal { children } => {
                if children.is_empty() {
                    return;
                }
                let per_child =
                    area.width / children.len() as u16;
                let remainder =
                    area.width % children.len() as u16;
                let mut x = area.x;
                for (i, child) in children.iter().enumerate() {
                    let extra = if i < remainder as usize {
                        1
                    } else {
                        0
                    };
                    let w = per_child + extra;
                    let child_area =
                        Rect::new(x, area.y, w, area.height);
                    Self::layout_node(child, child_area, out);
                    x += w;
                }
            }
            LayoutNode::Vertical { children } => {
                if children.is_empty() {
                    return;
                }
                let per_child =
                    area.height / children.len() as u16;
                let remainder =
                    area.height % children.len() as u16;
                let mut y = area.y;
                for (i, child) in children.iter().enumerate() {
                    let extra = if i < remainder as usize {
                        1
                    } else {
                        0
                    };
                    let h = per_child + extra;
                    let child_area =
                        Rect::new(area.x, y, area.width, h);
                    Self::layout_node(child, child_area, out);
                    y += h;
                }
            }
        }
    }

    /// Collect all leaf window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        let mut ids = Vec::new();
        Self::collect_ids(&self.root, &mut ids);
        ids
    }

    fn collect_ids(
        node: &LayoutNode,
        out: &mut Vec<WindowId>,
    ) {
        match node {
            LayoutNode::Leaf { window_id, .. } => {
                out.push(*window_id);
            }
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                for child in children {
                    Self::collect_ids(child, out);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    fn buf_content(id: u64) -> ContentKind {
        ContentKind::Buffer(BufferId(id))
    }

    #[test]
    fn single_tree_rect() {
        let tree =
            LayoutTree::single(WindowId(0), buf_content(0));
        let rects =
            tree.compute_rects(Rect::new(0, 0, 80, 24));
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].2, Rect::new(0, 0, 80, 24));
    }
}
