use crate::cell::{Cell, CellGrid};
use crate::grid_window::render_window;
use kjxlkj_core_types::ContentSource;
use kjxlkj_core_ui::{Color, EditorSnapshot, Style};

/// Build a cell grid from an editor snapshot.
pub fn build_grid(snapshot: &EditorSnapshot) -> CellGrid {
    let (cols, rows) = snapshot.terminal_size;
    let mut grid = CellGrid::new(cols, rows);

    if let Some(tab) = snapshot.tabs.get(snapshot.active_tab) {
        for (wid, ws) in &tab.windows {
            render_window(
                &mut grid,
                ws,
                &snapshot.buffers,
                *wid == snapshot.focused_window,
                &snapshot.theme.default_style,
                &snapshot.theme.line_number_style,
                &snapshot.theme.cursor_style,
            );
        }
    }

    if rows >= 2 {
        render_statusline(&mut grid, cols, rows - 2, snapshot);
    }

    if rows >= 1 {
        render_cmdline(&mut grid, cols, rows - 1, snapshot);
    }

    grid
}

fn render_statusline(grid: &mut CellGrid, cols: u16, row: u16, snapshot: &EditorSnapshot) {
    let style = snapshot.theme.statusline_style;

    for col in 0..cols {
        grid.set(
            col,
            row,
            Cell {
                grapheme: " ".to_string(),
                width: 1,
                style,
                is_wide_continuation: false,
            },
        );
    }

    let mode_str = format!(" {} ", snapshot.mode);
    grid.set_str(0, row, &mode_str, style);

    let tab = snapshot.tabs.get(snapshot.active_tab);
    let buf_name = if let Some(tab) = tab {
        if let Some(ws) = tab.windows.get(&snapshot.focused_window) {
            if let ContentSource::Buffer(bid) = &ws.content {
                snapshot
                    .buffers
                    .get(bid)
                    .map(|b| {
                        let modified = if b.modified { "[+]" } else { "" };
                        format!(" {}{} ", b.name, modified)
                    })
                    .unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let mode_len = mode_str.len() as u16;
    grid.set_str(mode_len, row, &buf_name, style);

    if let Some(tab) = tab {
        if let Some(ws) = tab.windows.get(&snapshot.focused_window) {
            let pos_str = format!(" {}:{} ", ws.cursor.line + 1, ws.cursor.grapheme + 1);
            let pos_len = pos_str.len() as u16;
            if cols > pos_len {
                grid.set_str(cols - pos_len, row, &pos_str, style);
            }
        }
    }
}

fn render_cmdline(grid: &mut CellGrid, _cols: u16, row: u16, snapshot: &EditorSnapshot) {
    if snapshot.cmdline.active {
        let prefix = snapshot
            .cmdline
            .prefix
            .map(|c| c.to_string())
            .unwrap_or_default();
        let text = format!("{}{}", prefix, snapshot.cmdline.content);
        grid.set_str(0, row, &text, snapshot.theme.cmdline_style);
    } else if let Some(notif) = snapshot.notifications.last() {
        let style = match notif.level {
            kjxlkj_core_ui::NotificationLevel::Error => Style {
                fg: Color::Rgb(255, 0, 0),
                ..Style::default()
            },
            kjxlkj_core_ui::NotificationLevel::Warning => Style {
                fg: Color::Rgb(255, 255, 0),
                ..Style::default()
            },
            _ => Style::default(),
        };
        grid.set_str(0, row, &notif.message, style);
    }
}
