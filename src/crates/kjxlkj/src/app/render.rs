//! Rendering methods for Application.

use super::Application;
use kjxlkj_core::Mode;
use std::io;

impl Application {
    /// Renders the current state.
    pub(super) fn render(&mut self) -> io::Result<()> {
        let (_width, height) = self.terminal.size();
        let height = height as usize;
        let text_height = height.saturating_sub(2);

        self.terminal.clear()?;

        for i in 0..text_height {
            let line_num = self.scroll_offset + i;
            if let Some(line) = self.buffer.line(line_num) {
                self.terminal.write_at(0, i as u16, line.trim_end_matches('\n'))?;
            } else {
                self.terminal.write_at(0, i as u16, "~")?;
            }
        }

        let status = format!(
            " {} | L{}/{} C{} ",
            self.mode,
            self.buffer.cursor_line() + 1,
            self.buffer.line_count(),
            self.buffer.cursor_col() + 1,
        );
        self.terminal.write_at(0, (height - 2) as u16, &status)?;

        let bottom = self.get_bottom_line();
        self.terminal.write_at(0, (height - 1) as u16, &bottom)?;

        let cursor_y = (self.buffer.cursor_line() - self.scroll_offset) as u16;
        self.terminal.move_cursor(self.buffer.cursor_col() as u16, cursor_y)?;
        self.terminal.show_cursor()?;
        self.terminal.flush()
    }

    pub(super) fn get_bottom_line(&self) -> String {
        if matches!(self.mode, Mode::Command(_)) {
            format!(":{}", self.command_line)
        } else if let Some(ref msg) = self.message {
            msg.clone()
        } else {
            match self.mode {
                Mode::Normal => "-- NORMAL --".to_string(),
                Mode::Insert => "-- INSERT --".to_string(),
                Mode::Visual(_) => "-- VISUAL --".to_string(),
                Mode::Replace => "-- REPLACE --".to_string(),
                _ => String::new(),
            }
        }
    }
}
