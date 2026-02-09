//! Statusline rendering.

use kjxlkj_core_types::{CellAttrs, Mode};
use kjxlkj_core_ui::ThemeColor;
use kjxlkj_core_ui::BufferSnapshot;

use crate::CellGrid;

/// Mode display string.
pub fn mode_string(mode: &Mode) -> &'static str {
    match mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
        Mode::Replace => "REPLACE",
        Mode::Visual(_) => "VISUAL",
        Mode::Command(_) => "COMMAND",
        Mode::OperatorPending(_) => "OP-PENDING",
        Mode::TerminalInsert => "TERMINAL",
        Mode::InsertNormal => "INS-NOR",
    }
}

/// Render the statusline for a window.
pub fn render_statusline(
    grid: &mut CellGrid,
    y: u16,
    width: u16,
    mode: &Mode,
    buf: &BufferSnapshot,
    is_focused: bool,
    color: ThemeColor,
    mode_color: ThemeColor,
) {
    // Clear the row.
    for col in 0..width {
        let mut cell = kjxlkj_core_types::Cell::default();
        cell.fg = color.fg;
        cell.bg = color.bg;
        cell.grapheme = compact_str::CompactString::from(" ");
        cell.width = 1;
        grid.set(col, y, cell);
    }

    let mut x: u16 = 0;

    // Mode indicator (only for focused window).
    if is_focused {
        let mode_str = format!(" {} ", mode_string(mode));
        grid.write_str(
            x,
            y,
            &mode_str,
            mode_color.fg,
            mode_color.bg,
            CellAttrs::BOLD,
        );
        x += mode_str.len() as u16;
    }

    // File name.
    let name = if buf.modified {
        format!(" {} [+]", buf.name)
    } else {
        format!(" {}", buf.name)
    };
    grid.write_str(
        x,
        y,
        &name,
        color.fg,
        color.bg,
        CellAttrs::empty(),
    );

    // Right-aligned: cursor position, file type, encoding.
    let right = format!(
        "{} | {} | {} ",
        buf.cursor_display(),
        buf.file_type,
        buf.line_ending,
    );
    let right_x = width.saturating_sub(right.len() as u16);
    grid.write_str(
        right_x,
        y,
        &right,
        color.fg,
        color.bg,
        CellAttrs::empty(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_strings() {
        assert_eq!(mode_string(&Mode::Normal), "NORMAL");
        assert_eq!(mode_string(&Mode::Insert), "INSERT");
    }
}
