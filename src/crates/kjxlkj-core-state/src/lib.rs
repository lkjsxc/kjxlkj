//! Editor state aggregation for kjxlkj editor.
//!
//! This crate aggregates all editor state and produces snapshots.

mod buffer;
mod editor;
mod macros;
mod registers;
mod window;

#[cfg(test)]
mod tests;

pub use buffer::BufferState;
pub use editor::{BufferInfo, EditorState};
pub use macros::{Macro, MacroStore, RecordingState};
pub use registers::Registers;
pub use window::WindowState;
