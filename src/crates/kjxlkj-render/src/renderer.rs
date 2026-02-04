//! Terminal renderer.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_ui::Snapshot;
use std::io::{self, Write};

/// Terminal renderer.
pub struct Renderer<W: Write> {
    writer: W,
}

impl<W: Write> Renderer<W> {
    /// Create a new renderer.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Render a snapshot.
    pub fn render(&mut self, snapshot: &Snapshot) -> io::Result<()> {
        execute!(self.writer, Hide, MoveTo(0, 0), Clear(ClearType::All))?;

        // Render text lines
        let text_height = snapshot.viewport_height.saturating_sub(2);
        for (i, line) in snapshot.lines.iter().enumerate() {
            if i >= text_height {
                break;
            }
            execute!(self.writer, MoveTo(0, i as u16))?;

            // Handle selection highlighting
            if let Some(sel) = &snapshot.selection {
                let line_idx = snapshot.top_line + i;
                self.render_line_with_selection(line, line_idx, sel, snapshot)?;
            } else {
                // Truncate line to viewport width
                let display: String = line.chars().take(snapshot.viewport_width).collect();
                execute!(self.writer, Print(&display))?;
            }
        }

        // Render empty lines with tildes
        for i in snapshot.lines.len()..text_height {
            execute!(
                self.writer,
                MoveTo(0, i as u16),
                SetForegroundColor(Color::DarkBlue),
                Print("~"),
                ResetColor
            )?;
        }

        // Render status line
        let status_row = (snapshot.viewport_height - 2) as u16;
        execute!(
            self.writer,
            MoveTo(0, status_row),
            SetAttribute(Attribute::Reverse)
        )?;

        let mode_str = snapshot.mode.display_name();
        let file_str = snapshot.file_path.as_deref().unwrap_or("[No Name]");
        let mod_str = if snapshot.modified { "[+]" } else { "" };
        let pos_str = format!(
            "{}:{} ({}/{})",
            snapshot.cursor.line + 1,
            snapshot.cursor.col + 1,
            snapshot.cursor.line + 1,
            snapshot.total_lines
        );

        let left = format!(" {} | {}{} ", mode_str, file_str, mod_str);
        let right = format!(" {} ", pos_str);
        let padding = snapshot
            .viewport_width
            .saturating_sub(left.len() + right.len());

        execute!(
            self.writer,
            Print(&left),
            Print(" ".repeat(padding)),
            Print(&right),
            SetAttribute(Attribute::Reset)
        )?;

        // Render command/status line
        let cmdline_row = (snapshot.viewport_height - 1) as u16;
        execute!(
            self.writer,
            MoveTo(0, cmdline_row),
            Clear(ClearType::CurrentLine)
        )?;

        if snapshot.mode == kjxlkj_core_types::Mode::Command {
            execute!(self.writer, Print(":"), Print(&snapshot.cmdline))?;
        } else if !snapshot.status.is_empty() {
            execute!(self.writer, Print(&snapshot.status))?;
        }

        // Position cursor
        let (row, col) = snapshot.cursor_viewport_pos();
        let cursor_row = row.min(text_height.saturating_sub(1)) as u16;
        let cursor_col = col.min(snapshot.viewport_width.saturating_sub(1)) as u16;

        if snapshot.mode == kjxlkj_core_types::Mode::Command {
            let cmd_col = (1 + snapshot.cmdline.len()) as u16;
            execute!(self.writer, MoveTo(cmd_col, cmdline_row), Show)?;
        } else {
            execute!(self.writer, MoveTo(cursor_col, cursor_row), Show)?;
        }

        self.writer.flush()
    }

    fn render_line_with_selection(
        &mut self,
        line: &str,
        line_idx: usize,
        sel: &kjxlkj_core_types::Range,
        snapshot: &Snapshot,
    ) -> io::Result<()> {
        let sel = sel.normalized();
        let in_sel = line_idx >= sel.start.line && line_idx <= sel.end.line;

        if !in_sel {
            let display: String = line.chars().take(snapshot.viewport_width).collect();
            execute!(self.writer, Print(&display))?;
            return Ok(());
        }

        let chars: Vec<char> = line.chars().collect();
        let start_col = if line_idx == sel.start.line {
            sel.start.col
        } else {
            0
        };
        let end_col = if line_idx == sel.end.line {
            sel.end.col
        } else {
            chars.len()
        };

        // Before selection
        let before: String = chars.iter().take(start_col).collect();
        execute!(self.writer, Print(&before))?;

        // Selection
        let selected: String = chars
            .iter()
            .skip(start_col)
            .take(end_col - start_col)
            .collect();
        execute!(
            self.writer,
            SetAttribute(Attribute::Reverse),
            Print(&selected),
            SetAttribute(Attribute::Reset)
        )?;

        // After selection
        let after: String = chars.iter().skip(end_col).collect();
        execute!(self.writer, Print(&after))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let buf = Vec::new();
        let _renderer = Renderer::new(buf);
    }
}
