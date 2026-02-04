//! Core types shared across the editor.
//!
//! This crate defines fundamental types used by core, UI, render, and services.

mod cursor;
mod mode;
mod position;
mod range;
mod register;

pub use cursor::Cursor;
pub use mode::Mode;
pub use position::Position;
pub use range::Range;
pub use register::{Register, RegisterName};
