//! Core types for kjxlkj editor.
//!
//! This crate provides foundational types used across the editor.

mod buffer;
mod cursor;
mod event;
mod mode;
mod position;
mod range;
mod version;
mod window;

pub use buffer::{BufferId, BufferName};
pub use cursor::Cursor;
pub use event::{EditorEvent, Intent, KeyEvent, Modifier};
pub use mode::Mode;
pub use position::Position;
pub use range::Range;
pub use version::{BufferVersion, VersionCounter};
pub use window::WindowId;
