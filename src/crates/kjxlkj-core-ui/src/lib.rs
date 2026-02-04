//! UI model types and snapshot structures.

mod snapshot;
mod status;
mod viewport;

pub use snapshot::{EditorSnapshot, SnapshotLine};
pub use status::StatusLine;
pub use viewport::Viewport;
