//! Editing primitives and operators.

mod motion;
mod operator;
mod text_object;

#[cfg(test)]
mod motion_tests;

pub use motion::{Motion, MotionKind};
pub use operator::{Operator, OperatorKind};
pub use text_object::{TextObject, TextObjectKind};
