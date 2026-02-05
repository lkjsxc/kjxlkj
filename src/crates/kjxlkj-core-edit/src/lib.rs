//! Editing primitives and operators.

mod motion;
mod operator;
mod text_object;

pub use motion::{apply_motion, Motion};
pub use operator::Operator;
pub use text_object::TextObject;
