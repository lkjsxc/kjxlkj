//! Editor state aggregation for kjxlkj editor.
//!
//! This crate aggregates all editor state and produces snapshots.

mod buffer;
mod editor;
mod registers;
mod window;

pub use buffer::BufferState;
pub use editor::EditorState;
pub use registers::Registers;
pub use window::WindowState;
