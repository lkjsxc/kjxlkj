//! Rendering pipeline for kjxlkj editor.
//!
//! This crate renders snapshots to terminal frames.

mod buffer;
mod frame;
mod renderer;
mod style;

pub use buffer::ScreenBuffer;
pub use frame::Frame;
pub use renderer::Renderer;
pub use style::{Color, Style};
