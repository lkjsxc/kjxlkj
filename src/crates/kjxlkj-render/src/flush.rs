//! Flush escape sequences to stdout.

use std::io::Write;

use kjxlkj_core_types::{CellAttrs, Color};

use crate::diff::{CellChange, FrameDiff};

/// Build a byte buffer with all escape sequences for the diff.
pub fn build_flush_buffer(diff: &FrameDiff) -> Vec<u8> {
    let mut buf = Vec::with_capacity(diff.change_count() * 20);

    // Hide cursor during update.
    buf.extend_from_slice(b"\x1b[?25l");

    let mut last_row: Option<u16> = None;
    let mut last_col: Option<u16> = None;
    let mut last_fg = Color::Default;
    let mut last_bg = Color::Default;
    let mut last_attrs = CellAttrs::empty();

    for change in &diff.changes {
        // Move cursor if not sequential.
        let need_move = match (last_row, last_col) {
            (Some(r), Some(c)) => {
                r != change.row || c + 1 != change.col
            }
            _ => true,
        };

        if need_move {
            write_cursor_pos(&mut buf, change.col, change.row);
        }

        // Set attributes.
        if change.cell.fg != last_fg
            || change.cell.bg != last_bg
            || change.cell.attrs != last_attrs
        {
            write_sgr(&mut buf, &change.cell);
            last_fg = change.cell.fg;
            last_bg = change.cell.bg;
            last_attrs = change.cell.attrs;
        }

        // Write grapheme.
        let g = change.cell.grapheme.as_str();
        if g.is_empty() {
            buf.push(b' ');
        } else {
            buf.extend_from_slice(g.as_bytes());
        }

        last_row = Some(change.row);
        last_col = Some(change.col);
    }

    // Reset attributes and show cursor.
    buf.extend_from_slice(b"\x1b[0m\x1b[?25h");

    buf
}

fn write_cursor_pos(buf: &mut Vec<u8>, col: u16, row: u16) {
    // CSI row;col H (1-based).
    let _ = write!(buf, "\x1b[{};{}H", row + 1, col + 1);
}

fn write_sgr(buf: &mut Vec<u8>, cell: &kjxlkj_core_types::Cell) {
    // Reset first.
    buf.extend_from_slice(b"\x1b[0");

    // Attributes.
    if cell.attrs.contains(CellAttrs::BOLD) {
        buf.extend_from_slice(b";1");
    }
    if cell.attrs.contains(CellAttrs::DIM) {
        buf.extend_from_slice(b";2");
    }
    if cell.attrs.contains(CellAttrs::ITALIC) {
        buf.extend_from_slice(b";3");
    }
    if cell.attrs.contains(CellAttrs::UNDERLINE) {
        buf.extend_from_slice(b";4");
    }
    if cell.attrs.contains(CellAttrs::REVERSE) {
        buf.extend_from_slice(b";7");
    }
    if cell.attrs.contains(CellAttrs::STRIKETHROUGH) {
        buf.extend_from_slice(b";9");
    }

    // Foreground color.
    write_color(buf, cell.fg, true);
    // Background color.
    write_color(buf, cell.bg, false);

    buf.push(b'm');
}

fn write_color(buf: &mut Vec<u8>, color: Color, is_fg: bool) {
    match color {
        Color::Default => {}
        Color::Indexed(n) => {
            let base = if is_fg { 38 } else { 48 };
            let _ = write!(buf, ";{};5;{}", base, n);
        }
        Color::Rgb(r, g, b_val) => {
            let base = if is_fg { 38 } else { 48 };
            let _ = write!(
                buf,
                ";{};2;{};{};{}",
                base, r, g, b_val
            );
        }
    }
}

/// Flush the buffer to stdout in a single write.
pub fn flush_to_stdout(buf: &[u8]) -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(buf)?;
    stdout.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_diff_minimal_output() {
        let diff = FrameDiff::default();
        let buf = build_flush_buffer(&diff);
        // Should contain hide/show cursor sequences.
        assert!(buf.len() > 0);
        let s = String::from_utf8_lossy(&buf);
        assert!(s.contains("\x1b[?25l"));
        assert!(s.contains("\x1b[?25h"));
    }
}
