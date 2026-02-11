mod focus;
mod node_ops;

use std::collections::HashMap;

use node_ops::{remove_leaf, replace_leaf, Node};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Down,
    Up,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Buffer,
    Explorer,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub row: u16,
    pub col: u16,
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Clone)]
pub struct WindowTree {
    root: Node,
    kinds: HashMap<u64, WindowKind>,
    focus_seq: HashMap<u64, u64>,
    focused: u64,
    previous: Option<u64>,
    seq: u64,
    next_id: u64,
}

impl WindowTree {
    pub fn new() -> Self {
        let mut kinds = HashMap::new();
        kinds.insert(1, WindowKind::Buffer);
        let mut focus_seq = HashMap::new();
        focus_seq.insert(1, 1);
        Self {
            root: Node::Leaf(1),
            kinds,
            focus_seq,
            focused: 1,
            previous: None,
            seq: 1,
            next_id: 2,
        }
    }

    pub fn focused(&self) -> u64 {
        self.focused
    }

    pub fn split_focused(&mut self, axis: Axis, kind: WindowKind) -> u64 {
        let old = self.focused;
        let new_id = self.next_id;
        self.next_id += 1;
        let replacement = Node::Split(
            axis,
            Box::new(Node::Leaf(old)),
            Box::new(Node::Leaf(new_id)),
        );
        self.root = replace_leaf(&self.root, old, &replacement).expect("target leaf must exist");
        self.kinds.insert(new_id, kind);
        self.focus_window(new_id);
        new_id
    }

    pub fn close_focused(&mut self) -> bool {
        if self.leaf_ids().len() <= 1 {
            return false;
        }
        let closing = self.focused;
        self.root = remove_leaf(&self.root, closing).expect("multi-leaf tree keeps root");
        self.kinds.remove(&closing);
        self.focus_seq.remove(&closing);
        let fallback = self
            .previous
            .filter(|id| self.kinds.contains_key(id))
            .unwrap_or_else(|| self.leaf_ids()[0]);
        self.focus_window(fallback);
        true
    }

    pub fn only(&mut self) {
        self.root = Node::Leaf(self.focused);
        self.kinds.retain(|id, _| *id == self.focused);
        self.focus_seq.retain(|id, _| *id == self.focused);
        self.previous = None;
    }

    pub fn cycle_next(&mut self) {
        self.cycle(true);
    }

    pub fn cycle_prev(&mut self) {
        self.cycle(false);
    }

    pub fn focus_previous(&mut self) {
        if let Some(prev) = self.previous.filter(|id| self.kinds.contains_key(id)) {
            self.focus_window(prev);
        }
    }
}

impl Default for WindowTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
