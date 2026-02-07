//! View tree for focus management and split tracking.

use crate::layout::Rect;
use serde::{Deserialize, Serialize};

/// Target of editor focus.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusTarget {
    Editor,
    CommandLine,
    Explorer,
    Popup(u64),
    Notification,
}

/// A node in the view tree representing a UI region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewNode {
    pub id: u64,
    pub kind: String,
    pub rect: Rect,
    pub focusable: bool,
    pub children: Vec<ViewNode>,
}

/// Tree of views with a focus stack for managing focus order.
#[derive(Debug, Clone)]
pub struct ViewTree {
    pub root: ViewNode,
    pub focus_stack: Vec<FocusTarget>,
}

impl ViewTree {
    /// Create a new view tree with the given root node.
    pub fn new(root: ViewNode) -> Self {
        Self {
            root,
            focus_stack: vec![FocusTarget::Editor],
        }
    }

    /// Push a new focus target onto the stack.
    pub fn push_focus(&mut self, target: FocusTarget) {
        self.focus_stack.push(target);
    }

    /// Pop the current focus target, returning to the previous one.
    /// The stack always retains at least one entry.
    pub fn pop_focus(&mut self) -> FocusTarget {
        if self.focus_stack.len() > 1 {
            self.focus_stack.pop().unwrap_or(FocusTarget::Editor)
        } else {
            self.current_focus()
        }
    }

    /// Return the current focus target.
    pub fn current_focus(&self) -> FocusTarget {
        self.focus_stack
            .last()
            .cloned()
            .unwrap_or(FocusTarget::Editor)
    }

    /// Build a view tree from a list of `(id, Rect)` pairs representing split regions.
    pub fn from_splits(rects: Vec<(u64, Rect)>) -> ViewTree {
        let children: Vec<ViewNode> = rects
            .into_iter()
            .map(|(id, rect)| ViewNode {
                id,
                kind: "split".into(),
                rect,
                focusable: true,
                children: Vec::new(),
            })
            .collect();

        let root_rect = if children.is_empty() {
            Rect::new(0, 0, 0, 0)
        } else {
            let max_x = children
                .iter()
                .map(|c| c.rect.x + c.rect.w)
                .max()
                .unwrap_or(0);
            let max_y = children
                .iter()
                .map(|c| c.rect.y + c.rect.h)
                .max()
                .unwrap_or(0);
            Rect::new(0, 0, max_x, max_y)
        };

        let root = ViewNode {
            id: 0,
            kind: "root".into(),
            rect: root_rect,
            focusable: false,
            children,
        };

        ViewTree::new(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_stack() {
        let root = ViewNode {
            id: 0,
            kind: "root".into(),
            rect: Rect::new(0, 0, 80, 24),
            focusable: false,
            children: Vec::new(),
        };
        let mut tree = ViewTree::new(root);
        assert_eq!(tree.current_focus(), FocusTarget::Editor);

        tree.push_focus(FocusTarget::CommandLine);
        assert_eq!(tree.current_focus(), FocusTarget::CommandLine);

        tree.push_focus(FocusTarget::Popup(42));
        assert_eq!(tree.current_focus(), FocusTarget::Popup(42));

        let popped = tree.pop_focus();
        assert_eq!(popped, FocusTarget::Popup(42));
        assert_eq!(tree.current_focus(), FocusTarget::CommandLine);
    }

    #[test]
    fn pop_at_bottom() {
        let root = ViewNode {
            id: 0,
            kind: "root".into(),
            rect: Rect::new(0, 0, 80, 24),
            focusable: false,
            children: Vec::new(),
        };
        let mut tree = ViewTree::new(root);
        let popped = tree.pop_focus();
        assert_eq!(popped, FocusTarget::Editor);
        assert_eq!(tree.focus_stack.len(), 1);
    }

    #[test]
    fn from_splits_builds_tree() {
        let rects = vec![(1, Rect::new(0, 0, 40, 24)), (2, Rect::new(40, 0, 40, 24))];
        let tree = ViewTree::from_splits(rects);
        assert_eq!(tree.root.children.len(), 2);
        assert_eq!(tree.root.rect.w, 80);
    }
}
