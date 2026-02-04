//! Main renderer implementation.

use crossterm::{
    cursor::{self, MoveTo},
    queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{Clear, ClearType},
};
use kjxlkj_core_text::display_width;
use kjxlkj_core_types::{CursorShape, Mode};
use kjxlkj_core_ui::EditorSnapshot;
use std::io::Write;

use crate::style::mode_colors;

/// Renderer for terminal output.
pub struct Renderer {
    /// Cached terminal width.
    width: u16,
    /// Cached terminal height.
    height: u16,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Update dimensions.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Render a snapshot to the terminal.
    pub fn render<W: Write>(&self, w: &mut W, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        // Clear screen
        queue!(w, Clear(ClearType::All))?;

        // Render buffer lines
        self.render_buffer(w, snapshot)?;

        // Render status line
        self.render_status(w, snapshot)?;

        // Render command line
        self.render_command_line(w, snapshot)?;

        // Position cursor
        self.render_cursor(w, snapshot)?;

        w.flush()?;
        Ok(())
    }

    /// Render buffer content.
    fn render_buffer<W: Write>(&self, w: &mut W, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        let viewport_height = self.height.saturating_sub(2) as usize;

        for (row, line) in snapshot.buffer.lines.iter().enumerate() {
            if row >= viewport_height {
                break;
            }

            queue!(w, MoveTo(0, row as u16))?;

            // Truncate line to viewport width
            let display = self.truncate_to_width(line, self.width as usize);
            queue!(w, Print(&display))?;

            // Clear rest of line
            let remaining = self.width as usize - display_width(&display);
            if remaining > 0 {
                queue!(w, Print(" ".repeat(remaining)))?;
            }
        }

        // Fill empty lines with tildes
        for row in snapshot.buffer.lines.len()..viewport_height {
            queue!(
                w,
                MoveTo(0, row as u16),
                SetForegroundColor(Color::DarkGrey),
                Print("~"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    /// Render status line.
    fn render_status<W: Write>(&self, w: &mut W, snapshot: &EditorSnapshot) -> std::io::Result<()> {
        let status_row = self.height.saturating_sub(2);

        queue!(w, MoveTo(0, status_row))?;

        // Mode indicator with color
        let mode_color = match snapshot.mode {
            Mode::Normal => mode_colors::NORMAL,
            Mode::Insert => mode_colors::INSERT,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => mode_colors::VISUAL,
            Mode::Replace => mode_colors::REPLACE,
            Mode::Command => mode_colors::COMMAND,
        };

        queue!(
            w,
            SetBackgroundColor(mode_color),
            SetForegroundColor(Color::Black),
            SetAttribute(Attribute::Bold),
            Print(format!(" {} ", snapshot.mode.name())),
            ResetColor,
        )?;

        // Filename
        let filename = snapshot.buffer.name.as_str();
        let modified = if snapshot.buffer.modified { " [+]" } else { "" };
        queue!(w, Print(format!(" {}{}", filename, modified)))?;

        // Right side - position info
        let position_info = format!(
            " {}:{} ",
            snapshot.cursor.position.line + 1,
            snapshot.cursor.position.col + 1
        );
        let right_start = self.width as usize - position_info.len();

        // Fill middle with spaces
        let left_len = 3 + snapshot.mode.name().len() + 2 + filename.len() + modified.len();
        let fill_len = right_start.saturating_sub(left_len);
        queue!(w, Print(" ".repeat(fill_len)))?;

        queue!(w, Print(&position_info))?;

        Ok(())
    }

    /// Render command line.
    fn render_command_line<W: Write>(
        &self,
        w: &mut W,
        snapshot: &EditorSnapshot,
    ) -> std::io::Result<()> {
        let cmd_row = self.height.saturating_sub(1);
        queue!(w, MoveTo(0, cmd_row))?;

        if let Some(ref cmd) = snapshot.status.command_line {
            queue!(w, Print(cmd))?;
        } else if let Some(ref msg) = snapshot.status.message {
            queue!(w, Print(msg))?;
        }

        // Clear rest of line
        queue!(w, Clear(ClearType::UntilNewLine))?;

        Ok(())
    }

    /// Position and style the cursor.
    fn render_cursor<W: Write>(
        &self,
        w: &mut W,
        snapshot: &EditorSnapshot,
    ) -> std::io::Result<()> {
        // Set cursor shape
        match snapshot.cursor_shape {
            CursorShape::Block => {
                queue!(w, cursor::SetCursorStyle::SteadyBlock)?;
            }
            CursorShape::Bar => {
                queue!(w, cursor::SetCursorStyle::SteadyBar)?;
            }
            CursorShape::Underline => {
                queue!(w, cursor::SetCursorStyle::SteadyUnderScore)?;
            }
            CursorShape::Hollow => {
                queue!(w, cursor::SetCursorStyle::SteadyBlock)?;
            }
        }

        // Position cursor
        if let Some((col, row)) = snapshot.cursor_viewport_position() {
            queue!(w, MoveTo(col, row), cursor::Show)?;
        }

        Ok(())
    }

    /// Truncate a string to fit display width.
    fn truncate_to_width(&self, s: &str, max_width: usize) -> String {
        let mut result = String::new();
        let mut width = 0;

        for ch in s.chars() {
            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if width + ch_width > max_width {
                break;
            }
            result.push(ch);
            width += ch_width;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_ascii() {
        let renderer = Renderer::new(80, 24);
        let truncated = renderer.truncate_to_width("hello world", 5);
        assert_eq!(truncated, "hello");
    }

    #[test]
    fn truncate_wide_chars() {
        let renderer = Renderer::new(80, 24);
        let truncated = renderer.truncate_to_width("你好世界", 4);
        assert_eq!(truncated, "你好");
    }
}
