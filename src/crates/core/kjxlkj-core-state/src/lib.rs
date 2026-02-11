mod editor;
mod editor_window;
pub mod windows;

pub use editor::{ApplyResult, EditorAction, EditorState};

#[cfg(test)]
mod editor_tests;
