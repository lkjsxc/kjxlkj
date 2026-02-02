//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod edit;
mod motion;
mod operator;
mod text_object;
mod transaction;

pub use edit::{Edit, EditKind};
pub use motion::{Motion, MotionKind};
pub use operator::{Operator, OperatorKind};
pub use text_object::{TextObject, TextObjectKind};
pub use transaction::Transaction;
