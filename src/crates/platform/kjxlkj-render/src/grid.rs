//! Grid building utilities.

use kjxlkj_core_ui::{Cell, CellGrid, EditorSnapshot, Style};
use unicode_width::UnicodeWidthStr;

/// Build a cell grid from snapshot.
pub fn build_grid(snapshot: &EditorSnapshot, cols: u16, rows: u16) -> CellGrid {
    let mut grid = CellGrid::new(cols, rows);

    // For simplicity, render first buffer's content.
    if let Some((_, buffer)) = snapshot.buffers.iter().next() {
        let text_rows = (rows as usize).saturating_sub(2); // Reserve for status + cmdline.

        for (y, line) in buffer.lines.iter().enumerate().take(text_rows) {
            render_line_to_grid(&mut grid, line, y as u16, cols);
        }
    }

    grid
}

/// Render a line to the grid.
pub fn render_line_to_grid(grid: &mut CellGrid, line: &str, y: u16, cols: u16) {
    let mut x = 0u16;

    for grapheme in unicode_segmentation::UnicodeSegmentation::graphemes(line, true) {
        if x >= cols {
            break;
        }

        let width = UnicodeWidthStr::width(grapheme);

        if width == 0 {
            continue;
        }

        // Check if we need to wrap wide character.
        if width == 2 && x + 1 >= cols {
            // Padding cell.
            grid.set(x, y, Cell::padding(Style::default()));
            break;
        }

        let cell = Cell::new(grapheme.to_string(), width as u8, Style::default());
        grid.set(x, y, cell);

        if width == 2 {
            x += 1;
            if x < cols {
                grid.set(x, y, Cell::continuation(Style::default()));
            }
        }

        x += 1;
    }
}
