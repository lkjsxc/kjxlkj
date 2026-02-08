//! Gutter rendering (line numbers, sign column).

use kjxlkj_core_types::{CellAttrs, Color};
use kjxlkj_core_ui::ThemeColor;

use crate::CellGrid;

/// Render line numbers in the gutter area.
pub fn render_gutter(
    grid: &mut CellGrid,
    x: u16,
    y: u16,
    gutter_width: u16,
    top_line: usize,
    line_count: usize,
    cursor_line: usize,
    height: u16,
    number_color: ThemeColor,
    cursor_line_color: ThemeColor,
) {
    for row in 0..height {
        let line = top_line + row as usize;
        if line < line_count {
            let is_cursor = line == cursor_line;
            let color = if is_cursor {
                cursor_line_color
            } else {
                number_color
            };

            let line_str = format!("{:>w$} ", line + 1, w = (gutter_width - 1) as usize);
            let display = if line_str.len() > gutter_width as usize {
                &line_str[..gutter_width as usize]
            } else {
                &line_str
            };

            grid.write_str(
                x,
                y + row,
                display,
                color.fg,
                color.bg,
                if is_cursor {
                    CellAttrs::BOLD
                } else {
                    CellAttrs::empty()
                },
            );
        } else {
            // Tilde for non-existent lines.
            let mut tilde = String::from("~");
            while tilde.len() < gutter_width as usize {
                tilde.push(' ');
            }
            grid.write_str(
                x,
                y + row,
                &tilde,
                Color::Indexed(8),
                Color::Default,
                CellAttrs::empty(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter_line_numbers() {
        let mut grid = CellGrid::new(80, 24);
        render_gutter(
            &mut grid,
            0,
            0,
            4,
            0,
            100,
            5,
            10,
            ThemeColor::default(),
            ThemeColor::default(),
        );
        // Line 1 should be in the gutter.
        let cell = grid.get(0, 0);
        assert!(!cell.grapheme.is_empty());
    }
}
