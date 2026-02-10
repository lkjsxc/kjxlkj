//! Text storage and manipulation using rope data structure.
//!
//! This crate provides efficient text storage with O(log n) operations.

mod grapheme;
mod rope_ext;

pub use grapheme::*;
pub use rope_ext::*;

// Re-export ropey types.
pub use ropey::{Rope, RopeSlice};
