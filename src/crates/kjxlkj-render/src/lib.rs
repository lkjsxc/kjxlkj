//! kjxlkj-render - Terminal rendering.
//!
//! This crate provides terminal output and rendering.

#![allow(dead_code)]

mod terminal;
mod renderer;
mod style;
mod buffer;
mod statusline;
mod gutter;
mod themes;

pub use terminal::Terminal;
pub use renderer::Renderer;
pub use style::{Color, Style};
pub use buffer::{Cell, ScreenBuffer};
pub use statusline::{render_statusline, StatuslineConfig, StatuslineContext};
pub use gutter::{gutter_width, render_gutter_line, GutterConfig};
pub use themes::RenderTheme;
