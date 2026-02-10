//! Grid diff and flush to terminal.

use crossterm::{
    cursor::MoveTo,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

use crate::grid::CellGrid;

/// Diff current grid against previous and flush changes.
pub fn flush_grid(grid: &CellGrid, prev: &Option<CellGrid>) -> io::Result<()> {
    let mut stdout = io::stdout();

    let full_redraw = match prev {
        None => true,
        Some(p) => p.rows != grid.rows || p.cols != grid.cols,
    };

    if full_redraw {
        stdout.queue(Clear(ClearType::All))?;
    }

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let cell = &grid.cells[row][col];
            if cell.is_wide_continuation {
                continue;
            }
            let changed = match prev {
                None => true,
                Some(p) => {
                    if row < p.rows && col < p.cols {
                        &p.cells[row][col] != cell
                    } else {
                        true
                    }
                }
            };
            if !changed {
                continue;
            }
            stdout.queue(MoveTo(col as u16, row as u16))?;
            stdout.queue(SetForegroundColor(Color::Rgb {
                r: cell.fg.0,
                g: cell.fg.1,
                b: cell.fg.2,
            }))?;
            stdout.queue(SetBackgroundColor(Color::Rgb {
                r: cell.bg.0,
                g: cell.bg.1,
                b: cell.bg.2,
            }))?;
            if cell.bold {
                stdout.queue(SetAttribute(Attribute::Bold))?;
            }
            stdout.queue(Print(&cell.grapheme))?;
            if cell.bold {
                stdout.queue(SetAttribute(Attribute::NoBold))?;
            }
        }
    }

    stdout.queue(ResetColor)?;
    stdout.flush()?;
    Ok(())
}
