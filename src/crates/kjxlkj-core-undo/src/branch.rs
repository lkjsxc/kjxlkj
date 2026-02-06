//! True undo tree with branching history and node traversal.

use std::time::Instant;

/// A unique node identifier in the undo tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// A single undo/redo change entry.
#[derive(Debug, Clone)]
pub struct ChangeEntry { pub forward: Vec<u8>, pub reverse: Vec<u8>, pub timestamp: Instant }

#[derive(Debug, Clone)]
struct TreeNode { id: NodeId, entry: ChangeEntry, parent: Option<NodeId>, children: Vec<NodeId> }

/// A tree-structured undo history supporting branching and traversal.
pub struct BranchingUndoTree {
    nodes: Vec<TreeNode>,
    current: Option<NodeId>,
    next_id: u32,
}

impl BranchingUndoTree {
    pub fn new() -> Self { Self { nodes: Vec::new(), current: None, next_id: 0 } }

    fn alloc_id(&mut self) -> NodeId { let id = NodeId(self.next_id); self.next_id += 1; id }

    fn find(&self, id: NodeId) -> Option<usize> { self.nodes.iter().position(|n| n.id == id) }

    /// Push a new change, creating a branch if we're not at the tip of the current branch.
    pub fn push(&mut self, entry: ChangeEntry) {
        let id = self.alloc_id();
        let parent = self.current;
        let node = TreeNode { id, entry, parent, children: Vec::new() };
        self.nodes.push(node);
        if let Some(pid) = parent {
            if let Some(idx) = self.find(pid) { self.nodes[idx].children.push(id); }
        }
        self.current = Some(id);
    }

    /// Undo: move to parent node, returning the current node's reverse patch.
    pub fn undo(&mut self) -> Option<&[u8]> {
        let cid = self.current?;
        let idx = self.find(cid)?;
        let parent = self.nodes[idx].parent;
        self.current = parent;
        Some(&self.nodes[idx].entry.reverse)
    }

    /// Redo: move to last child of current node, return its forward patch.
    pub fn redo(&mut self) -> Option<&[u8]> {
        let cid = match self.current {
            Some(c) => c,
            None => {
                if self.nodes.is_empty() { return None; }
                let id = self.nodes[0].id;
                self.current = Some(id);
                return Some(&self.nodes[0].entry.forward);
            }
        };
        let idx = self.find(cid)?;
        let child = *self.nodes[idx].children.last()?;
        self.current = Some(child);
        let cidx = self.find(child)?;
        Some(&self.nodes[cidx].entry.forward)
    }

    pub fn can_undo(&self) -> bool { self.current.is_some() }
    pub fn can_redo(&self) -> bool {
        self.current.map_or(!self.nodes.is_empty(), |cid| self.find(cid).map_or(false, |i| !self.nodes[i].children.is_empty()))
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn current_id(&self) -> Option<NodeId> { self.current }

    /// Count branches (children) at the current node.
    pub fn branch_count(&self) -> usize {
        match self.current {
            Some(cid) => self.find(cid).map_or(0, |i| self.nodes[i].children.len()),
            None => if self.nodes.is_empty() { 0 } else { 1 },
        }
    }

    /// Select a specific branch (child index) at the current node for redo.
    pub fn select_branch(&mut self, branch: usize) -> bool {
        let cid = match self.current { Some(c) => c, None => return false };
        let idx = match self.find(cid) { Some(i) => i, None => return false };
        if branch >= self.nodes[idx].children.len() { return false; }
        let child = self.nodes[idx].children[branch];
        // Move selected branch to last position (redo prefers last)
        self.nodes[idx].children.retain(|c| *c != child);
        self.nodes[idx].children.push(child);
        true
    }

    /// Collect all node IDs from root to current (the active path).
    pub fn path_to_current(&self) -> Vec<NodeId> {
        let mut path = Vec::new();
        let mut cur = self.current;
        while let Some(cid) = cur {
            path.push(cid);
            cur = self.find(cid).and_then(|i| self.nodes[i].parent);
        }
        path.reverse();
        path
    }
}

impl Default for BranchingUndoTree {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(fwd: &[u8], rev: &[u8]) -> ChangeEntry {
        ChangeEntry { forward: fwd.to_vec(), reverse: rev.to_vec(), timestamp: Instant::now() }
    }

    #[test]
    fn push_and_undo() {
        let mut t = BranchingUndoTree::new();
        t.push(entry(b"a", b"A"));
        t.push(entry(b"b", b"B"));
        assert_eq!(t.node_count(), 2);
        assert_eq!(t.undo(), Some(b"B".as_slice()));
        assert_eq!(t.undo(), Some(b"A".as_slice()));
        assert_eq!(t.undo(), None);
    }

    #[test]
    fn redo_follows_last_child() {
        let mut t = BranchingUndoTree::new();
        t.push(entry(b"a", b"A"));
        t.push(entry(b"b", b"B"));
        t.undo();
        t.undo();
        assert_eq!(t.redo(), Some(b"a".as_slice()));
        assert_eq!(t.redo(), Some(b"b".as_slice()));
    }

    #[test]
    fn branch_created_on_push_after_undo() {
        let mut t = BranchingUndoTree::new();
        t.push(entry(b"a", b"A"));
        t.push(entry(b"b", b"B"));
        t.undo(); // back to node "a"
        t.push(entry(b"c", b"C")); // creates branch
        assert_eq!(t.node_count(), 3);
        assert_eq!(t.branch_count(), 0); // "c" has no children
        t.undo(); // back to "a"
        assert_eq!(t.branch_count(), 2); // "a" now has "b" and "c"
    }

    #[test]
    fn select_branch() {
        let mut t = BranchingUndoTree::new();
        t.push(entry(b"a", b"A"));
        t.push(entry(b"b", b"B"));
        t.undo(); // back to a
        t.push(entry(b"c", b"C"));
        t.undo(); // back to a
        assert!(t.select_branch(0)); // select first child (b)
        let fwd = t.redo().unwrap();
        assert_eq!(fwd, b"b");
    }

    #[test]
    fn path_to_current() {
        let mut t = BranchingUndoTree::new();
        t.push(entry(b"a", b"A"));
        t.push(entry(b"b", b"B"));
        t.push(entry(b"c", b"C"));
        let path = t.path_to_current();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], NodeId(0));
        assert_eq!(path[2], NodeId(2));
    }

    #[test]
    fn can_undo_redo() {
        let mut t = BranchingUndoTree::new();
        assert!(!t.can_undo());
        assert!(!t.can_redo());
        t.push(entry(b"x", b"X"));
        assert!(t.can_undo());
        assert!(!t.can_redo());
        t.undo();
        assert!(!t.can_undo());
        assert!(t.can_redo());
    }

    #[test]
    fn empty_tree() {
        let t = BranchingUndoTree::new();
        assert_eq!(t.node_count(), 0);
        assert!(t.current_id().is_none());
    }
}
