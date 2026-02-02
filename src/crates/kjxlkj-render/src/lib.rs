//! Rendering pipeline for kjxlkj editor.
//!
//! This crate renders snapshots to terminal frames.

mod buffer;
mod frame;
mod renderer;
mod style;

#[cfg(test)]
mod tests;

pub use buffer::{Cell, ScreenBuffer};
pub use frame::Frame;
pub use renderer::Renderer;
pub use style::{Color, Style};
