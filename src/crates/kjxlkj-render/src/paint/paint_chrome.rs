//! Statusline and command-line rendering.

use kjxlkj_core_ui::{EditorSnapshot, WindowContent};

use crate::grid::{Cell, CellGrid};

/// Render the statusline at the given row.
pub fn render_statusline(snapshot: &EditorSnapshot, grid: &mut CellGrid, row: usize, cols: usize) {
    let tab = &snapshot.tabs[snapshot.active_tab];
    let win = &tab.windows[tab.active_window];
    let mode_str = snapshot.mode.to_string();

    let (file_name, modified) = match &win.content {
        WindowContent::Buffer(id) => snapshot
            .buffers
            .get(id)
            .map_or(("[No Name]".to_string(), false), |b| {
                (b.name.clone(), b.modified)
            }),
        WindowContent::Terminal(_) => ("[Terminal]".to_string(), false),
        WindowContent::Explorer => ("[Explorer]".to_string(), false),
    };

    let mod_flag = if modified { " [+]" } else { "" };
    let pos = format!("{}:{} ", win.cursor_line + 1, win.cursor_col + 1,);
    let left = format!(" {mode_str} | {file_name}{mod_flag} ");
    let right = pos;

    for c in 0..cols {
        let ch = if c < left.len() {
            left.chars().nth(c).unwrap_or(' ')
        } else if c >= cols - right.len() {
            let ri = c - (cols - right.len());
            right.chars().nth(ri).unwrap_or(' ')
        } else {
            ' '
        };
        grid.set(
            row,
            c,
            Cell {
                grapheme: ch.to_string(),
                fg: (0, 0, 0),
                bg: (200, 200, 200),
                ..Cell::default()
            },
        );
    }
}

/// Render the command line at the given row.
pub fn render_cmdline(snapshot: &EditorSnapshot, grid: &mut CellGrid, row: usize, cols: usize) {
    let content = if snapshot.cmdline.visible {
        format!("{}{}", snapshot.cmdline.prefix, snapshot.cmdline.content)
    } else if let Some(notif) = snapshot.notifications.last() {
        notif.message.clone()
    } else {
        String::new()
    };

    for (c, ch) in content.chars().enumerate() {
        if c >= cols {
            break;
        }
        grid.set(
            row,
            c,
            Cell {
                grapheme: ch.to_string(),
                ..Cell::default()
            },
        );
    }
}
