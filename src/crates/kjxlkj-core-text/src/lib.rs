//! Text model for kjxlkj editor.
//!
//! This crate provides the rope-based text buffer implementation.

mod grapheme;
mod rope_ext;
mod text_buffer;

#[cfg(test)]
mod tests;

pub use grapheme::{GraphemeIter, grapheme_count, grapheme_width};
pub use rope_ext::RopeExt;
pub use text_buffer::TextBuffer;
