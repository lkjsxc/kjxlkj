//! Main renderer that drives the pipeline.

use kjxlkj_core_types::{CellAttrs, Color, WindowId};
use kjxlkj_core_ui::{
    BufferSnapshot, EditorSnapshot, Rect,
};

use crate::cell_grid::CellGrid;
use crate::diff::FrameDiff;
use crate::flush;
use crate::gutter;
use crate::statusline;

/// The renderer owns a previous frame and performs diffing.
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
    pub fn render(
        &mut self,
        snapshot: &EditorSnapshot,
    ) -> std::io::Result<()> {
        let (cols, rows) = snapshot.terminal_size;
        self.cols = cols;
        self.rows = rows;

        let mut grid = CellGrid::new(cols, rows);

        // Render each window.
        for win_id in snapshot.layout.root.window_ids() {
            if let Some(rect) =
                snapshot.layout.root.find_window(win_id)
            {
                self.render_window(
                    &mut grid,
                    &snapshot,
                    win_id,
                    rect,
                );
            }
        }

        // Render command line / message area.
        self.render_cmdline(&mut grid, snapshot);

        // Diff and flush.
        let diff = match &self.prev_grid {
            Some(prev) => FrameDiff::compute(prev, &grid),
            None => FrameDiff::full(&grid),
        };

        let buf = flush::build_flush_buffer(&diff);
        flush::flush_to_stdout(&buf)?;

        self.prev_grid = Some(grid);
        Ok(())
    }

    fn render_window(
        &self,
        grid: &mut CellGrid,
        snapshot: &EditorSnapshot,
        win_id: WindowId,
        rect: Rect,
    ) {
        let is_focused =
            snapshot.layout.focused == win_id;

        // Find the buffer for this window.
        // For now, use the first buffer (simplified).
        if let Some((_id, buf)) =
            snapshot.buffers.iter().next()
        {
            let gutter_width = 4_u16;
            let text_x = rect.x + gutter_width;
            let text_width =
                rect.width.saturating_sub(gutter_width + 1);
            let text_height = rect.height.saturating_sub(1);

            // Render gutter.
            gutter::render_gutter(
                grid,
                rect.x,
                rect.y,
                gutter_width,
                buf.top_line,
                buf.line_count,
                buf.cursor_line,
                text_height,
                snapshot.theme.line_nr,
                snapshot.theme.cursor_line_nr,
            );

            // Render buffer text.
            self.render_buffer_text(
                grid,
                buf,
                text_x,
                rect.y,
                text_width,
                text_height,
            );

            // Render cursor.
            if is_focused {
                let cursor_row = buf
                    .cursor_line
                    .saturating_sub(buf.top_line)
                    as u16;
                let cursor_col =
                    text_x + buf.cursor_col as u16;
                if cursor_row < text_height
                    && cursor_col < rect.x + rect.width
                {
                    let cell = grid.get_mut(
                        cursor_col,
                        rect.y + cursor_row,
                    );
                    cell.attrs |= CellAttrs::REVERSE;
                }
            }

            // Render statusline.
            let status_y = rect.y + rect.height - 1;
            let status_color = if is_focused {
                snapshot.theme.statusline
            } else {
                snapshot.theme.statusline_nc
            };
            statusline::render_statusline(
                grid,
                status_y,
                rect.width,
                &snapshot.mode,
                buf,
                is_focused,
                status_color,
                snapshot.theme.mode_indicator,
            );
        }
    }

    fn render_buffer_text(
        &self,
        grid: &mut CellGrid,
        buf: &BufferSnapshot,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        for row in 0..height {
            let line_idx = row as usize;
            if line_idx < buf.visible_lines.len() {
                let line = &buf.visible_lines[line_idx];
                let truncated = if line.len() > width as usize
                {
                    &line[..width as usize]
                } else {
                    line.as_str()
                };
                grid.write_str(
                    x,
                    y + row,
                    truncated,
                    Color::Default,
                    Color::Default,
                    CellAttrs::empty(),
                );
            }
        }
    }

    fn render_cmdline(
        &self,
        grid: &mut CellGrid,
        snapshot: &EditorSnapshot,
    ) {
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
        } else if let Some(notif) =
            snapshot.notifications.last()
        {
            let color = match notif.level {
                kjxlkj_core_ui::NotificationLevel::Error => {
                    snapshot.theme.error_msg
                }
                kjxlkj_core_ui::NotificationLevel::Warning => {
                    snapshot.theme.warning_msg
                }
                kjxlkj_core_ui::NotificationLevel::Info => {
                    snapshot.theme.cmdline
                }
            };
            grid.write_str(
                0,
                y,
                &notif.message,
                color.fg,
                color.bg,
                CellAttrs::empty(),
            );
        }
    }

    /// Handle terminal resize.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        self.prev_grid = None; // Force full redraw.
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
