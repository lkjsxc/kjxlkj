//! Undo/redo system for the editor.
//!
//! Provides transactional undo with support for grouping multiple edits.

mod history;
mod transaction;

pub use history::UndoHistory;
pub use transaction::{Edit, Transaction};
