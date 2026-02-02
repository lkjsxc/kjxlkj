//! kjxlkj-core-state - Editor state management.
//!
//! This crate provides the central editor state.

#![allow(dead_code)]

mod buffers;
mod changelist;
mod editor;
mod jumplist;
mod marks;
mod registers;

pub use buffers::BufferStore;
pub use changelist::ChangeList;
pub use editor::EditorState;
pub use jumplist::JumpList;
pub use marks::{Mark, MarkStore};
pub use registers::RegisterStore;
