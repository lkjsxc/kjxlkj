//! CSI sequence dispatch.

use crate::parser::Parser;
use crate::screen::Screen;

/// Dispatch a CSI final byte.
pub fn dispatch(final_byte: u8, parser: &Parser, screen: &mut Screen) {
    let params = &parser.params;
    let private = parser.private_marker;

    match final_byte {
        b'A' => {
            screen.cursor_row = screen
                .cursor_row
                .saturating_sub(param_default(params, 0, 1))
        }
        b'B' => {
            let n = param_default(params, 0, 1);
            screen.cursor_row = (screen.cursor_row + n).min(screen.rows.saturating_sub(1));
        }
        b'C' => {
            let n = param_default(params, 0, 1);
            screen.cursor_col = (screen.cursor_col + n).min(screen.cols.saturating_sub(1));
        }
        b'D' => {
            screen.cursor_col = screen
                .cursor_col
                .saturating_sub(param_default(params, 0, 1))
        }
        b'E' => {
            let n = param_default(params, 0, 1);
            screen.cursor_row = (screen.cursor_row + n).min(screen.rows.saturating_sub(1));
            screen.cursor_col = 0;
        }
        b'F' => {
            screen.cursor_row = screen
                .cursor_row
                .saturating_sub(param_default(params, 0, 1));
            screen.cursor_col = 0;
        }
        b'G' => {
            let col = param_default(params, 0, 1).saturating_sub(1);
            screen.cursor_col = col.min(screen.cols.saturating_sub(1));
        }
        b'H' | b'f' => {
            let row = param_default(params, 0, 1).saturating_sub(1);
            let col = param_default(params, 1, 1).saturating_sub(1);
            screen.move_cursor(row, col);
        }
        b'J' => screen.erase_display(param_default(params, 0, 0)),
        b'K' => screen.erase_line(param_default(params, 0, 0)),
        b'L' => {
            let n = param_default(params, 0, 1);
            for _ in 0..n {
                if screen.cursor_row <= screen.scroll_bottom {
                    if screen.scroll_bottom < screen.cells.len() {
                        screen.cells.remove(screen.scroll_bottom);
                    }
                    let blank = vec![crate::cell::Cell::default(); screen.cols];
                    screen.cells.insert(screen.cursor_row, blank);
                }
            }
        }
        b'M' => {
            let n = param_default(params, 0, 1);
            for _ in 0..n {
                if screen.cursor_row <= screen.scroll_bottom {
                    screen.cells.remove(screen.cursor_row);
                    let blank = vec![crate::cell::Cell::default(); screen.cols];
                    let pos = screen.scroll_bottom.min(screen.cells.len());
                    screen.cells.insert(pos, blank);
                }
            }
        }
        b'P' => {
            let n = param_default(params, 0, 1);
            let row = screen.cursor_row;
            for _ in 0..n {
                if screen.cursor_col < screen.cols {
                    screen.cells[row].remove(screen.cursor_col);
                    screen.cells[row].push(crate::cell::Cell::default());
                }
            }
        }
        b'S' => screen.scroll_up(param_default(params, 0, 1)),
        b'T' => screen.scroll_down(param_default(params, 0, 1)),
        b'X' => {
            let n = param_default(params, 0, 1);
            for i in 0..n {
                let col = screen.cursor_col + i;
                if col < screen.cols {
                    screen.cells[screen.cursor_row][col] = crate::cell::Cell::default();
                }
            }
        }
        b'd' => {
            let row = param_default(params, 0, 1).saturating_sub(1);
            screen.cursor_row = row.min(screen.rows.saturating_sub(1));
        }
        b'h' if private => dispatch_private_set(params, screen, true),
        b'l' if private => dispatch_private_set(params, screen, false),
        b'm' => crate::sgr::dispatch_sgr(params, screen),
        b'r' => {
            let top = param_default(params, 0, 1).saturating_sub(1);
            let bottom = param_default(params, 1, screen.rows).saturating_sub(1);
            screen.scroll_top = top.min(screen.rows.saturating_sub(1));
            screen.scroll_bottom = bottom.min(screen.rows.saturating_sub(1));
        }
        b's' => screen.saved_cursor = Some((screen.cursor_row, screen.cursor_col)),
        b'u' => {
            if let Some((r, c)) = screen.saved_cursor {
                screen.move_cursor(r, c);
            }
        }
        _ => {} // Ignore unknown
    }
}

fn param_default(params: &[u16], idx: usize, default: usize) -> usize {
    params
        .get(idx)
        .copied()
        .map(|v| if v == 0 { default } else { v as usize })
        .unwrap_or(default)
}

fn dispatch_private_set(params: &[u16], screen: &mut Screen, set: bool) {
    for &p in params {
        match p {
            7 => screen.auto_wrap = set,
            25 => screen.cursor_visible = set,
            1049 => {
                // Alt screen: simplified - just clear on enter
                if set {
                    screen.saved_cursor = Some((screen.cursor_row, screen.cursor_col));
                    screen.erase_display(2);
                    screen.move_cursor(0, 0);
                } else if let Some((r, c)) = screen.saved_cursor {
                    screen.move_cursor(r, c);
                }
            }
            _ => {} // Track but ignore other modes
        }
    }
}
