//! UI model types for kjxlkj.
//!
//! This crate provides snapshot structures consumed by the renderer.

mod snapshot;
mod status;

pub use snapshot::*;
pub use status::*;

#[cfg(test)]
mod tests;
