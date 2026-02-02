//! Undo tree navigation methods.

use kjxlkj_core_edit::Transaction;

/// Navigation extension for undo trees.
pub trait UndoTreeNav {
    /// Returns all branch IDs at current node.
    fn branches(&self) -> &[usize];

    /// Switches to a specific branch by child index.
    fn switch_branch(&mut self, index: usize) -> Option<&Transaction>;

    /// Switches to the previous (older) branch.
    fn prev_branch(&mut self) -> Option<&Transaction>;

    /// Switches to the next (newer) branch.
    fn next_branch(&mut self) -> Option<&Transaction>;

    /// Returns path from root to current node.
    fn path_to_current(&self) -> Vec<usize>;

    /// Goes to a specific node by ID.
    fn goto_node(&mut self, id: usize) -> Option<&Transaction>;
}

#[cfg(test)]
mod tests {
    use crate::UndoTree;
    use kjxlkj_core_edit::Transaction;

    #[test]
    fn test_branching() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.undo();
        tree.push(Transaction::default());
        assert_eq!(tree.branches().len(), 0);
        tree.undo();
        assert_eq!(tree.branches().len(), 2);
    }

    #[test]
    fn test_switch_branch() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.undo();
        tree.push(Transaction::default());
        tree.undo();
        tree.switch_branch(0);
        assert_eq!(tree.current_id(), 1);
    }

    #[test]
    fn test_path_to_current() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.push(Transaction::default());
        let path = tree.path_to_current();
        assert_eq!(path, vec![0, 1, 2]);
    }

    #[test]
    fn test_goto_node() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.push(Transaction::default());
        tree.goto_node(1);
        assert_eq!(tree.current_id(), 1);
    }

    #[test]
    fn test_prev_next_branch() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.undo();
        tree.push(Transaction::default());
        tree.undo();
        tree.switch_branch(1);
        assert!(tree.prev_branch().is_some());
        assert!(tree.next_branch().is_some());
    }
}
