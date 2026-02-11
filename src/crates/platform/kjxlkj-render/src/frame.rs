//! Frame building from an editor snapshot.

use kjxlkj_core_types::EditorSnapshot;

/// A rendered frame ready for painting.
pub struct Frame {
    pub rows: Vec<String>,
    pub cursor_row: u16,
    pub cursor_col: u16,
}

/// Build a frame from an editor snapshot.
pub fn render_frame(snapshot: &EditorSnapshot) -> Frame {
    let (cols, rows) = snapshot.terminal_size;
    let total_rows = rows as usize;
    let mut frame_rows = vec![String::new(); total_rows];

    // Render each window content.
    for wc in &snapshot.window_contents {
        let y_start = wc.rect.y as usize;
        let x_start = wc.rect.x as usize;
        let width = wc.rect.width as usize;
        let text_height =
            (wc.rect.height as usize).saturating_sub(1);

        for (i, line) in wc.lines.iter().enumerate() {
            let row_idx = y_start + i;
            if row_idx >= total_rows {
                break;
            }
            // Pad/truncate line to window width.
            let display: String = line
                .chars()
                .take(width)
                .collect();
            let padded = format!(
                "{:<width$}",
                display,
                width = width
            );
            write_at(&mut frame_rows[row_idx], x_start, &padded);
        }

        // Fill empty lines with ~.
        for i in wc.lines.len()..text_height {
            let row_idx = y_start + i;
            if row_idx >= total_rows {
                break;
            }
            let tilde = format!(
                "{:<width$}",
                "~",
                width = width
            );
            write_at(&mut frame_rows[row_idx], x_start, &tilde);
        }

        // Statusline at bottom of window.
        let status_row = y_start + text_height;
        if status_row < total_rows {
            let st = format!(
                "{:<width$}",
                wc.statusline,
                width = width
            );
            write_at(
                &mut frame_rows[status_row],
                x_start,
                &st,
            );
        }
    }

    // Command line at last row.
    let cmd_row = total_rows.saturating_sub(1);
    if snapshot.cmdline.active {
        let cmd_display = format!(
            "{}{}",
            snapshot.cmdline.prefix,
            snapshot.cmdline.content
        );
        frame_rows[cmd_row] = format!(
            "{:<width$}",
            cmd_display,
            width = cols as usize
        );
    }

    // Find cursor position from focused window.
    let (cur_row, cur_col) = snapshot
        .window_contents
        .iter()
        .find(|wc| wc.window_id == snapshot.focused_window)
        .map(|wc| {
            (
                wc.rect.y as u16 + wc.cursor_row as u16,
                wc.rect.x as u16 + wc.cursor_col as u16,
            )
        })
        .unwrap_or((0, 0));

    Frame {
        rows: frame_rows,
        cursor_row: cur_row,
        cursor_col: cur_col,
    }
}

fn write_at(row: &mut String, col: usize, text: &str) {
    // Extend row if needed.
    while row.len() < col {
        row.push(' ');
    }
    if col < row.len() {
        // Replace from col.
        let prefix: String =
            row.chars().take(col).collect();
        *row = format!("{}{}", prefix, text);
    } else {
        row.push_str(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{
        CmdlineState, Mode, Rect, WindowContent, WindowId,
    };

    fn test_snapshot() -> EditorSnapshot {
        EditorSnapshot {
            sequence: 0,
            mode: Mode::Normal,
            terminal_size: (80, 24),
            cmdline: CmdlineState::default(),
            notifications: Vec::new(),
            layout_summary: "windows=1 focused=1".into(),
            focused_window: WindowId(1),
            window_contents: vec![WindowContent {
                window_id: WindowId(1),
                rect: Rect::new(0, 0, 80, 24),
                lines: vec!["hello".into()],
                cursor_row: 0,
                cursor_col: 0,
                window_type: "buffer".into(),
                statusline: " NORMAL | test ".into(),
            }],
        }
    }

    #[test]
    fn frame_has_correct_rows() {
        let snap = test_snapshot();
        let frame = render_frame(&snap);
        assert_eq!(frame.rows.len(), 24);
        assert!(frame.rows[0].starts_with("hello"));
    }
}
