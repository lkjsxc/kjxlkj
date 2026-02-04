//! Editor state aggregation and snapshot production.
//!
//! This crate holds the complete editor state and produces snapshots for rendering.

mod editor;
mod registers;

pub use editor::EditorState;
pub use registers::Registers;
