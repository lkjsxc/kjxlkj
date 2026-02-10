//! Core editor state.
//!
//! This crate provides the main EditorState type.

mod buffer;
mod buffer_list;
mod editor;
mod layout;
mod split;
mod tree;
mod window;
mod word_nav;

pub use buffer::*;
pub use buffer_list::*;
pub use editor::*;
pub use layout::*;
pub use tree::*;
pub use window::*;
