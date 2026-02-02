//! kjxlkj-core-state - Editor state management.
//!
//! This crate provides the central editor state.

#![allow(dead_code)]

mod editor;
mod buffers;
mod registers;
mod marks;
mod jumplist;
mod changelist;

pub use editor::EditorState;
pub use buffers::BufferStore;
pub use registers::RegisterStore;
pub use marks::{Mark, MarkStore};
pub use jumplist::JumpList;
pub use changelist::ChangeList;
