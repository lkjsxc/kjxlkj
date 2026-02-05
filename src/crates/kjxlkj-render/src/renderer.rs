//! Rendering pipeline (snapshot to terminal frame).

use crossterm::{
    cursor::{self, MoveTo},
    queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{Clear, ClearType},
};
use kjxlkj_core_types::{CursorShape, Mode};
use kjxlkj_core_ui::{EditorSnapshot, WindowSnapshot};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Renderer for terminal output.
pub struct Renderer {
    /// Last rendered snapshot sequence.
    last_seq: u64,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Create new renderer.
    pub fn new() -> Self {
        Self { last_seq: 0 }
    }

    /// Render a snapshot to the terminal.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        if snapshot.seq.0 <= self.last_seq {
            return Ok(());
        }
        self.last_seq = snapshot.seq.0;

        let mut stdout = io::stdout();

        queue!(stdout, cursor::Hide)?;
        queue!(stdout, Clear(ClearType::All))?;

        for window in &snapshot.windows {
            self.render_window(&mut stdout, window)?;
        }

        self.render_statusline(&mut stdout, snapshot)?;
        self.render_cmdline(&mut stdout, snapshot)?;

        if let Some(window) = snapshot.windows.iter().find(|w| w.focused) {
            let cursor_row = window.cursor.line.saturating_sub(window.viewport.top_line);
            let gutter_width = if window.show_numbers { 5 } else { 0 };
            let cursor_col = window.cursor.column + gutter_width;

            if cursor_row < window.viewport.text_rows as usize {
                queue!(stdout, MoveTo(cursor_col as u16, cursor_row as u16))?;

                match window.cursor_shape {
                    CursorShape::Block => {
                        queue!(stdout, cursor::SetCursorStyle::SteadyBlock)?;
                    }
                    CursorShape::Bar => {
                        queue!(stdout, cursor::SetCursorStyle::SteadyBar)?;
                    }
                    CursorShape::Underline => {
                        queue!(stdout, cursor::SetCursorStyle::SteadyUnderScore)?;
                    }
                    CursorShape::Hollow => {
                        queue!(stdout, cursor::SetCursorStyle::SteadyBlock)?;
                    }
                }
                queue!(stdout, cursor::Show)?;
            }
        }

        stdout.flush()
    }

    fn render_window(&self, stdout: &mut io::Stdout, window: &WindowSnapshot) -> io::Result<()> {
        let gutter_width = if window.show_numbers { 5 } else { 0 };

        for (idx, line) in window.lines.iter().enumerate() {
            let row = window.rect.y + idx as u16;
            queue!(stdout, MoveTo(window.rect.x, row))?;

            if window.show_numbers {
                queue!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("{:>4} ", line.line_number)),
                    ResetColor
                )?;
            }

            let display_width = window.viewport.text_cols as usize - gutter_width;
            let text = if line.text.width() > display_width {
                let mut result = String::new();
                let mut width = 0;
                for c in line.text.chars() {
                    let cw = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                    if width + cw > display_width {
                        break;
                    }
                    result.push(c);
                    width += cw;
                }
                result
            } else {
                line.text.clone()
            };

            if let Some(sel) = &window.selection {
                let line_idx = window.viewport.top_line + idx;
                let sel_start = sel.start();
                let sel_end = sel.end();

                if line_idx >= sel_start.line && line_idx <= sel_end.line {
                    let start_col = if line_idx == sel_start.line {
                        sel_start.col
                    } else {
                        0
                    };
                    let end_col = if line_idx == sel_end.line {
                        sel_end.col + 1
                    } else {
                        text.len()
                    };

                    let before = &text[..start_col.min(text.len())];
                    let selected = &text[start_col.min(text.len())..end_col.min(text.len())];
                    let after = &text[end_col.min(text.len())..];

                    queue!(stdout, Print(before))?;
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::DarkBlue),
                        Print(selected),
                        ResetColor
                    )?;
                    queue!(stdout, Print(after))?;
                } else {
                    queue!(stdout, Print(&text))?;
                }
            } else {
                queue!(stdout, Print(&text))?;
            }
        }

        for idx in window.lines.len()..window.viewport.text_rows as usize {
            let row = window.rect.y + idx as u16;
            queue!(stdout, MoveTo(window.rect.x, row))?;
            queue!(
                stdout,
                SetForegroundColor(Color::DarkBlue),
                Print("~"),
                ResetColor
            )?;
        }

        Ok(())
    }

    fn render_statusline(
        &self,
        stdout: &mut io::Stdout,
        snapshot: &EditorSnapshot,
    ) -> io::Result<()> {
        let row = snapshot.terminal_size.rows.saturating_sub(2);
        queue!(stdout, MoveTo(0, row))?;

        let mode_bg = match snapshot.mode {
            Mode::Normal => Color::DarkGreen,
            Mode::Insert => Color::DarkBlue,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => Color::DarkMagenta,
            Mode::Replace => Color::DarkRed,
            Mode::Command => Color::DarkYellow,
        };

        queue!(
            stdout,
            SetBackgroundColor(mode_bg),
            SetForegroundColor(Color::White),
            SetAttribute(Attribute::Bold),
            Print(&snapshot.statusline.left),
            ResetColor
        )?;

        let center_width = snapshot.terminal_size.cols as usize
            - snapshot.statusline.left.len()
            - snapshot.statusline.right.len();
        let center = format!(
            "{:^width$}",
            &snapshot.statusline.center,
            width = center_width
        );

        queue!(
            stdout,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print(center),
            Print(&snapshot.statusline.right),
            ResetColor
        )?;

        Ok(())
    }

    fn render_cmdline(&self, stdout: &mut io::Stdout, snapshot: &EditorSnapshot) -> io::Result<()> {
        let row = snapshot.terminal_size.rows.saturating_sub(1);
        queue!(stdout, MoveTo(0, row))?;

        if let Some(ref cmdline) = snapshot.cmdline {
            queue!(stdout, Print(cmdline.prompt), Print(&cmdline.input))?;
        } else if let Some(ref message) = snapshot.message {
            queue!(stdout, Print(message))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new();
        assert_eq!(renderer.last_seq, 0);
    }
}
