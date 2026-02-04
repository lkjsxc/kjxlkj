//! Editor state aggregation and snapshot production.
//!
//! This crate ties together all core subsystems.

mod editor;
mod registers;

pub use editor::EditorState;
pub use registers::RegisterStore;
