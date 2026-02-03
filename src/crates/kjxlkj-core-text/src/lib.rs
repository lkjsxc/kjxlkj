//! Text model for kjxlkj.
//!
//! This crate provides the rope-based text storage and manipulation
//! primitives used by the editor core.

mod buffer;
mod grapheme;
mod rope_ext;

pub use buffer::*;
pub use grapheme::*;
pub use rope_ext::*;

#[cfg(test)]
mod tests;
