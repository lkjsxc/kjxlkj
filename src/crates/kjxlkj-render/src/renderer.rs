//! Terminal renderer.

use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Attribute, Print, SetAttribute},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use kjxlkj_core_ui::EditorSnapshot;

/// Renders editor snapshots to the terminal.
pub struct Renderer<W: Write> {
    out: W,
}

impl<W: Write> Renderer<W> {
    /// Creates a new renderer.
    pub fn new(out: W) -> Self {
        Self { out }
    }

    /// Renders a snapshot to the terminal.
    pub fn render(&mut self, snapshot: &EditorSnapshot) -> io::Result<()> {
        let (width, height) = snapshot.terminal_size;
        let viewport = &snapshot.buffer.viewport;

        self.out.queue(Hide)?;
        self.out.queue(MoveTo(0, 0))?;
        self.out.queue(Clear(ClearType::All))?;

        let text_height = height.saturating_sub(2) as usize;
        for row in 0..text_height {
            let line_idx = viewport.top_line + row;
            self.out.queue(MoveTo(0, row as u16))?;
            if line_idx < snapshot.buffer.lines.len() {
                let line = &snapshot.buffer.lines[line_idx];
                let display: String = line
                    .chars()
                    .skip(viewport.left_col)
                    .take(width as usize)
                    .collect();
                self.out.queue(Print(&display))?;
            } else {
                self.out.queue(Print("~"))?;
            }
        }

        self.render_status_line(snapshot, width, height)?;
        self.render_command_line(snapshot, width, height)?;

        let cursor_row = snapshot
            .buffer
            .cursor
            .position
            .line
            .saturating_sub(viewport.top_line as u32);
        let cursor_col = snapshot
            .buffer
            .cursor
            .position
            .col
            .saturating_sub(viewport.left_col as u32);
        self.out.queue(MoveTo(cursor_col as u16, cursor_row as u16))?;
        self.out.queue(Show)?;
        self.out.flush()?;

        Ok(())
    }

    fn render_status_line(
        &mut self,
        snapshot: &EditorSnapshot,
        width: u16,
        height: u16,
    ) -> io::Result<()> {
        let status_row = height.saturating_sub(2);
        self.out.queue(MoveTo(0, status_row))?;
        self.out.queue(SetAttribute(Attribute::Reverse))?;

        let mode_str = snapshot.status.mode.name();
        let modified = if snapshot.status.modified { "[+]" } else { "" };
        let left = format!(
            " {} {} {}",
            mode_str, snapshot.status.file_name, modified
        );
        let right = format!(
            "{}:{} / {} ",
            snapshot.status.cursor_line,
            snapshot.status.cursor_col,
            snapshot.status.line_count
        );
        let padding = width as usize - left.len() - right.len();
        let line = format!("{}{:padding$}{}", left, "", right, padding = padding);
        self.out.queue(Print(&line[..(width as usize).min(line.len())]))?;
        self.out.queue(SetAttribute(Attribute::Reset))?;

        Ok(())
    }

    fn render_command_line(
        &mut self,
        snapshot: &EditorSnapshot,
        width: u16,
        height: u16,
    ) -> io::Result<()> {
        let cmd_row = height.saturating_sub(1);
        self.out.queue(MoveTo(0, cmd_row))?;

        if let Some(ref cmd) = snapshot.command_line {
            let display: String = cmd.chars().take(width as usize).collect();
            self.out.queue(Print(display))?;
        } else if let Some(ref msg) = snapshot.status.message {
            let display: String = msg.chars().take(width as usize).collect();
            self.out.queue(Print(display))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_ui::{BufferSnapshot, StatusLine, Viewport};
    use kjxlkj_core_types::{BufferId, BufferVersion, Cursor};

    #[test]
    fn render_empty_snapshot() {
        let mut buf = Vec::new();
        let mut renderer = Renderer::new(&mut buf);

        let snapshot = EditorSnapshot {
            buffer: BufferSnapshot {
                id: BufferId::new(1),
                version: BufferVersion::new(0),
                name: "test".to_string(),
                lines: vec!["Hello".to_string()],
                cursor: Cursor::origin(),
                viewport: Viewport::new(0, 10, 0, 80),
                modified: false,
            },
            status: StatusLine::default(),
            command_line: None,
            terminal_size: (80, 24),
        };

        assert!(renderer.render(&snapshot).is_ok());
    }
}
