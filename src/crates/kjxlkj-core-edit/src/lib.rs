//! Editing primitives and operators.
//!
//! This crate provides editing operations that work on text buffers.

mod motion;
mod operator;
mod text_object;

pub use motion::{apply_motion, Motion};
pub use operator::{apply_operator, Operator};
pub use text_object::{find_text_object, TextObject, TextObjectKind};
