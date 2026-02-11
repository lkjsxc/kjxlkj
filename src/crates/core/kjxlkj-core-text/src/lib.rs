//! Rope-based text storage and buffer model.
//!
//! See /docs/spec/editor/buffers.md for normative requirements.

mod buffer;
mod buffer_edit;
mod grapheme;
mod line;

pub use buffer::Buffer;
pub use grapheme::{grapheme_count, grapheme_to_byte_offset};
pub use line::{line_content, line_count};
