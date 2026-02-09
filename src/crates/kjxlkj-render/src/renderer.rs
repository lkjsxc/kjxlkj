//! Main renderer that drives the pipeline.

use kjxlkj_core_types::CellAttrs;
use kjxlkj_core_ui::EditorSnapshot;

use crate::cell_grid::CellGrid;
use crate::diff::FrameDiff;
use crate::flush;
use crate::renderer_window;

/// The renderer owns a previous frame and performs
/// diffing.
pub struct Renderer {
    /// Previous frame's cell grid.
    prev_grid: Option<CellGrid>,
    /// Current terminal dimensions.
    cols: u16,
    rows: u16,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            prev_grid: None,
            cols,
            rows,
        }
    }

    /// Render a snapshot to the terminal.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        let (cols, rows) = snapshot.terminal_size;
        self.cols = cols;
        self.rows = rows;

        let mut grid = CellGrid::new(cols, rows);

        for win_id in snapshot.layout.root.window_ids() {
            if let Some(rect) = snapshot.layout.root.find_window(win_id) {
                renderer_window::render_window(&mut grid, snapshot, win_id, rect);
            }
        }

        self.render_cmdline(&mut grid, snapshot);

        let diff = match &self.prev_grid {
            Some(prev) => FrameDiff::compute(prev, &grid),
            None => FrameDiff::full(&grid),
        };

        let buf = flush::build_flush_buffer(&diff);
        flush::flush_to_stdout(&buf)?;

        self.prev_grid = Some(grid);
        Ok(())
    }

    fn render_cmdline(&self, grid: &mut CellGrid, snapshot: &EditorSnapshot) {
        let y = self.rows - 1;
        if snapshot.cmdline.active {
            let display = snapshot.cmdline.display_string();
            grid.write_str(
                0,
                y,
                &display,
                snapshot.theme.cmdline.fg,
                snapshot.theme.cmdline.bg,
                CellAttrs::empty(),
            );
        } else if let Some(notif) = snapshot.notifications.last() {
            let color = match notif.level {
                kjxlkj_core_ui::NotificationLevel::Error => snapshot.theme.error_msg,
                kjxlkj_core_ui::NotificationLevel::Warning => snapshot.theme.warning_msg,
                kjxlkj_core_ui::NotificationLevel::Info => snapshot.theme.cmdline,
            };
            grid.write_str(0, y, &notif.message, color.fg, color.bg, CellAttrs::empty());
        }
    }

    /// Handle terminal resize.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        self.prev_grid = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renderer_creation() {
        let r = Renderer::new(80, 24);
        assert_eq!(r.cols, 80);
        assert_eq!(r.rows, 24);
    }
}
