//! Editing primitives and operators for the editor.

mod motion;
mod operator;
mod text_object;

pub use motion::{Motion, MotionKind};
pub use operator::{Operator, OperatorKind};
pub use text_object::{TextObject, TextObjectKind};
