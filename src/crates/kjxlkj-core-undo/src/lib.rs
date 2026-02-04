//! Undo/redo model.
//!
//! This crate provides:
//! - Undo tree structure
//! - Transaction grouping
//! - Cursor position restoration

mod history;
mod transaction;

pub use history::UndoHistory;
pub use transaction::Transaction;

#[cfg(test)]
mod tests;
