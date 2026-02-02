//! Editor state aggregation for kjxlkj editor.
//!
//! This crate aggregates all editor state and produces snapshots.

mod buffer;
mod clipboard;
mod editor;
mod jumplist;
mod macros;
mod marks;
mod registers;
mod window;

#[cfg(test)]
mod tests;

pub use buffer::BufferState;
pub use clipboard::{ClipboardManager, ClipboardProvider, PrimarySelection, SystemClipboard};
pub use editor::{BufferInfo, EditorState};
pub use jumplist::{ChangeList, Jump, JumpList};
pub use macros::{Macro, MacroStore, RecordingState};
pub use marks::{Mark, MarkStore, MarkType};
pub use registers::Registers;
pub use window::WindowState;
