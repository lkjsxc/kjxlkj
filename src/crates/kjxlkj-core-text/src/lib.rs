//! Text model implementation using rope data structure.
//!
//! This crate provides the text model for the editor, including:
//! - Rope-based text storage for efficient edits
//! - Grapheme cluster aware operations
//! - Line and character navigation

pub mod grapheme;
mod rope_ext;
mod text_buffer;
pub mod word;

pub use grapheme::{grapheme_count, nth_grapheme_offset};
pub use rope_ext::RopeSliceExt;
pub use text_buffer::TextBuffer;
pub use word::{find_word_boundary, WordKind};
