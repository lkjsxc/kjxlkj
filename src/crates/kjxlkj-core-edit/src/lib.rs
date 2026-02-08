//! Editing primitives: motions, operators, text objects, and register operations.
//!
//! This crate implements the core editing engine as specified in
//! /docs/spec/editing/README.md.

mod char_find;
mod cursor;
mod motion_exec;
mod motion_helpers;
mod operator_exec;
mod registers;
mod text_manip;
mod text_object_delimited;
mod text_object_exec;

pub use char_find::{CharFind, CharFindKind};
pub use cursor::CursorPosition;
pub use motion_exec::execute_motion;
pub use operator_exec::execute_operator;
pub use registers::RegisterFile;
pub use text_manip::{
    case_toggle, indent_line, join_lines, sort_lines,
};
pub use text_object_exec::resolve_text_object;
