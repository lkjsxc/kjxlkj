//! Rendering pipeline: snapshot → terminal frame.

mod cell_grid;
mod diff;
mod flush;
mod gutter;
mod renderer;
mod renderer_window;
mod statusline;

pub use cell_grid::CellGrid;
pub use diff::FrameDiff;
pub use renderer::Renderer;
