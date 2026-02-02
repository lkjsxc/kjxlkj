//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod edit;
mod highlight;
mod motion;
mod motion_handler;
mod operator;
mod search;
mod text_object;
mod transaction;

#[cfg(test)]
mod tests;

pub use edit::{Edit, EditKind};
pub use highlight::{BufferHighlights, HighlightGroup, HighlightSpan, LineHighlights};
pub use motion::{Motion, MotionKind};
pub use motion_handler::{execute_motion, MotionResult};
pub use operator::{Operator, OperatorKind};
pub use search::{SearchDirection, SearchMatch, SearchState};
pub use text_object::{TextObject, TextObjectKind, TextObjectModifier};
pub use transaction::Transaction;
