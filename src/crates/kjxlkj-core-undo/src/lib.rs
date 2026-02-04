//! Undo/redo model for the editor.

mod history;
mod transaction;

pub use history::UndoHistory;
pub use transaction::{Edit, EditKind, Transaction};
