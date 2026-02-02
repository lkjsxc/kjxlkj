//! Undo/redo model for kjxlkj editor.
//!
//! This crate implements the undo tree.

mod branch;
mod history;
mod tree;
mod tree_nav;
mod undo_node;

pub use branch::{Branch, BranchId, BranchManager, TimeTravel};
pub use history::UndoHistory;
pub use tree::UndoTree;
pub use undo_node::UndoNode;
