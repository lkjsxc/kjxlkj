//! Rendering to terminal.
//!
//! This crate provides the render task and cell grid output.

mod color;
mod grid;
mod painter;
mod task;

pub use painter::*;
pub use task::*;
