//! Window content rendering onto the cell grid.

use kjxlkj_core_ui::{EditorSnapshot, WindowContent, WindowSnapshot};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::grid::{Cell, CellGrid};

/// Render a window's content into the grid.
pub fn render_window(
    win: &WindowSnapshot,
    snapshot: &EditorSnapshot,
    grid: &mut CellGrid,
    total_cols: usize,
    total_rows: usize,
    is_active: bool,
) {
    let text_rows = total_rows.saturating_sub(2);
    match &win.content {
        WindowContent::Buffer(buf_id) => {
            if let Some(buf) = snapshot.buffers.get(buf_id) {
                if win.wrap {
                    // Delegate to wrapped renderer for soft-wrap mode
                    super::paint_wrap::render_buffer_wrapped(
                        win,
                        &buf.content,
                        buf.line_count,
                        grid,
                        total_cols,
                        text_rows,
                        is_active,
                    );
                    return;
                }
                let gutter_width = if win.line_numbers {
                    line_number_width(buf.line_count)
                } else {
                    0
                };
                let _text_cols = total_cols.saturating_sub(gutter_width);

                for screen_row in 0..text_rows {
                    let line_idx = win.top_line + screen_row;
                    if line_idx >= buf.line_count {
                        if gutter_width > 0 {
                            let tilde = Cell {
                                grapheme: "~".to_string(),
                                fg: (100, 100, 200),
                                ..Cell::default()
                            };
                            grid.set(screen_row, 0, tilde);
                        }
                        continue;
                    }

                    // Gutter: line number
                    if win.line_numbers {
                        let num_str =
                            format!("{:>width$} ", line_idx + 1, width = gutter_width - 1);
                        for (ci, ch) in num_str.chars().enumerate() {
                            if ci < gutter_width {
                                grid.set(
                                    screen_row,
                                    ci,
                                    Cell {
                                        grapheme: ch.to_string(),
                                        fg: (200, 200, 100),
                                        ..Cell::default()
                                    },
                                );
                            }
                        }
                    }

                    // Text content
                    let line = buf.content.line(line_idx).to_string();
                    let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
                    let mut col = gutter_width;
                    for grapheme in trimmed.graphemes(true) {
                        let w = UnicodeWidthStr::width(grapheme);
                        if col + w > total_cols {
                            break;
                        }
                        let is_cursor = is_active
                            && line_idx == win.cursor_line
                            && col - gutter_width
                                == display_col_for_grapheme_offset(trimmed, win.cursor_col);
                        let (fg, bg) = if is_cursor {
                            ((0, 0, 0), (255, 255, 255))
                        } else {
                            ((255, 255, 255), (0, 0, 0))
                        };
                        grid.set(
                            screen_row,
                            col,
                            Cell {
                                grapheme: grapheme.to_string(),
                                width: w as u8,
                                fg,
                                bg,
                                ..Cell::default()
                            },
                        );
                        if w == 2 && col + 1 < total_cols {
                            grid.set(
                                screen_row,
                                col + 1,
                                Cell {
                                    grapheme: String::new(),
                                    width: 0,
                                    is_wide_continuation: true,
                                    fg,
                                    bg,
                                    ..Cell::default()
                                },
                            );
                        }
                        col += w;
                    }

                    // Cursor on empty part of line
                    if is_active
                        && line_idx == win.cursor_line
                        && col
                            <= gutter_width
                                + display_col_for_grapheme_offset(trimmed, win.cursor_col)
                    {
                        let cursor_col =
                            gutter_width + display_col_for_grapheme_offset(trimmed, win.cursor_col);
                        if cursor_col < total_cols {
                            grid.set(
                                screen_row,
                                cursor_col,
                                Cell {
                                    grapheme: " ".to_string(),
                                    fg: (0, 0, 0),
                                    bg: (255, 255, 255),
                                    ..Cell::default()
                                },
                            );
                        }
                    }
                }
            }
        }
        WindowContent::Terminal(_tid) => {
            super::paint_special::render_terminal_window(
                win, snapshot, grid, total_cols, text_rows, is_active,
            );
        }
        WindowContent::Explorer => {
            super::paint_special::render_explorer_window(grid, total_cols);
        }
    }
}

/// Convert a grapheme offset to a display column.
pub fn display_col_for_grapheme_offset(line: &str, offset: usize) -> usize {
    line.graphemes(true)
        .take(offset)
        .map(UnicodeWidthStr::width)
        .sum()
}

/// Compute gutter width for line numbers.
pub fn line_number_width(max_line: usize) -> usize {
    let digits = if max_line == 0 {
        1
    } else {
        (max_line as f64).log10().floor() as usize + 1
    };
    digits + 1
}
