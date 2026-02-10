//! Wrapped buffer rendering: integrates wrap algorithm into paint pipeline.
//!
//! When `wrap` is enabled for a buffer window, lines are split into
//! continuation display rows using the wrap algorithm from wrap.rs.

use kjxlkj_core_ui::WindowSnapshot;

use super::paint_window::{display_col_for_grapheme_offset, line_number_width};
use crate::grid::{Cell, CellGrid};
use crate::wrap::wrap_line;

/// Render buffer content with soft-wrap enabled.
pub fn render_buffer_wrapped(
    win: &WindowSnapshot,
    content: &ropey::Rope,
    line_count: usize,
    grid: &mut CellGrid,
    total_cols: usize,
    text_rows: usize,
    is_active: bool,
) {
    let gutter_width = if win.line_numbers {
        line_number_width(line_count)
    } else {
        0
    };
    let text_cols = total_cols.saturating_sub(gutter_width);
    if text_cols == 0 {
        return;
    }

    let mut screen_row = 0usize;
    let mut line_idx = win.top_line;

    while screen_row < text_rows && line_idx < line_count {
        let line = content.line(line_idx).to_string();
        let wrapped = wrap_line(&line, text_cols);

        for (wrap_row, wrow) in wrapped.iter().enumerate() {
            if screen_row >= text_rows {
                break;
            }

            // Gutter: show line number only on first wrapped row
            if win.line_numbers {
                if wrap_row == 0 {
                    let num_str = format!("{:>width$} ", line_idx + 1, width = gutter_width - 1);
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
                } else {
                    // Continuation row: empty gutter
                    for ci in 0..gutter_width {
                        grid.set(
                            screen_row,
                            ci,
                            Cell {
                                grapheme: " ".to_string(),
                                ..Cell::default()
                            },
                        );
                    }
                }
            }

            // Render segments
            let mut col = gutter_width;
            for seg in &wrow.segments {
                if seg.width == 0 {
                    continue;
                }
                if col + seg.width > total_cols {
                    break;
                }

                // Check cursor
                let is_cursor = is_active
                    && line_idx == win.cursor_line
                    && is_cursor_at_segment(win, &line, col - gutter_width);

                let (fg, bg) = if is_cursor {
                    ((0, 0, 0), (255, 255, 255))
                } else {
                    ((255, 255, 255), (0, 0, 0))
                };

                grid.set(
                    screen_row,
                    col,
                    Cell {
                        grapheme: seg.grapheme.clone(),
                        width: seg.width as u8,
                        fg,
                        bg,
                        ..Cell::default()
                    },
                );
                if seg.width == 2 && col + 1 < total_cols {
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
                col += seg.width;
            }

            // Padding cell for width-2 boundary
            if wrow.has_pad && col < total_cols {
                grid.set(
                    screen_row,
                    col,
                    Cell {
                        grapheme: " ".to_string(),
                        fg: (80, 80, 80),
                        bg: (0, 0, 0),
                        ..Cell::default()
                    },
                );
            }

            screen_row += 1;
        }
        line_idx += 1;
    }

    // Tilde filler for empty lines below content
    while screen_row < text_rows {
        if gutter_width > 0 {
            grid.set(
                screen_row,
                0,
                Cell {
                    grapheme: "~".to_string(),
                    fg: (100, 100, 200),
                    ..Cell::default()
                },
            );
        }
        screen_row += 1;
    }
}

/// Check if the cursor display column matches a segment's display column.
fn is_cursor_at_segment(win: &WindowSnapshot, line: &str, seg_display_col: usize) -> bool {
    let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
    let cursor_display_col = display_col_for_grapheme_offset(trimmed, win.cursor_col);
    seg_display_col == cursor_display_col
}
