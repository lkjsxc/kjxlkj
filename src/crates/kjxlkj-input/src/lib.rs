//! Terminal input decoding.
//!
//! Converts crossterm events to editor key inputs.

mod decoder;

pub use decoder::InputDecoder;
