//! Edit operations: cursor movement, text manipulation.
//!
//! Applies typed actions to buffer content.

mod cursor;
mod motion;
mod motion_big_word;
mod motion_find;
mod motion_word;
pub mod regex_compile;
pub mod text_object;

pub use cursor::Cursor;
pub use motion::apply_motion;
pub use regex_compile::vim_to_rust_regex;
pub use regex_compile::vim_to_rust_regex_ex;
pub use text_object::text_obj_range;
