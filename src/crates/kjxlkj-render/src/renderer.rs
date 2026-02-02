//! Renderer that converts snapshots to terminal output.

use kjxlkj_core_types::snapshot::{EditorSnapshot, WindowSnapshot};
use crate::terminal::Terminal;
use std::io;

/// Renders editor snapshots to the terminal.
pub struct Renderer {
    /// Previous snapshot for diffing.
    prev_snapshot: Option<EditorSnapshot>,
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new() -> Self {
        Self {
            prev_snapshot: None,
        }
    }

    /// Renders a snapshot to the terminal.
    pub fn render(&mut self, terminal: &mut Terminal, snapshot: &EditorSnapshot) -> io::Result<()> {
        let (_width, height) = terminal.size();

        // Clear and render each window
        for window in &snapshot.windows {
            self.render_window(terminal, window)?;
        }

        // Render status line
        self.render_status_line(terminal, snapshot, height - 2)?;

        // Render command line or message
        self.render_command_line(terminal, snapshot, height - 1)?;

        // Position cursor
        if let Some(window) = snapshot.windows.iter().find(|w| w.id == snapshot.active_window) {
            let cursor_y = window.cursor.position().line.as_usize()
                .saturating_sub(window.top_line) as u16;
            let cursor_x = window.cursor.position().col.as_usize() as u16;
            terminal.move_cursor(
                window.dimensions.x + cursor_x,
                window.dimensions.y + cursor_y,
            )?;
            terminal.show_cursor()?;
        }

        terminal.flush()?;
        self.prev_snapshot = Some(snapshot.clone());
        Ok(())
    }

    fn render_window(&self, terminal: &mut Terminal, window: &WindowSnapshot) -> io::Result<()> {
        let dims = &window.dimensions;

        // Render visible lines
        for row in 0..dims.text_height() {
            let line_idx = window.top_line + row as usize;
            terminal.move_cursor(dims.x, dims.y + row)?;

            // For now, just show line numbers and placeholder
            // TODO: Get actual line content from buffer
            let line_num = format!("{:4} ", line_idx + 1);
            terminal.print(&line_num)?;

            // Fill rest with spaces
            let remaining = dims.width.saturating_sub(5);
            terminal.print(&" ".repeat(remaining as usize))?;
        }

        Ok(())
    }

    fn render_status_line(
        &self,
        terminal: &mut Terminal,
        snapshot: &EditorSnapshot,
        row: u16,
    ) -> io::Result<()> {
        terminal.move_cursor(0, row)?;

        let (width, _) = terminal.size();
        let status = &snapshot.status;

        let flags_str = if status.file_flags.is_empty() {
            String::new()
        } else {
            format!(" {}", status.file_flags)
        };

        let left = format!(" {} | {}{} ",
            status.mode_text,
            status.file_name,
            flags_str
        );

        let right = format!(" {}:{} {} ",
            status.position,
            status.file_type,
            status.percentage
        );

        let padding = width as usize - left.len() - right.len();
        let line = format!("{}{}{}", left, " ".repeat(padding.max(0)), right);

        let end = (width as usize).min(line.len());
        terminal.print(&line[..end])?;
        Ok(())
    }

    fn render_command_line(
        &self,
        terminal: &mut Terminal,
        snapshot: &EditorSnapshot,
        row: u16,
    ) -> io::Result<()> {
        terminal.move_cursor(0, row)?;

        let (width, _) = terminal.size();

        if let Some(cmd) = &snapshot.command_line {
            let line = format!("{}{}", cmd.prompt, cmd.content);
            terminal.print(&line)?;
            let remaining = width as usize - line.len();
            if remaining > 0 {
                terminal.print(&" ".repeat(remaining))?;
            }
        } else if let Some(msg) = &snapshot.message {
            terminal.print(&msg.text)?;
            let remaining = width as usize - msg.text.len();
            if remaining > 0 {
                terminal.print(&" ".repeat(remaining))?;
            }
        } else {
            terminal.print(&" ".repeat(width as usize))?;
        }

        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
