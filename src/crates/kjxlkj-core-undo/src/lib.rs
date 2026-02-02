//! Undo/redo model for kjxlkj editor.
//!
//! This crate implements the undo tree.

mod branch;
mod history;
mod tree;

pub use branch::{Branch, BranchId, BranchManager, TimeTravel};
pub use history::UndoHistory;
pub use tree::{UndoNode, UndoTree};
