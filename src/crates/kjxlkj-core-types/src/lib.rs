//! Core types shared across the kjxlkj editor.
//!
//! This crate defines fundamental types used by core, UI, render, and services.

mod buffer;
mod cursor;
mod event;
mod mode;
mod position;
mod range;

pub use buffer::{BufferId, BufferName};
pub use cursor::Cursor;
pub use event::{EditorEvent, KeyEvent, Modifier};
pub use mode::Mode;
pub use position::Position;
pub use range::Range;

#[cfg(test)]
mod tests;
