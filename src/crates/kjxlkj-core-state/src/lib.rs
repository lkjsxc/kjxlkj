//! Editor state aggregation for kjxlkj.
//!
//! This crate owns the complete editor state and produces snapshots.

mod apply;
mod editor;
mod registers;

pub use editor::*;
pub use registers::*;

#[cfg(test)]
mod tests;
