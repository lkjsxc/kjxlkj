//! Painter: converts EditorSnapshot into terminal output.
//!
//! Thin facade. Implementation split across:
//! - paint_window.rs: buffer/terminal window rendering
//! - paint_chrome.rs: statusline and command-line
//! - paint_flush.rs: grid diff and terminal flush

use kjxlkj_core_ui::EditorSnapshot;
use std::io;

use crate::grid::CellGrid;
use crate::paint::paint_chrome;
use crate::paint::paint_flush;
use crate::paint::paint_window;

/// Paint a snapshot to the terminal.
pub fn paint(snapshot: &EditorSnapshot, prev_grid: &mut Option<CellGrid>) -> io::Result<()> {
    let (cols, rows) = snapshot.terminal_size;
    let cols = cols as usize;
    let rows = rows as usize;
    if rows == 0 || cols == 0 {
        return Ok(());
    }

    let mut grid = CellGrid::new(rows, cols);
    let tab = &snapshot.tabs[snapshot.active_tab];

    // Render each window
    for (i, win) in tab.windows.iter().enumerate() {
        let is_active = i == tab.active_window;
        paint_window::render_window(win, snapshot, &mut grid, cols, rows, is_active);
    }

    // Render statusline at bottom - 1
    if rows >= 2 {
        paint_chrome::render_statusline(snapshot, &mut grid, rows - 2, cols);
    }

    // Render command line at bottom
    if rows >= 1 {
        paint_chrome::render_cmdline(snapshot, &mut grid, rows - 1, cols);
    }

    // Diff and flush
    paint_flush::flush_grid(&grid, prev_grid)?;
    *prev_grid = Some(grid);

    Ok(())
}
