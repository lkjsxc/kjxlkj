//! Editing primitives and operators.
//!
//! This crate provides the core editing operations that transform text.

mod buffer;
mod cursor_ops;
mod text_objects;
mod transaction;

pub use buffer::Buffer;
pub use cursor_ops::CursorOps;
pub use text_objects::{find_text_object_range, TextObjectRange};
pub use transaction::Transaction;
