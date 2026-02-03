//! UI model types and snapshot structures.
//!
//! This crate defines the data structures consumed by the renderer.

mod snapshot;
mod viewport;

pub use snapshot::{BufferSnapshot, EditorSnapshot, SnapshotSeq, StatusLine};
pub use viewport::Viewport;
