//! Undo/redo model for kjxlkj editor.
//!
//! This crate implements the undo tree.

mod history;
mod tree;

pub use history::UndoHistory;
pub use tree::{UndoNode, UndoTree};
