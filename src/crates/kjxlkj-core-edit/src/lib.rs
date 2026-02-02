//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod edit;
mod motion;
mod operator;
mod search;
mod text_object;
mod transaction;

#[cfg(test)]
mod tests;

pub use edit::{Edit, EditKind};
pub use motion::{Motion, MotionKind};
pub use operator::{Operator, OperatorKind};
pub use search::{SearchDirection, SearchMatch, SearchState};
pub use text_object::{TextObject, TextObjectKind, TextObjectModifier};
pub use transaction::Transaction;
