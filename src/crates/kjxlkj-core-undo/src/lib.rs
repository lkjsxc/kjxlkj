//! Undo/redo model.
//!
//! This crate provides a linear undo/redo stack.

mod history;

pub use history::{Edit, UndoHistory};
