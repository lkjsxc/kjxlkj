//! Text model using rope data structure.
//!
//! This crate provides the core text buffer and grapheme utilities.

mod buffer;
mod grapheme;

pub use buffer::TextBuffer;
pub use grapheme::{grapheme_width, is_word_char};
