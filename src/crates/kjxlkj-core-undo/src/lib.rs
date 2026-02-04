//! Undo/redo system for the editor.
//!
//! Provides a linear undo/redo stack with cursor position tracking.

mod history;

pub use history::{Change, UndoHistory};
