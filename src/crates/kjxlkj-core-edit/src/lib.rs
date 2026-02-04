//! Editing primitives and operators.
//!
//! This crate provides:
//! - Edit operations (insert, delete, replace)
//! - Operators (d, c, y, etc.)
//! - Text objects (word, sentence, paragraph)
//! - Motions as range generators

mod edit_op;
mod motion;
mod operator;
mod text_object;

pub use edit_op::{EditOp, EditResult};
pub use motion::{Motion, MotionKind};
pub use operator::{Operator, OperatorKind};
pub use text_object::{TextObject, TextObjectKind};

#[cfg(test)]
mod tests;
