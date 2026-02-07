//! kjxlkj-core-undo: undo/redo model and window state types.

pub mod branching;
pub mod undo_tree;
pub mod window_full;

pub use branching::{BranchNode, BranchingUndoTree, ChangeEntry, NodeId};
pub use undo_tree::{ChangeKind, TextChange, UndoEntry, UndoTree};
pub use window_full::{
    can_close, CloseGuard, SignColumn, WindowOptionStore, WindowOptions, WindowSnapshot,
};
