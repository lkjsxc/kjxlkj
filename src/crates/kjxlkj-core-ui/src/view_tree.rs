//! View tree — composable view hierarchy with focus routing.

use crate::component::{ComponentId, ComponentKind, Rect, LayoutNode, Visibility};

/// Focus target within the view tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTarget { Editor, CommandLine, Explorer, Popup(u32), Notification }

/// A view node in the focus tree.
#[derive(Debug, Clone)]
pub struct ViewNode {
    pub id: ComponentId,
    pub kind: ComponentKind,
    pub rect: Rect,
    pub focusable: bool,
    pub children: Vec<ViewNode>,
}

impl ViewNode {
    pub fn leaf(id: ComponentId, kind: ComponentKind, rect: Rect, focusable: bool) -> Self {
        Self { id, kind, rect, focusable, children: Vec::new() }
    }
    pub fn with_child(mut self, child: ViewNode) -> Self { self.children.push(child); self }
    pub fn find(&self, id: ComponentId) -> Option<&ViewNode> {
        if self.id == id { return Some(self); }
        self.children.iter().find_map(|c| c.find(id))
    }
    pub fn flatten(&self) -> Vec<LayoutNode> {
        let mut out = vec![LayoutNode::new(self.id, self.kind, self.rect)];
        for c in &self.children { out.extend(c.flatten()); }
        out
    }
}

/// The root of the view tree, managing focus state.
#[derive(Debug, Clone)]
pub struct ViewTree {
    pub root: ViewNode,
    pub focus: FocusTarget,
    pub focus_stack: Vec<FocusTarget>,
}

impl ViewTree {
    pub fn new(root: ViewNode) -> Self {
        Self { root, focus: FocusTarget::Editor, focus_stack: Vec::new() }
    }

    pub fn push_focus(&mut self, target: FocusTarget) {
        self.focus_stack.push(self.focus);
        self.focus = target;
    }

    pub fn pop_focus(&mut self) -> FocusTarget {
        let prev = self.focus;
        self.focus = self.focus_stack.pop().unwrap_or(FocusTarget::Editor);
        prev
    }

    pub fn is_focused(&self, target: FocusTarget) -> bool { self.focus == target }

    pub fn layout(&self) -> Vec<LayoutNode> { self.root.flatten() }

    /// Build from standard split: top=tabline, mid=editor, bot=status+cmdline.
    pub fn from_splits(width: u16, height: u16) -> Self {
        if height < 4 { return Self::new(ViewNode::leaf(
            ComponentId(0), ComponentKind::BufferView, Rect::new(0, 0, width, height), true)); }
        let tab = ViewNode::leaf(ComponentId(0), ComponentKind::TabLine, Rect::new(0, 0, width, 1), false);
        let buf_h = height.saturating_sub(3);
        let buf = ViewNode::leaf(ComponentId(1), ComponentKind::BufferView, Rect::new(0, 1, width, buf_h), true);
        let status = ViewNode::leaf(ComponentId(2), ComponentKind::StatusLine,
            Rect::new(0, 1 + buf_h, width, 1), false);
        let cmd = ViewNode::leaf(ComponentId(3), ComponentKind::CommandLine,
            Rect::new(0, height - 1, width, 1), true);
        let root = ViewNode { id: ComponentId(100), kind: ComponentKind::BufferView,
            rect: Rect::new(0, 0, width, height), focusable: false,
            children: vec![tab, buf, status, cmd] };
        Self::new(root)
    }

    /// Find node by id.
    pub fn find(&self, id: ComponentId) -> Option<&ViewNode> { self.root.find(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_tree_focus_stack() {
        let root = ViewNode::leaf(ComponentId(0), ComponentKind::BufferView, Rect::new(0,0,80,24), true);
        let mut tree = ViewTree::new(root);
        assert!(tree.is_focused(FocusTarget::Editor));
        tree.push_focus(FocusTarget::CommandLine);
        assert!(tree.is_focused(FocusTarget::CommandLine));
        tree.push_focus(FocusTarget::Popup(1));
        assert!(tree.is_focused(FocusTarget::Popup(1)));
        tree.pop_focus();
        assert!(tree.is_focused(FocusTarget::CommandLine));
        tree.pop_focus();
        assert!(tree.is_focused(FocusTarget::Editor));
    }

    #[test]
    fn view_tree_from_splits() {
        let tree = ViewTree::from_splits(80, 24);
        let layout = tree.layout();
        assert!(layout.len() >= 4);
    }

    #[test]
    fn flatten_nested() {
        let child = ViewNode::leaf(ComponentId(1), ComponentKind::StatusLine, Rect::new(0,23,80,1), false);
        let root = ViewNode::leaf(ComponentId(0), ComponentKind::BufferView, Rect::new(0,0,80,24), true)
            .with_child(child);
        assert_eq!(root.flatten().len(), 2);
    }

    #[test]
    fn find_node() {
        let tree = ViewTree::from_splits(80, 24);
        assert!(tree.find(ComponentId(1)).is_some());
        assert!(tree.find(ComponentId(99)).is_none());
    }

    #[test]
    fn pop_empty_stack() {
        let root = ViewNode::leaf(ComponentId(0), ComponentKind::BufferView, Rect::new(0,0,80,24), true);
        let mut tree = ViewTree::new(root);
        tree.pop_focus(); // should fallback to Editor
        assert!(tree.is_focused(FocusTarget::Editor));
    }

    #[test]
    fn small_terminal() {
        let tree = ViewTree::from_splits(80, 3);
        let layout = tree.layout();
        assert_eq!(layout.len(), 1); // just buffer view
    }
}
