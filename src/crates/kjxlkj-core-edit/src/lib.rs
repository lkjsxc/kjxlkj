//! Editing primitives for kjxlkj editor.
//!
//! This crate defines edit operations and operators.

mod change_event;
mod diff;
mod diff_types;
mod edit;
mod fold;
mod fold_types;
mod highlight;
mod highlight_types;
mod indent;
mod indent_types;
mod motion;
mod motion_handler;
mod motion_helpers;
mod motion_result;
mod operator;
mod operator_handler;
mod search;
mod search_highlight;
mod search_hl_types;
mod search_nav;
mod search_types;
mod spell;
mod spell_checker;
mod text_object;
mod text_object_finder;
mod text_object_helpers;
mod transaction;

#[cfg(test)]
mod tests;

pub use change_event::{ChangeDispatcher, ChangeKind, TextChange};
pub use diff::{DiffHunk, DiffKind, DiffState};
pub use edit::{Edit, EditKind};
pub use fold::Folds;
pub use fold_types::{Fold, FoldRange, FoldState};
pub use highlight::{BufferHighlights, LineHighlights};
pub use highlight_types::{HighlightGroup, HighlightSpan};
pub use indent::{
    adjust_indent_for_closing, compute_indent, detect_indent, detect_indent_style, IndentConfig,
    IndentStyle,
};
pub use motion::{Motion, MotionKind};
pub use motion_handler::execute_motion;
pub use motion_result::MotionResult;
pub use operator::{Operator, OperatorKind};
pub use operator_handler::{
    execute_operator, extract_text, indent_text, transform_case, IndentDirection, OperatorResult,
};
pub use search::SearchState;
pub use search_types::{SearchDirection, SearchMatch};
pub use search_highlight::SearchResult;
pub use search_hl_types::SearchHighlight;
pub use spell::{SimpleSpellChecker, SpellChecker, SpellState, SpellingError};
pub use text_object::{TextObject, TextObjectKind, TextObjectModifier};
pub use text_object_finder::find_text_object;
pub use transaction::Transaction;
