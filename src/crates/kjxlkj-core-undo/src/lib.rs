//! Undo/redo model for the editor.

mod history;
mod transaction;

#[cfg(test)]
mod transaction_tests;

pub use history::UndoHistory;
pub use transaction::{Edit, EditKind, Transaction};
