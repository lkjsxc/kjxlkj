//! Shared types for kjxlkj editor.
//!
//! This crate defines fundamental types used across all kjxlkj crates.

mod buffer;
mod buffer_meta;
mod cursor;
mod encoding;
pub mod error;
mod filetype;
mod mode;
mod options;
mod position;
mod range;
mod variables;
mod version;
mod window;

#[cfg(test)]
mod tests;

pub use buffer::{BufferId, BufferName};
pub use buffer_meta::{BufferFlags, BufferMetadata, BufferType};
pub use cursor::Cursor;
pub use encoding::{detect_encoding, Encoding, FileMetadata, LineEnding};
pub use error::{Error, Message, Result, Severity};
pub use filetype::{detect, FileType};
pub use mode::Mode;
pub use options::{BufferOptions, OptionScope, OptionValue};
pub use position::Position;
pub use range::Range;
pub use variables::{BufferVars, VarValue, WindowVars};
pub use version::BufferVersion;
pub use window::WindowId;
