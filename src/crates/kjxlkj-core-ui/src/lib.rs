//! UI model types and snapshot structures.
//!
//! This crate provides:
//! - Viewport representation
//! - Snapshot structures for rendering
//! - Status line model

mod snapshot;
mod status;
mod viewport;

pub use snapshot::{BufferSnapshot, EditorSnapshot};
pub use status::{StatusLine, StatusSection};
pub use viewport::Viewport;

#[cfg(test)]
mod tests;
