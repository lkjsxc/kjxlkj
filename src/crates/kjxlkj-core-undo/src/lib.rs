/// Undo tree with persistent branches.
///
/// Each undo node stores the inverse edit operations
/// needed to revert or redo a change.
mod tree;

pub use tree::UndoTree;
