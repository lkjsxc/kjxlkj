//! Terminal input decoding.
//!
//! This crate converts terminal events to editor actions.

mod convert;
mod key;

pub use convert::convert_event;
pub use key::{Key, KeyCode, Modifiers};
