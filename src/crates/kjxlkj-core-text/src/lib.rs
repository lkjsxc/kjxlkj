//! Core text handling for kjxlkj editor.
//!
//! This crate provides the text buffer implementation using a rope data
//! structure for efficient editing of large files.

mod grapheme;
mod text_buffer;

pub use grapheme::{grapheme_count, grapheme_width, GraphemeIter};
pub use text_buffer::TextBuffer;
