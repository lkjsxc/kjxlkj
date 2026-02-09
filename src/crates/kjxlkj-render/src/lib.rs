/// Snapshot-to-terminal rendering pipeline.
///
/// Implements the five-stage render pipeline:
/// 1. Receive snapshot (watch channel)
/// 2. Build cell grid
/// 3. Apply decorations
/// 4. Diff against previous
/// 5. Flush to stdout
mod cell;
mod grid;
mod grid_window;
mod render_task;

pub use cell::{Cell, CellGrid};
pub use render_task::RenderTask;
