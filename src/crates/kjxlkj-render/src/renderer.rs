//! Terminal renderer.

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_types::{Mode, Position, Selection, SelectionKind};
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{self, Write};

/// Terminal renderer.
pub struct Renderer {
    _private: (),
}

impl Renderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Render a snapshot.
    pub fn render<W: Write>(&self, writer: &mut W, snapshot: &EditorSnapshot) -> io::Result<()> {
        execute!(writer, Hide, MoveTo(0, 0))?;

        // Render buffer lines
        for (i, line) in snapshot.buffer.lines.iter().enumerate() {
            execute!(writer, MoveTo(0, i as u16))?;
            self.render_line(writer, snapshot, i, line)?;
        }

        // Clear remaining lines
        for i in snapshot.buffer.lines.len()..snapshot.viewport.height as usize {
            execute!(
                writer,
                MoveTo(0, i as u16),
                Clear(ClearType::CurrentLine),
                SetForegroundColor(Color::DarkGrey),
                Print("~"),
                ResetColor
            )?;
        }

        // Render status line
        self.render_status(writer, snapshot)?;

        // Position cursor
        let (col, row) = snapshot.cursor_screen_position();
        execute!(writer, MoveTo(col, row))?;

        // Set cursor style based on mode
        let style = match snapshot.mode {
            Mode::Normal => SetCursorStyle::SteadyBlock,
            Mode::Insert => SetCursorStyle::SteadyBar,
            Mode::Replace => SetCursorStyle::SteadyUnderScore,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => SetCursorStyle::SteadyBlock,
            Mode::Command => SetCursorStyle::SteadyBar,
        };
        execute!(writer, style, Show)?;

        writer.flush()
    }

    fn render_line<W: Write>(&self, writer: &mut W, snapshot: &EditorSnapshot, screen_row: usize, line: &str) -> io::Result<()> {
        execute!(writer, Clear(ClearType::CurrentLine))?;

        let buffer_line = snapshot.viewport.first_line + screen_row;

        // Check if this line is in selection
        if let Some(ref sel) = snapshot.selection {
            self.render_line_with_selection(writer, line, buffer_line, sel)?;
        } else {
            execute!(writer, Print(line))?;
        }

        Ok(())
    }

    fn render_line_with_selection<W: Write>(
        &self,
        writer: &mut W,
        line: &str,
        line_idx: usize,
        selection: &Selection,
    ) -> io::Result<()> {
        let start = selection.start();
        let end = selection.end();

        match selection.kind {
            SelectionKind::Char => {
                self.render_char_selection(writer, line, line_idx, start, end)?;
            }
            SelectionKind::Line => {
                if line_idx >= start.line && line_idx <= end.line {
                    execute!(
                        writer,
                        SetBackgroundColor(Color::DarkBlue),
                        Print(line),
                        ResetColor
                    )?;
                } else {
                    execute!(writer, Print(line))?;
                }
            }
            SelectionKind::Block => {
                self.render_block_selection(writer, line, line_idx, start, end)?;
            }
        }

        Ok(())
    }

    fn render_char_selection<W: Write>(
        &self,
        writer: &mut W,
        line: &str,
        line_idx: usize,
        start: Position,
        end: Position,
    ) -> io::Result<()> {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();

        if line_idx < start.line || line_idx > end.line {
            execute!(writer, Print(line))?;
            return Ok(());
        }

        let sel_start = if line_idx == start.line { start.col } else { 0 };
        let sel_end = if line_idx == end.line { end.col + 1 } else { len };

        // Before selection
        let before: String = chars[..sel_start.min(len)].iter().collect();
        execute!(writer, Print(&before))?;

        // Selection
        let selected: String = chars[sel_start.min(len)..sel_end.min(len)].iter().collect();
        execute!(
            writer,
            SetBackgroundColor(Color::DarkBlue),
            Print(&selected),
            ResetColor
        )?;

        // After selection
        let after: String = chars[sel_end.min(len)..].iter().collect();
        execute!(writer, Print(&after))?;

        Ok(())
    }

    fn render_block_selection<W: Write>(
        &self,
        writer: &mut W,
        line: &str,
        line_idx: usize,
        start: Position,
        end: Position,
    ) -> io::Result<()> {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();

        if line_idx < start.line || line_idx > end.line {
            execute!(writer, Print(line))?;
            return Ok(());
        }

        let col_start = start.col.min(end.col);
        let col_end = start.col.max(end.col) + 1;

        let before: String = chars[..col_start.min(len)].iter().collect();
        execute!(writer, Print(&before))?;

        let selected: String = chars[col_start.min(len)..col_end.min(len)].iter().collect();
        execute!(
            writer,
            SetBackgroundColor(Color::DarkBlue),
            Print(&selected),
            ResetColor
        )?;

        let after: String = chars[col_end.min(len)..].iter().collect();
        execute!(writer, Print(&after))?;

        Ok(())
    }

    fn render_status<W: Write>(&self, writer: &mut W, snapshot: &EditorSnapshot) -> io::Result<()> {
        let height = snapshot.viewport.height;
        let width = snapshot.viewport.width as usize;

        execute!(writer, MoveTo(0, height))?;

        // Status line background
        execute!(
            writer,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;

        // If there's a command line, show it
        if let Some(ref cmd) = snapshot.status.command_line {
            let line = format!(":{}", cmd);
            let padded = format!("{:width$}", line, width = width);
            execute!(writer, Print(&padded), ResetColor)?;
            return Ok(());
        }

        // If there's a message, show it
        if let Some(ref msg) = snapshot.status.message {
            let padded = format!("{:width$}", msg, width = width);
            execute!(writer, Print(&padded), ResetColor)?;
            return Ok(());
        }

        // Normal status line
        let mode_indicator = format!(" {} ", snapshot.status.mode);
        let file_info = if snapshot.status.modified {
            format!(" {} [+]", snapshot.status.filename)
        } else {
            format!(" {}", snapshot.status.filename)
        };
        let position = format!(" {} {} ", snapshot.status.position, snapshot.status.percentage);

        let left = format!("{}{}", mode_indicator, file_info);
        let spaces = width.saturating_sub(left.len()).saturating_sub(position.len());
        let line = format!("{}{:spaces$}{}", left, "", position, spaces = spaces);

        execute!(writer, Print(&line[..line.len().min(width)]), ResetColor)?;

        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_ui::Viewport;

    #[test]
    fn renderer_creation() {
        let renderer = Renderer::new();
        drop(renderer);
    }

    #[test]
    fn renderer_render() {
        let mut buf = Vec::new();
        let renderer = Renderer::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        let result = renderer.render(&mut buf, &snapshot);
        assert!(result.is_ok());
        assert!(!buf.is_empty());
    }
}
