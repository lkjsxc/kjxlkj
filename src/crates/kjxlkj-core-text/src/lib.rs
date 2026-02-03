//! Text model using rope data structure.
//!
//! Provides efficient text storage and manipulation for the editor.

mod rope_text;
mod text_utils;

pub use rope_text::RopeText;
pub use text_utils::{grapheme_width, line_grapheme_count};
