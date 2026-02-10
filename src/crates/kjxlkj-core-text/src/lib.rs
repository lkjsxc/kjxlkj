//! Rope/text/grapheme model for kjxlkj.
//!
//! Provides the text buffer abstraction built on `ropey::Rope`,
//! with grapheme-aware indexing and display-width computation.

mod buffer;
pub mod grapheme;
mod grapheme_tests;

pub use buffer::TextBuffer;
pub use grapheme::{
    display_col, grapheme_at_display_col, grapheme_count, line_graphemes, DisplayWidthCache,
};
pub use ropey::Rope;
