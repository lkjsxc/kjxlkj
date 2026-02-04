//! Status line rendering.

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use kjxlkj_core_ui::EditorSnapshot;
use std::io::{self, Write};

/// Render the status line.
pub fn render<W: Write>(writer: &mut W, snapshot: &EditorSnapshot) -> io::Result<()> {
    let height = snapshot.viewport.height;
    let width = snapshot.viewport.width as usize;

    execute!(writer, MoveTo(0, height))?;
    execute!(writer, SetBackgroundColor(Color::DarkGrey), SetForegroundColor(Color::White))?;

    // If there's a command line, show it
    if let Some(ref cmd) = snapshot.status.command_line {
        let line = format!(":{}", cmd);
        let padded = format!("{:width$}", line, width = width);
        return execute!(writer, Print(&padded), ResetColor);
    }

    // If there's a message, show it
    if let Some(ref msg) = snapshot.status.message {
        let padded = format!("{:width$}", msg, width = width);
        return execute!(writer, Print(&padded), ResetColor);
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

    execute!(writer, Print(&line[..line.len().min(width)]), ResetColor)
}
