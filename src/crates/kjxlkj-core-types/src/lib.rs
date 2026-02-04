//! Core types shared across all kjxlkj crates.
//!
//! This crate defines fundamental types used throughout the editor:
//! - Buffer identity and versioning
//! - Cursor and position types
//! - Mode enumeration
//! - Common result/error types

mod buffer;
mod cursor;
mod error;
mod mode;
mod position;

pub use buffer::{BufferId, BufferName, BufferVersion};
pub use cursor::{Cursor, CursorShape};
pub use error::{EditorError, EditorResult};
pub use mode::Mode;
pub use position::{ByteOffset, CharOffset, LineCol, Position};

#[cfg(test)]
mod tests;
