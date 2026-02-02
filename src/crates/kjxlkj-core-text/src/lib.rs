//! kjxlkj-core-text - Text rope and buffer storage.
//!
//! This crate provides efficient text storage using ropes.

mod rope;
mod buffer;
pub mod grapheme;

pub use rope::TextRope;
pub use buffer::TextBuffer;
pub use grapheme::{grapheme_count, display_width, char_width};
