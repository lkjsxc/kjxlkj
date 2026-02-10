//! Editing primitives: motions, operators, text objects, cursor ops.

mod cursor;
mod edit_tests;
mod insert_ops;
mod motion;
mod operator;
pub mod register;
#[cfg(test)]
mod register_tests;
mod word_motion;

pub use cursor::{clamp_cursor, CursorPosition};
pub use insert_ops::{
    delete_char_backward, delete_char_forward, insert_char_at, insert_newline_above,
    insert_newline_below, join_lines, replace_char_at,
};
pub use motion::apply_motion;
pub use operator::apply_operator;
pub use register::{RegisterEntry, RegisterSet, RegisterType};
