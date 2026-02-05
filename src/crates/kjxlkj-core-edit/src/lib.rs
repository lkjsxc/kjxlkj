//! Editing operations for kjxlkj editor.
//!
//! This crate provides editing primitives including motions,
//! operators, and text objects.

mod motion;
mod operator;
mod text_object;

pub use motion::{apply_motion, Motion, MotionResult};
pub use operator::{apply_operator, Operator};
pub use text_object::{find_text_object, TextObject, TextObjectKind};
