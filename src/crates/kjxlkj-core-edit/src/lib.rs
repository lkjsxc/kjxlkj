//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod edit;
mod highlight;
mod indent;
mod motion;
mod motion_handler;
mod operator;
mod operator_handler;
mod search;
mod text_object;
mod transaction;

#[cfg(test)]
mod tests;

pub use edit::{Edit, EditKind};
pub use highlight::{BufferHighlights, HighlightGroup, HighlightSpan, LineHighlights};
pub use indent::{
    adjust_indent_for_closing, compute_indent, detect_indent, detect_indent_style, IndentConfig,
    IndentStyle,
};
pub use motion::{Motion, MotionKind};
pub use motion_handler::{execute_motion, MotionResult};
pub use operator::{Operator, OperatorKind};
pub use operator_handler::{
    execute_operator, extract_text, indent_text, transform_case, IndentDirection, OperatorResult,
};
pub use search::{SearchDirection, SearchMatch, SearchState};
pub use text_object::{TextObject, TextObjectKind, TextObjectModifier};
pub use transaction::Transaction;
