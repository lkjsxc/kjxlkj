//! Undo/redo model with tree-based branching.
//!
//! Implements the undo tree as specified in
//! /docs/spec/editing/text-manipulation/undo.md
//! and /docs/spec/features/session/undo_tree.md.

mod persistence;
mod tree;
#[cfg(test)]
mod tree_tests;

pub use persistence::{load_undo_tree, save_undo_tree};
pub use tree::{UndoEntry, UndoGroup, UndoTree};
