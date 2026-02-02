//! kjxlkj-core-undo - Undo/redo system.
//!
//! This crate provides undo tree functionality.

mod history;
mod tree;

pub use history::UndoHistory;
pub use tree::UndoTree;
