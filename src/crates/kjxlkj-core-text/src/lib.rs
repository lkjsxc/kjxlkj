//! Text model for the kjxlkj editor.
//!
//! Uses a rope data structure for efficient text manipulation.

mod buffer;
mod grapheme;
mod line;

pub use buffer::TextBuffer;
pub use grapheme::{grapheme_count, grapheme_width, nth_grapheme_offset};
pub use line::LineInfo;
