//! Rendering to terminal.
//!
//! This crate provides the render task and cell grid output.

mod painter;
mod task;

pub use painter::*;
pub use task::*;
