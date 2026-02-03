//! Core types shared across kjxlkj crates.
//!
//! This crate provides foundational types used by the editor core, UI,
//! rendering, and services. Types here are designed for:
//! - Serialization (for snapshots and IPC)
//! - Clone efficiency (Arc-wrapped where appropriate)
//! - Deterministic behavior

mod buffer;
mod cursor;
mod error;
mod ids;
mod intent;
mod key;
mod mode;
mod position;
mod range;
mod register;
mod window;

pub use buffer::*;
pub use cursor::*;
pub use error::*;
pub use ids::*;
pub use intent::*;
pub use key::*;
pub use mode::*;
pub use position::*;
pub use range::*;
pub use register::*;
pub use window::*;

#[cfg(test)]
mod tests;
