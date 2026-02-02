//! kjxlkj-core-text - Text rope and buffer storage.
//!
//! This crate provides efficient text storage using ropes.

mod buffer;
pub mod grapheme;
mod rope;

pub use buffer::TextBuffer;
pub use grapheme::{char_width, display_width, grapheme_count};
pub use rope::TextRope;
