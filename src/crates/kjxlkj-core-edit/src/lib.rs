//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod change_event;
mod diff;
mod edit;
mod fold;
mod highlight;
mod indent;
mod motion;
mod motion_handler;
mod operator;
mod operator_handler;
mod search;
mod search_highlight;
mod spell;
mod text_object;
mod text_object_finder;
mod transaction;

#[cfg(test)]
mod tests;

pub use change_event::{ChangeDispatcher, ChangeKind, TextChange};
pub use diff::{DiffHunk, DiffKind, DiffState};
pub use edit::{Edit, EditKind};
pub use fold::{Fold, FoldRange, FoldState, Folds};
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
pub use search_highlight::{SearchHighlight, SearchResult};
pub use spell::{SimpleSpellChecker, SpellChecker, SpellState, SpellingError};
pub use text_object::{TextObject, TextObjectKind, TextObjectModifier};
pub use text_object_finder::find_text_object;
pub use transaction::Transaction;
