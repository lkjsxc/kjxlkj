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

/// Terminal renderer.
pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }

    /// Render a snapshot to the terminal.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        execute!(self.stdout, Hide, MoveTo(0, 0), Clear(ClearType::All))?;

        let height = snapshot.height as usize;
        let width = snapshot.width as usize;
        let text_height = height.saturating_sub(2);

        // Render buffer lines
        for (i, line) in snapshot.window.buffer.lines.iter().enumerate() {
            if i >= text_height {
                break;
            }
            execute!(self.stdout, MoveTo(0, i as u16))?;
            let display_line = if line.width() > width {
                &line[..width]
            } else {
                line.as_str()
            };
            execute!(self.stdout, Print(display_line))?;
        }

        // Fill empty lines with tildes
        for i in snapshot.window.buffer.lines.len()..text_height {
            execute!(self.stdout, MoveTo(0, i as u16), Print("~"))?;
        }

        // Render status line
        let status_y = height.saturating_sub(2) as u16;
        execute!(
            self.stdout,
            MoveTo(0, status_y),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;

        let mode_str = snapshot.mode.indicator();
        let file_name = snapshot.window.buffer.name.as_str();
        let modified = if snapshot.window.buffer.modified {
            "[+]"
        } else {
            ""
        };
        let cursor = &snapshot.window.cursor;
        let line_info = format!(
            "{}:{} ({}/{})",
            cursor.line() + 1,
            cursor.column() + 1,
            cursor.line() + 1,
            snapshot.window.buffer.total_lines
        );

        let left_status = format!(" {} | {}{} ", mode_str, file_name, modified);
        let padding = width.saturating_sub(left_status.width() + line_info.width());
        let status_line = format!("{}{:padding$}{}", left_status, "", line_info);
        let status_display = if status_line.width() > width {
            status_line[..width].to_string()
        } else {
            format!("{:width$}", status_line)
        };
        execute!(self.stdout, Print(&status_display), ResetColor)?;

        // Render command line or status message
        let cmd_y = height.saturating_sub(1) as u16;
        execute!(self.stdout, MoveTo(0, cmd_y))?;
        if snapshot.mode == Mode::Command {
            execute!(self.stdout, Print(format!(":{}", snapshot.command_line)))?;
        } else if !snapshot.status.is_empty() {
            execute!(self.stdout, Print(&snapshot.status))?;
        }

        // Position cursor
        let cursor_y = cursor.line().saturating_sub(snapshot.window.top_line) as u16;
        let cursor_x = cursor.column() as u16;
        execute!(self.stdout, MoveTo(cursor_x, cursor_y), Show)?;

        self.stdout.flush()
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
