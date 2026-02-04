//! Core types shared across the kjxlkj editor.
//!
//! This crate contains foundational types used by core, UI, render, and services.

mod buffer_id;
mod cursor;
mod key;
mod mode;
mod position;
mod register;
mod selection;

pub use buffer_id::BufferId;
pub use cursor::Cursor;
pub use key::{Key, KeyCode, Modifiers};
pub use mode::Mode;
pub use position::Position;
pub use register::{Register, RegisterName};
pub use selection::{Selection, SelectionKind};
