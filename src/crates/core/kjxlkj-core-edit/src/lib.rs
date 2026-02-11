//! Edit operations: cursor movement, text manipulation.
//!
//! Applies typed actions to buffer content.

mod cursor;
mod motion;
mod motion_big_word;
mod motion_find;
mod motion_word;
pub mod regex_compile;

pub use cursor::Cursor;
pub use motion::apply_motion;
pub use regex_compile::vim_to_rust_regex;
