//! Shared types for kjxlkj editor.
//!
//! This crate defines fundamental types used across all kjxlkj crates.

mod buffer;
mod cursor;
mod mode;
mod position;
mod range;
mod version;
mod window;

#[cfg(test)]
mod tests;

pub use buffer::{BufferId, BufferName};
pub use cursor::Cursor;
pub use mode::Mode;
pub use position::Position;
pub use range::Range;
pub use version::BufferVersion;
pub use window::WindowId;
