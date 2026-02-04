//! Text model using rope data structure.
//!
//! This crate provides:
//! - Rope-based text storage for efficient editing
//! - Line/column to byte offset conversion
//! - Unicode-aware text manipulation

mod rope_text;
mod text_utils;

pub use rope_text::RopeText;
pub use text_utils::{display_width, grapheme_count, line_graphemes};

#[cfg(test)]
mod tests;
