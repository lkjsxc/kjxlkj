//! Layout tree split and close operations.
//!
//! See /docs/spec/editor/windows.md for tree model spec.

use kjxlkj_core_types::{ContentKind, WindowId};

use crate::layout::{LayoutNode, LayoutTree};

impl LayoutTree {
    /// Split the given window horizontally. Returns true on success.
    pub fn split_horizontal(
        &mut self,
        target: WindowId,
        new_id: WindowId,
        new_content: ContentKind,
    ) -> bool {
        Self::split_node(
            &mut self.root,
            target,
            new_id,
            new_content,
            true,
        )
    }

    /// Split the given window vertically.
    pub fn split_vertical(
        &mut self,
        target: WindowId,
        new_id: WindowId,
        new_content: ContentKind,
    ) -> bool {
        Self::split_node(
            &mut self.root,
            target,
            new_id,
            new_content,
            false,
        )
    }

    fn split_node(
        node: &mut LayoutNode,
        target: WindowId,
        new_id: WindowId,
        new_content: ContentKind,
        horizontal: bool,
    ) -> bool {
        match node {
            LayoutNode::Leaf { window_id, content } => {
                if *window_id == target {
                    let old_leaf = LayoutNode::Leaf {
                        window_id: *window_id,
                        content: *content,
                    };
                    let new_leaf = LayoutNode::Leaf {
                        window_id: new_id,
                        content: new_content,
                    };
                    *node = if horizontal {
                        LayoutNode::Horizontal {
                            children: vec![old_leaf, new_leaf],
                        }
                    } else {
                        LayoutNode::Vertical {
                            children: vec![old_leaf, new_leaf],
                        }
                    };
                    true
                } else {
                    false
                }
            }
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                for child in children {
                    if Self::split_node(
                        child,
                        target,
                        new_id,
                        new_content,
                        horizontal,
                    ) {
                        return true;
                    }
                }
                false
            }
        }
    }

    /// Close a window leaf and collapse unary containers.
    pub fn close_window(
        &mut self,
        target: WindowId,
    ) -> bool {
        if self.window_ids().len() <= 1 {
            return false;
        }
        let removed =
            Self::remove_leaf(&mut self.root, target);
        if removed {
            Self::collapse_unary(&mut self.root);
        }
        removed
    }

    fn remove_leaf(
        node: &mut LayoutNode,
        target: WindowId,
    ) -> bool {
        match node {
            LayoutNode::Leaf { .. } => false,
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                let before = children.len();
                children.retain(|child| {
                    !matches!(
                        child,
                        LayoutNode::Leaf { window_id, .. }
                            if *window_id == target
                    )
                });
                if children.len() < before {
                    return true;
                }
                for child in children {
                    if Self::remove_leaf(child, target) {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn collapse_unary(node: &mut LayoutNode) {
        match node {
            LayoutNode::Horizontal { children }
            | LayoutNode::Vertical { children } => {
                for child in children.iter_mut() {
                    Self::collapse_unary(child);
                }
                if children.len() == 1 {
                    let only = children.remove(0);
                    *node = only;
                }
            }
            LayoutNode::Leaf { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, Rect};

    fn buf_content(id: u64) -> ContentKind {
        ContentKind::Buffer(BufferId(id))
    }

    #[test]
    fn horizontal_split() {
        let mut tree =
            LayoutTree::single(WindowId(0), buf_content(0));
        tree.split_horizontal(
            WindowId(0),
            WindowId(1),
            buf_content(1),
        );
        let rects =
            tree.compute_rects(Rect::new(0, 0, 80, 24));
        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].2.width, 40);
        assert_eq!(rects[1].2.width, 40);
    }

    #[test]
    fn close_window_collapses() {
        let mut tree =
            LayoutTree::single(WindowId(0), buf_content(0));
        tree.split_horizontal(
            WindowId(0),
            WindowId(1),
            buf_content(1),
        );
        assert!(tree.close_window(WindowId(1)));
        assert_eq!(tree.window_ids().len(), 1);
    }
}
