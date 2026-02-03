//! Core types shared across all crates.
//!
//! This crate defines stable identifiers, positions, and events used
//! by the editor core, services, and render pipeline.

mod buffer;
mod cursor;
mod event;
mod mode;
mod position;
mod window;

pub use buffer::{BufferId, BufferVersion};
pub use cursor::Cursor;
pub use event::{EditorAction, EditorEvent, Motion, Operator, ServiceEvent, ServiceRequest, TextObject};
pub use mode::Mode;
pub use position::{ByteOffset, CharOffset, LineCol, Point};
pub use window::WindowId;
