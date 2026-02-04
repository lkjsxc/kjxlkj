//! Editor state aggregation and snapshot production.
//!
//! This crate provides:
//! - Buffer state management
//! - Complete editor state
//! - Snapshot generation for rendering

mod buffer_state;
mod editor_state;

pub use buffer_state::BufferState;
pub use editor_state::EditorState;

#[cfg(test)]
mod tests;
