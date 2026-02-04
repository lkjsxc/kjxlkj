//! Terminal renderer.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{Stdout, Write};
use unicode_width::UnicodeWidthStr;

/// Renders the editor to the terminal.
pub struct Renderer {
    last_mode: Option<Mode>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self { last_mode: None }
    }

    /// Render a snapshot to the terminal.
    pub fn render(&mut self, stdout: &mut Stdout, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        execute!(stdout, Hide)?;

        // Clear and render lines.
        let height = snapshot.viewport.height;
        let width = snapshot.viewport.width;

        for row in 0..height.saturating_sub(1) {
            execute!(stdout, MoveTo(0, row as u16))?;

            if row < snapshot.lines.len() {
                let line = &snapshot.lines[row];
                self.render_line(stdout, &line.content, width, snapshot)?;
            } else {
                // Empty line indicator.
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print("~"),
                    ResetColor
                )?;
            }

            // Clear to end of line.
            execute!(stdout, Clear(ClearType::UntilNewLine))?;
        }

        // Render status line.
        self.render_status(stdout, snapshot, height.saturating_sub(1) as u16, width)?;

        // Position cursor.
        let cursor_row = snapshot.cursor.line.saturating_sub(snapshot.viewport.top_line);
        let cursor_col = self.visual_column(snapshot, snapshot.cursor.col);

        execute!(
            stdout,
            MoveTo(cursor_col as u16, cursor_row as u16),
            Show
        )?;

        stdout.flush()?;
        self.last_mode = Some(snapshot.mode);
        Ok(())
    }

    fn render_line(
        &self,
        stdout: &mut Stdout,
        content: &str,
        _width: usize,
        snapshot: &EditorSnapshot,
    ) -> std::io::Result<()> {
        // Simple rendering without syntax highlighting.
        // Check for selection highlighting.
        if let Some(sel) = &snapshot.selection {
            let _ = sel; // TODO: highlight selection
        }

        execute!(stdout, Print(content))?;
        Ok(())
    }

    fn render_status(
        &self,
        stdout: &mut Stdout,
        snapshot: &EditorSnapshot,
        row: u16,
        width: usize,
    ) -> std::io::Result<()> {
        execute!(stdout, MoveTo(0, row))?;

        // Mode indicator.
        let mode_str = snapshot.status.mode.name();
        execute!(
            stdout,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print(format!(" {} ", mode_str)),
            ResetColor
        )?;

        // File info.
        let file_name = snapshot.status.file_name.as_deref().unwrap_or("[No Name]");
        let modified = if snapshot.status.modified { "[+]" } else { "" };
        execute!(stdout, Print(format!(" {}{} ", file_name, modified)))?;

        // Command line or status message.
        if let Some(ref cmd) = snapshot.command_line {
            execute!(stdout, Clear(ClearType::UntilNewLine))?;
            execute!(stdout, MoveTo(0, row + 1))?;
            execute!(stdout, Print(cmd))?;
        } else if let Some(ref msg) = snapshot.status.message {
            if snapshot.status.is_error {
                execute!(stdout, SetForegroundColor(Color::Red))?;
            }
            execute!(stdout, Print(msg))?;
            execute!(stdout, ResetColor)?;
        }

        // Right-aligned position info.
        let pos_str = format!(
            " {}:{}/{} ",
            snapshot.status.line,
            snapshot.status.col,
            snapshot.status.total_lines
        );
        let pos_col = width.saturating_sub(pos_str.len());
        execute!(
            stdout,
            MoveTo(pos_col as u16, row),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print(&pos_str),
            ResetColor
        )?;

        execute!(stdout, Clear(ClearType::UntilNewLine))?;
        Ok(())
    }

    fn visual_column(&self, snapshot: &EditorSnapshot, col: usize) -> usize {
        // Get the actual line content to calculate visual width.
        let line_idx = snapshot.cursor.line.saturating_sub(snapshot.viewport.top_line);
        if line_idx >= snapshot.lines.len() {
            return col;
        }

        let content = &snapshot.lines[line_idx].content;
        let prefix: String = content.chars().take(col).collect();
        UnicodeWidthStr::width(prefix.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_renderer() {
        let renderer = Renderer::new();
        assert!(renderer.last_mode.is_none());
    }
}
