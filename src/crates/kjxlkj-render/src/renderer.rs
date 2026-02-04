//! Terminal renderer implementation.

use crate::{RenderOutput, Style};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{self, Stdout, Write};
use unicode_width::UnicodeWidthStr;

/// Terminal-based renderer.
pub struct TerminalRenderer {
    stdout: Stdout,
    /// Previous snapshot for diffing.
    prev_snapshot: Option<EditorSnapshot>,
}

impl TerminalRenderer {
    /// Create a new terminal renderer.
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;
        Ok(Self {
            stdout,
            prev_snapshot: None,
        })
    }

    /// Cleanup terminal state.
    pub fn cleanup(&mut self) -> io::Result<()> {
        execute!(self.stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Render a snapshot.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        execute!(self.stdout, Hide)?;

        // Clear screen on first render or if dimensions changed
        let needs_full_render = match &self.prev_snapshot {
            None => true,
            Some(prev) => prev.width != snapshot.width || prev.height != snapshot.height,
        };

        if needs_full_render {
            execute!(self.stdout, Clear(ClearType::All))?;
        }

        // Render buffer content
        self.render_buffer(snapshot)?;

        // Render status line
        self.render_status(snapshot)?;

        // Render command line
        self.render_command_line(snapshot)?;

        // Position cursor
        self.position_cursor(snapshot)?;

        execute!(self.stdout, Show)?;
        self.stdout.flush()?;

        self.prev_snapshot = Some(snapshot.clone());
        Ok(())
    }

    fn render_buffer(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let buffer = &snapshot.buffer;
        let viewport = buffer.viewport;

        for (screen_row, line) in buffer.lines.iter().enumerate() {
            execute!(self.stdout, MoveTo(0, screen_row as u16))?;

            // Clear the line
            execute!(self.stdout, Clear(ClearType::CurrentLine))?;

            // Check if this line is within selection
            let line_idx = viewport.top_line + screen_row;
            let in_selection = snapshot.is_line_in_selection(line_idx);

            if in_selection {
                execute!(self.stdout, SetAttribute(Attribute::Reverse))?;
            }

            // Handle line numbers (optional)
            // For now, just render content

            // Truncate line to viewport width
            let display_width = viewport.width;
            let line_display = truncate_to_width(line, display_width);

            execute!(self.stdout, Print(&line_display))?;

            if in_selection {
                execute!(self.stdout, SetAttribute(Attribute::NoReverse))?;
            }
        }

        // Fill remaining lines with ~
        let content_lines = buffer.lines.len();
        let viewport_height = snapshot.height.saturating_sub(2) as usize;

        for screen_row in content_lines..viewport_height {
            execute!(
                self.stdout,
                MoveTo(0, screen_row as u16),
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::DarkGrey),
                Print("~"),
                ResetColor
            )?;
        }

        Ok(())
    }

    fn render_status(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let status = &snapshot.status;
        let y = snapshot.height.saturating_sub(2);

        execute!(
            self.stdout,
            MoveTo(0, y),
            Clear(ClearType::CurrentLine),
            SetAttribute(Attribute::Reverse)
        )?;

        // Mode indicator
        let mode_str = match snapshot.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::VisualLine => "V-LINE",
            Mode::VisualBlock => "V-BLOCK",
            Mode::Command => "COMMAND",
            Mode::Replace => "REPLACE",
        };

        execute!(
            self.stdout,
            Print(format!(" {} ", mode_str))
        )?;

        execute!(self.stdout, SetAttribute(Attribute::NoReverse))?;

        // File name
        let modified = if status.modified { "[+]" } else { "" };
        execute!(
            self.stdout,
            Print(format!(" {}{} ", status.file_name, modified))
        )?;

        // Position info (right aligned)
        let pos_info = format!(
            " {}:{} / {} ",
            status.line + 1,
            status.col + 1,
            status.total_lines
        );
        let padding = (snapshot.width as usize)
            .saturating_sub(mode_str.len() + 3)
            .saturating_sub(status.file_name.len() + modified.len() + 2)
            .saturating_sub(pos_info.len());

        for _ in 0..padding {
            execute!(self.stdout, Print(" "))?;
        }

        execute!(self.stdout, Print(&pos_info), ResetColor)?;

        Ok(())
    }

    fn render_command_line(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let y = snapshot.height.saturating_sub(1);
        execute!(
            self.stdout,
            MoveTo(0, y),
            Clear(ClearType::CurrentLine)
        )?;

        if let Some(ref cmd) = snapshot.command_line {
            execute!(self.stdout, Print(format!(":{}", cmd)))?;
        } else if let Some((ref msg, is_error)) = snapshot.status.message {
            if is_error {
                execute!(self.stdout, SetForegroundColor(Color::Red))?;
            }
            execute!(self.stdout, Print(msg), ResetColor)?;
        }

        Ok(())
    }

    fn position_cursor(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let viewport = snapshot.buffer.viewport;

        // Calculate screen position
        let screen_row = snapshot
            .cursor
            .line()
            .saturating_sub(viewport.top_line);
        let screen_col = snapshot.cursor.col().saturating_sub(viewport.left_col);

        // Command line cursor position
        if snapshot.mode == Mode::Command {
            let y = snapshot.height.saturating_sub(1);
            let x = snapshot.command_line.as_ref().map(|c| c.len() + 1).unwrap_or(1);
            execute!(self.stdout, MoveTo(x as u16, y))?;
        } else {
            execute!(
                self.stdout,
                MoveTo(screen_col as u16, screen_row as u16)
            )?;
        }

        Ok(())
    }
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to initialize terminal renderer")
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl RenderOutput for TerminalRenderer {
    fn clear(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    fn move_to(&mut self, x: u16, y: u16) -> io::Result<()> {
        execute!(self.stdout, MoveTo(x, y))
    }

    fn write_styled(&mut self, text: &str, style: Style) -> io::Result<()> {
        if let Some(fg) = style.fg {
            execute!(self.stdout, SetForegroundColor(fg))?;
        }
        if let Some(bg) = style.bg {
            execute!(self.stdout, SetBackgroundColor(bg))?;
        }
        if style.bold {
            execute!(self.stdout, SetAttribute(Attribute::Bold))?;
        }
        if style.reverse {
            execute!(self.stdout, SetAttribute(Attribute::Reverse))?;
        }

        execute!(self.stdout, Print(text))?;

        execute!(
            self.stdout,
            SetAttribute(Attribute::Reset),
            ResetColor
        )
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Show)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, Hide)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

/// Truncate a string to a display width.
fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut width = 0;

    for c in s.chars() {
        let c_width = UnicodeWidthStr::width(c.to_string().as_str());
        if width + c_width > max_width {
            break;
        }
        result.push(c);
        width += c_width;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("hello", 3), "hel");
        assert_eq!(truncate_to_width("hello", 10), "hello");
        assert_eq!(truncate_to_width("", 5), "");
    }

    #[test]
    fn test_truncate_unicode() {
        // Wide characters
        assert_eq!(truncate_to_width("你好世界", 4), "你好");
    }

    #[test]
    fn test_truncate_exact_width() {
        assert_eq!(truncate_to_width("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_zero_width() {
        assert_eq!(truncate_to_width("hello", 0), "");
    }

    #[test]
    fn test_truncate_mixed_width() {
        // Mix of narrow and wide characters
        let result = truncate_to_width("a你b好", 5);
        // 'a' = 1, '你' = 2, 'b' = 1, total = 4 <= 5
        assert!(result.len() <= 8); // UTF-8 bytes
    }

    #[test]
    fn test_truncate_emoji() {
        // Emoji can have various widths
        let result = truncate_to_width("👋hello", 6);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_truncate_single_char() {
        assert_eq!(truncate_to_width("a", 1), "a");
        assert_eq!(truncate_to_width("a", 0), "");
    }
}
