//! kjxlkj-render - Terminal rendering.
//!
//! This crate provides terminal output and rendering.

#![allow(dead_code)]

mod buffer;
mod gutter;
mod renderer;
mod statusline;
mod style;
mod terminal;
mod themes;

pub use buffer::{Cell, ScreenBuffer};
pub use gutter::{gutter_width, render_gutter_line, GutterConfig};
pub use renderer::Renderer;
pub use statusline::{render_statusline, StatuslineConfig, StatuslineContext};
pub use style::{Color, Style};
pub use terminal::Terminal;
pub use themes::RenderTheme;
