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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_ui::{Viewport, StatusLine};

    #[test]
    fn render_status_line() {
        let mut buf = Vec::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_line_with_command() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.command_line = Some("write".to_string());
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_line_with_message() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.message = Some("File saved".to_string());
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_modified_file() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.modified = true;
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_narrow_viewport() {
        let mut buf = Vec::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(20, 10));
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_wide_viewport() {
        let mut buf = Vec::new();
        let snapshot = EditorSnapshot::empty(Viewport::new(200, 50));
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_command_mode() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.command_line = Some(":q".to_string());
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_long_filename() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.filename = "very/long/path/to/file.rs".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_position_max() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.position = "99999:999".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_percentage_100() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.percentage = "100%".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_insert_mode() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.mode = "INSERT".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_visual_mode() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.mode = "VISUAL".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_percentage_0() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.percentage = "0%".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn render_status_position_1_1() {
        let mut buf = Vec::new();
        let mut snapshot = EditorSnapshot::empty(Viewport::new(80, 24));
        snapshot.status.position = "1:1".to_string();
        let result = render(&mut buf, &snapshot);
        assert!(result.is_ok());
    }
}
