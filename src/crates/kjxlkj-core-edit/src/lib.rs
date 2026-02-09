//! Editing primitives: motions, operators, text objects, and register operations.
//!
//! This crate implements the core editing engine as specified in
//! /docs/spec/editing/README.md.

mod char_find;
mod clipboard;
mod cursor;
mod motion_exec;
mod motion_extended;
mod motion_helpers;
mod motion_line;
mod motion_search;
#[cfg(test)]
mod motion_tests;
mod operator_exec;
mod operator_helpers;
#[cfg(test)]
mod operator_tests;
mod registers;
mod surround;
mod text_manip;
mod text_object_argument;
mod text_object_bracket;
mod text_object_delimited;
mod text_object_exec;
mod text_object_tag;

pub use char_find::{CharFind, CharFindKind};
pub use clipboard::{clipboard_get, clipboard_set, detect_provider, ClipboardProvider};
pub use cursor::CursorPosition;
pub use motion_exec::execute_motion;
pub use operator_exec::execute_operator;
pub use registers::RegisterFile;
pub use surround::{add_surround, change_surround, delete_surround, surround_pair};
pub use text_manip::{case_toggle, indent_line, join_lines, sort_lines};
pub use text_object_exec::resolve_text_object;
