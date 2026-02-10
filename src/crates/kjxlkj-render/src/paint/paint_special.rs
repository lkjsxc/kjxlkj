//! Terminal and explorer window rendering.

use kjxlkj_core_ui::{EditorSnapshot, WindowContent, WindowSnapshot};

use crate::grid::{Cell, CellGrid};

/// Render terminal window content from snapshot.
pub fn render_terminal_window(
    win: &WindowSnapshot,
    snapshot: &EditorSnapshot,
    grid: &mut CellGrid,
    total_cols: usize,
    text_rows: usize,
    is_active: bool,
) {
    if let WindowContent::Terminal(tid) = &win.content {
        if let Some(term) = snapshot.terminals.get(tid) {
            for r in 0..text_rows.min(term.rows as usize) {
                if r < term.cells.len() {
                    for (c, cell) in term.cells[r].iter().enumerate() {
                        if c >= total_cols {
                            break;
                        }
                        let is_cursor = is_active && r == term.cursor_row && c == term.cursor_col;
                        let (fg, bg) = if is_cursor {
                            ((0, 0, 0), (255, 255, 255))
                        } else {
                            (
                                (cell.fg.r, cell.fg.g, cell.fg.b),
                                (cell.bg.r, cell.bg.g, cell.bg.b),
                            )
                        };
                        grid.set(
                            r,
                            c,
                            Cell {
                                grapheme: cell.grapheme.clone(),
                                width: cell.width,
                                fg,
                                bg,
                                ..Cell::default()
                            },
                        );
                    }
                }
            }
        }
    }
}

/// Render explorer window content.
pub fn render_explorer_window(grid: &mut CellGrid, total_cols: usize) {
    let header = "[Explorer]";
    for (c, ch) in header.chars().enumerate() {
        if c >= total_cols {
            break;
        }
        grid.set(
            0,
            c,
            Cell {
                grapheme: ch.to_string(),
                fg: (100, 200, 100),
                bg: (30, 30, 30),
                ..Cell::default()
            },
        );
    }
}
