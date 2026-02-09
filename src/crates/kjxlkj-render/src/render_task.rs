use crate::cell::CellGrid;
use crate::grid::build_grid;
use kjxlkj_core_ui::{Color, EditorSnapshot, Style};
use std::io::{self, Write};
use tokio::sync::watch;

/// Render task: consumes snapshots and renders to terminal.
pub struct RenderTask;

impl RenderTask {
    /// Run the render loop.
    pub async fn run(
        mut snapshot_rx: watch::Receiver<EditorSnapshot>,
        mut quit_rx: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let mut prev_grid: Option<CellGrid> = None;

        loop {
            tokio::select! {
                _ = quit_rx.recv() => break,
                result = snapshot_rx.changed() => {
                    if result.is_err() {
                        break;
                    }
                    let snapshot = snapshot_rx.borrow().clone();
                    let grid = build_grid(&snapshot);
                    flush_frame(&grid, prev_grid.as_ref())?;
                    prev_grid = Some(grid);
                }
            }
        }
        Ok(())
    }
}

/// Flush a frame to stdout using diff rendering.
fn flush_frame(grid: &CellGrid, prev: Option<&CellGrid>) -> anyhow::Result<()> {
    let mut buf = Vec::with_capacity(4096);
    let full_redraw = prev.is_none()
        || prev
            .map(|p| p.width() != grid.width() || p.height() != grid.height())
            .unwrap_or(true);

    if full_redraw {
        // Full redraw
        for row in 0..grid.height() {
            // Move cursor to start of row
            write!(buf, "\x1b[{};{}H", row + 1, 1)?;
            for col in 0..grid.width() {
                let cell = grid.get(col, row);
                if cell.is_wide_continuation {
                    continue;
                }
                write_cell_style(&mut buf, &cell.style)?;
                write!(buf, "{}", cell.grapheme)?;
            }
            // Reset style at end of row
            write!(buf, "\x1b[0m")?;
        }
    } else if let Some(prev) = prev {
        // Diff-based rendering
        let dirty = grid.diff(prev);
        for (col, row) in dirty {
            let cell = grid.get(col, row);
            if cell.is_wide_continuation {
                continue;
            }
            write!(buf, "\x1b[{};{}H", row + 1, col + 1)?;
            write_cell_style(&mut buf, &cell.style)?;
            write!(buf, "{}", cell.grapheme)?;
        }
        write!(buf, "\x1b[0m")?;
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(&buf)?;
    stdout.flush()?;
    Ok(())
}

fn write_cell_style(buf: &mut Vec<u8>, style: &Style) -> io::Result<()> {
    // Reset
    write!(buf, "\x1b[0m")?;

    // Foreground
    match style.fg {
        Color::Rgb(r, g, b) => {
            write!(buf, "\x1b[38;2;{r};{g};{b}m")?;
        }
        Color::Indexed(n) => {
            write!(buf, "\x1b[38;5;{n}m")?;
        }
        Color::Default => {}
    }

    // Background
    match style.bg {
        Color::Rgb(r, g, b) => {
            write!(buf, "\x1b[48;2;{r};{g};{b}m")?;
        }
        Color::Indexed(n) => {
            write!(buf, "\x1b[48;5;{n}m")?;
        }
        Color::Default => {}
    }

    // Attributes
    if style.bold {
        write!(buf, "\x1b[1m")?;
    }
    if style.italic {
        write!(buf, "\x1b[3m")?;
    }
    if style.underline {
        write!(buf, "\x1b[4m")?;
    }
    if style.reverse {
        write!(buf, "\x1b[7m")?;
    }

    Ok(())
}
