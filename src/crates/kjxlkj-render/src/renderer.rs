//! Renderer.

use crate::{Color, Frame, ScreenBuffer, Style};
use kjxlkj_core_ui::{Dimensions, EditorSnapshot};

/// Renderer that converts snapshots to frames.
pub struct Renderer {
    /// Current buffer.
    buffer: ScreenBuffer,
    /// Previous buffer for diffing.
    _prev_buffer: Option<ScreenBuffer>,
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new(dimensions: Dimensions) -> Self {
        Self {
            buffer: ScreenBuffer::new(dimensions),
            _prev_buffer: None,
        }
    }

    /// Renders a snapshot to a frame.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> Frame {
        self.buffer.resize(snapshot.dimensions);
        self.buffer.clear();

        // Calculate explorer offset
        let explorer_width = if snapshot.explorer.open {
            snapshot.explorer.width
        } else {
            0
        };

        self.render_file_explorer(snapshot);
        self.render_buffer_views(snapshot, explorer_width);
        self.render_status_line(snapshot);
        self.render_command_line(snapshot);

        Frame::new(self.buffer.clone())
    }

    /// Renders the file explorer.
    fn render_file_explorer(&mut self, snapshot: &EditorSnapshot) {
        if !snapshot.explorer.open {
            return;
        }

        let width = snapshot.explorer.width;
        let height = snapshot.dimensions.height.saturating_sub(2);
        let dir_style = Style::new().fg(Color::Blue).bold();
        let file_style = Style::default();
        let selected_style = Style::new().reverse();

        // Draw border
        for y in 0..height {
            self.buffer.set_char(width.saturating_sub(1), y, '│', Style::default());
        }

        // Draw entries
        for (i, entry) in snapshot.explorer.entries.iter().enumerate() {
            if i as u16 >= height {
                break;
            }
            let y = i as u16;
            let indent = "  ".repeat(entry.depth);
            let icon = if entry.is_dir {
                if entry.expanded { "▼ " } else { "▶ " }
            } else {
                "  "
            };
            let name = format!("{}{}{}", indent, icon, entry.name);
            let truncated: String = name.chars().take(width as usize - 2).collect();

            let style = if i == snapshot.explorer.selected {
                selected_style
            } else if entry.is_dir {
                dir_style
            } else {
                file_style
            };

            self.buffer.set_string(0, y, &truncated, style);
        }
    }

    /// Renders buffer views.
    fn render_buffer_views(&mut self, snapshot: &EditorSnapshot, x_offset: u16) {
        for view in &snapshot.views {
            let gutter_width = if view.line_numbers { 4 } else { 0 };

            for (i, line) in view.lines.iter().enumerate() {
                let y = i as u16;
                if y >= snapshot.dimensions.height.saturating_sub(2) {
                    break;
                }

                // Line number
                if view.line_numbers {
                    let line_num = view.viewport.first_line() + i + 1;
                    let num_str = format!("{:>3} ", line_num);
                    let num_style = Style::new().fg(Color::BrightBlack);
                    self.buffer.set_string(x_offset, y, &num_str, num_style);
                }

                // Line content
                let content_style = Style::default();
                self.buffer.set_string(x_offset + gutter_width, y, line, content_style);
            }

            // Render cursor - always show cursor even on empty cells
            let cursor_y = view.cursor_row() as u16;
            let cursor_x = x_offset + gutter_width + view.cursor_col() as u16;
            if cursor_y < snapshot.dimensions.height.saturating_sub(2) {
                let cursor_style = Style::new().reverse();
                let content = self.buffer
                    .get(cursor_x, cursor_y)
                    .map(|c| if c.content.is_empty() { " ".to_string() } else { c.content.clone() })
                    .unwrap_or_else(|| " ".to_string());
                self.buffer.set(cursor_x, cursor_y, crate::buffer::Cell::new(content, cursor_style));
            }
        }
    }

    /// Renders the status line.
    fn render_status_line(&mut self, snapshot: &EditorSnapshot) {
        let y = snapshot.dimensions.height.saturating_sub(2);
        let style = Style::new().fg(Color::Black).bg(Color::White);

        // Clear status line
        for x in 0..snapshot.dimensions.width {
            self.buffer.set_char(x, y, ' ', style);
        }

        // Mode
        let mode_str = format!(" {} ", snapshot.status.mode_str());
        self.buffer.set_string(0, y, &mode_str, style.bold());

        // File info
        let file_str = snapshot.status.file_info_str();
        self.buffer.set_string(10, y, &file_str, style);

        // Position
        let pos_str = snapshot.status.position_str();
        let pos_x = snapshot.dimensions.width.saturating_sub(pos_str.len() as u16 + 2);
        self.buffer.set_string(pos_x, y, &pos_str, style);
    }

    /// Renders the command line.
    fn render_command_line(&mut self, snapshot: &EditorSnapshot) {
        let y = snapshot.dimensions.height.saturating_sub(1);
        let style = Style::default();

        // Clear command line
        for x in 0..snapshot.dimensions.width {
            self.buffer.set_char(x, y, ' ', style);
        }

        // Show command or message
        if let Some(ref cmd) = snapshot.command_line {
            self.buffer.set_string(0, y, cmd, style);
        } else if let Some(ref msg) = snapshot.message {
            self.buffer.set_string(0, y, msg, style);
        }
    }
}
