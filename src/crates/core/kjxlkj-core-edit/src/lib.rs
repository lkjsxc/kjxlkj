//! Edit operations: cursor movement, text manipulation.
//!
//! Applies typed actions to buffer content.

mod cursor;
mod motion;

pub use cursor::Cursor;
pub use motion::apply_motion;
