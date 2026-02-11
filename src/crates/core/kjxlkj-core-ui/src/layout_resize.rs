//! Layout tree resize operations.
//!
//! See /docs/spec/features/window/splits-windows.md Resize Semantics.

use kjxlkj_core_types::WindowId;

use crate::layout::{LayoutNode, LayoutTree};

/// Stored weight for proportional sizing.
/// Each leaf gets a weight; resize adjusts sibling weights.
impl LayoutTree {
    /// Equalize all sibling children at each level.
    pub fn equalize(&mut self) {
        Self::equalize_node(&mut self.root);
    }

    fn equalize_node(node: &mut LayoutNode) {
        match node {
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                for child in children.iter_mut() {
                    Self::equalize_node(child);
                }
            }
            LayoutNode::Leaf { .. } => {}
        }
        // Equalize is implicitly achieved by the equal-
        // division in compute_rects. This method is a
        // placeholder for future weighted sizing.
    }

    /// Find the parent container of `target` and return
    /// (is_horizontal, sibling_count, child_index).
    fn find_container_info(
        node: &LayoutNode,
        target: WindowId,
    ) -> Option<(bool, usize, usize)> {
        match node {
            LayoutNode::Leaf { .. } => None,
            LayoutNode::Horizontal { children } => {
                for (i, child) in children.iter().enumerate() {
                    if Self::contains_leaf(child, target) {
                        if matches!(
                            child,
                            LayoutNode::Leaf { window_id, .. }
                                if *window_id == target
                        ) {
                            return Some((true, children.len(), i));
                        }
                        let sub = Self::find_container_info(child, target);
                        if sub.is_some() { return sub; }
                    }
                }
                None
            }
            LayoutNode::Vertical { children } => {
                for (i, child) in children.iter().enumerate() {
                    if Self::contains_leaf(child, target) {
                        if matches!(
                            child,
                            LayoutNode::Leaf { window_id, .. }
                                if *window_id == target
                        ) {
                            return Some((false, children.len(), i));
                        }
                        let sub = Self::find_container_info(child, target);
                        if sub.is_some() { return sub; }
                    }
                }
                None
            }
        }
    }

    fn contains_leaf(node: &LayoutNode, target: WindowId) -> bool {
        match node {
            LayoutNode::Leaf { window_id, .. } => *window_id == target,
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                children.iter().any(|c| Self::contains_leaf(c, target))
            }
        }
    }

    /// Check if `target` is in a container that splits along the given axis.
    /// `want_horizontal` true means we want a Horizontal container (width axis).
    pub fn is_in_axis_split(&self, target: WindowId, want_horizontal: bool) -> bool {
        Self::find_container_info(&self.root, target)
            .map_or(false, |(h, cnt, _)| h == want_horizontal && cnt > 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutTree;
    use kjxlkj_core_types::{BufferId, ContentKind, Rect};

    fn buf(id: u64) -> ContentKind {
        ContentKind::Buffer(BufferId(id))
    }

    #[test]
    fn equalize_does_not_panic() {
        let mut tree = LayoutTree::single(WindowId(0), buf(0));
        tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
        tree.equalize();
        let rects = tree.compute_rects(Rect::new(0, 0, 80, 24));
        assert_eq!(rects.len(), 2);
    }

    #[test]
    fn is_in_axis_split_horizontal() {
        let mut tree = LayoutTree::single(WindowId(0), buf(0));
        tree.split_horizontal(WindowId(0), WindowId(1), buf(1));
        assert!(tree.is_in_axis_split(WindowId(0), true));
        assert!(!tree.is_in_axis_split(WindowId(0), false));
    }

    #[test]
    fn is_in_axis_split_vertical() {
        let mut tree = LayoutTree::single(WindowId(0), buf(0));
        tree.split_vertical(WindowId(0), WindowId(1), buf(1));
        assert!(tree.is_in_axis_split(WindowId(0), false));
        assert!(!tree.is_in_axis_split(WindowId(0), true));
    }
}
