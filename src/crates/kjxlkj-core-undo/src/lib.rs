//! Undo/redo model.
//!
//! Implements a linear undo stack with redo capability.

mod history;
mod operation;

pub use history::UndoHistory;
pub use operation::{EditOperation, UndoGroup};
