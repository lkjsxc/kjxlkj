//! Editing primitives and operators.
//!
//! This crate provides the core editing operations that transform text.

mod buffer;
mod cursor_ops;
mod transaction;

pub use buffer::Buffer;
pub use cursor_ops::CursorOps;
pub use transaction::Transaction;
