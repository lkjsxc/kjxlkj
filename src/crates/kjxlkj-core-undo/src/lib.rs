//! Undo/redo model with tree-based branching.
//!
//! Implements the undo tree as specified in
//! /docs/spec/editing/text-manipulation/undo.md
//! and /docs/spec/features/session/undo_tree.md.

mod persistence;
mod tree;
mod tree_helpers;
#[cfg(test)]
mod tree_tests;
mod tree_types;

pub use persistence::{load_undo_tree, save_undo_tree};
pub use tree::UndoTree;
pub use tree_types::{UndoEntry, UndoGroup};
