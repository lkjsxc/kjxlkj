//! Core editor state.
//!
//! This crate provides the main EditorState type.

mod buffer;
mod editor;
mod layout;
mod window;

pub use buffer::*;
pub use editor::*;
pub use layout::*;
pub use window::*;
