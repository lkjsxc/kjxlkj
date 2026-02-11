//! Edit operations: cursor movement, text manipulation.
//!
//! Applies typed actions to buffer content.

mod cursor;
mod motion;
mod motion_big_word;
mod motion_find;
mod motion_word;

pub use cursor::Cursor;
pub use motion::apply_motion;
