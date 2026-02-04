//! UI model types and snapshot structures.
//!
//! This crate defines the types used for rendering the editor UI.

mod snapshot;
mod viewport;

pub use snapshot::{BufferSnapshot, EditorSnapshot, StatusLine};
pub use viewport::Viewport;
