//! Terminal renderer implementation.

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Terminal renderer.
pub struct Renderer<W: Write> {
    writer: W,
    last_sequence: u64,
}

impl<W: Write> Renderer<W> {
    /// Create a new renderer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            last_sequence: 0,
        }
    }

    /// Render an editor snapshot.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        // Skip stale snapshots
        if snapshot.sequence <= self.last_sequence {
            return Ok(());
        }
        self.last_sequence = snapshot.sequence;

        execute!(self.writer, Hide)?;

        // Clear screen
        execute!(self.writer, Clear(ClearType::All))?;

        // Render windows
        for window in &snapshot.windows {
            self.render_window(window, snapshot)?;
        }

        // Render status line
        self.render_status_line(snapshot)?;

        // Render command line
        self.render_command_line(snapshot)?;

        // Position cursor
        if let Some(win) = snapshot.active_window() {
            let (row, col) = win.cursor_screen_pos();
            execute!(self.writer, MoveTo(col as u16, row as u16))?;

            // Set cursor style based on mode
            let style = match snapshot.mode {
                Mode::Normal => SetCursorStyle::SteadyBlock,
                Mode::Insert => SetCursorStyle::SteadyBar,
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    SetCursorStyle::SteadyUnderScore
                }
                Mode::Replace => SetCursorStyle::SteadyUnderScore,
                _ => SetCursorStyle::SteadyBlock,
            };
            execute!(self.writer, style)?;
        }

        execute!(self.writer, Show)?;
        self.writer.flush()?;

        Ok(())
    }

    fn render_window(
        &mut self,
        window: &kjxlkj_core_ui::WindowSnapshot,
        snapshot: &EditorSnapshot,
    ) -> io::Result<()> {
        let height = snapshot.terminal_height as usize;
        let width = snapshot.terminal_width as usize;

        // Reserve 2 lines for status and command line
        let content_height = height.saturating_sub(2);

        for (row, line) in window.buffer.lines.iter().enumerate() {
            if row >= content_height {
                break;
            }

            execute!(self.writer, MoveTo(0, row as u16))?;

            // Truncate line to fit width
            let display_line = truncate_to_width(line, width);
            execute!(self.writer, Print(&display_line))?;
        }

        // Fill remaining lines with ~
        for row in window.buffer.lines.len()..content_height {
            execute!(self.writer, MoveTo(0, row as u16))?;
            execute!(
                self.writer,
                SetForegroundColor(Color::DarkGrey),
                Print("~"),
                ResetColor
            )?;
        }

        Ok(())
    }

    fn render_status_line(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let height = snapshot.terminal_height;
        let width = snapshot.terminal_width as usize;
        let status_row = height.saturating_sub(2);

        execute!(self.writer, MoveTo(0, status_row))?;
        execute!(
            self.writer,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;

        // Build status line
        let mut status = String::new();

        // Mode indicator
        status.push_str(" ");
        status.push_str(snapshot.mode.indicator());
        status.push_str(" ");

        // Buffer info
        if let Some(win) = snapshot.active_window() {
            status.push_str("│ ");
            status.push_str(win.buffer.name.as_str());
            if win.buffer.modified {
                status.push_str(" [+]");
            }
        }

        // Right side: cursor position
        let right_info = if let Some(win) = snapshot.active_window() {
            format!(
                "{}:{} ",
                win.cursor.line + 1,
                win.cursor.column + 1
            )
        } else {
            String::new()
        };

        // Pad to fill width
        let left_width = UnicodeWidthStr::width(status.as_str());
        let right_width = UnicodeWidthStr::width(right_info.as_str());
        let padding = width.saturating_sub(left_width + right_width);

        for _ in 0..padding {
            status.push(' ');
        }
        status.push_str(&right_info);

        execute!(self.writer, Print(truncate_to_width(&status, width)))?;
        execute!(self.writer, ResetColor)?;

        Ok(())
    }

    fn render_command_line(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let height = snapshot.terminal_height;
        let width = snapshot.terminal_width as usize;
        let cmd_row = height.saturating_sub(1);

        execute!(self.writer, MoveTo(0, cmd_row))?;

        let content = match snapshot.mode {
            Mode::Command => format!(":{}", snapshot.command_line),
            Mode::Search => format!("/{}", snapshot.command_line),
            _ => {
                if let Some(msg) = &snapshot.message {
                    msg.clone()
                } else {
                    String::new()
                }
            }
        };

        execute!(self.writer, Print(truncate_to_width(&content, width)))?;

        Ok(())
    }
}

fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut width = 0;

    for c in s.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1);
        if width + char_width > max_width {
            break;
        }
        result.push(c);
        width += char_width;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_width() {
        let s = "hello world";
        let truncated = truncate_to_width(s, 5);
        assert_eq!(truncated, "hello");
    }

    #[test]
    fn test_truncate_unicode() {
        let s = "你好世界";
        let truncated = truncate_to_width(s, 4);
        assert_eq!(truncated, "你好");
    }

    #[test]
    fn test_renderer_creation() {
        let buf = Vec::new();
        let renderer = Renderer::new(buf);
        assert_eq!(renderer.last_sequence, 0);
    }
}
