//! UI model types and snapshot structures.

mod snapshot;
mod viewport;

#[cfg(test)]
mod viewport_tests;
#[cfg(test)]
mod snapshot_tests;

pub use snapshot::{BufferSnapshot, EditorSnapshot, StatusLine};
pub use viewport::Viewport;
