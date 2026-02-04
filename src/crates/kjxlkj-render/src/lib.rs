//! Rendering pipeline.
//!
//! This crate provides:
//! - Snapshot to terminal frame conversion
//! - Efficient differential rendering
//! - Cursor positioning

mod renderer;
mod style;

pub use renderer::Renderer;
pub use style::Style;

#[cfg(test)]
mod tests;
