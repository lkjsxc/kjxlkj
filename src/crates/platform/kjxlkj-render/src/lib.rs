//! Terminal renderer: snapshot → terminal output.
//!
//! See /docs/spec/architecture/render-pipeline.md for stages.

mod frame;
mod paint;

pub use frame::render_frame;
pub use paint::paint_to_stdout;

use kjxlkj_core_types::EditorSnapshot;
use std::io::{self, Write};

/// Render a snapshot to stdout. Entry point for the render task.
pub fn render(snapshot: &EditorSnapshot) -> io::Result<()> {
    let frame = frame::render_frame(snapshot);
    paint::paint_to_stdout(&frame, snapshot)?;
    Ok(())
}
