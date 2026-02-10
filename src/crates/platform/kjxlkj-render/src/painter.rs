//! Terminal painting.

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color as CtColor, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_ui::{EditorSnapshot, Color, NamedColor, CellGrid, Cell, Style};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Paint a snapshot to the terminal.
pub fn paint(snapshot: &EditorSnapshot) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (cols, rows) = snapshot.terminal_size;

    // Build a cell grid.
    let grid = build_grid(snapshot, cols, rows);

    // Clear and render.
    execute!(stdout, MoveTo(0, 0))?;

    for y in 0..rows {
        execute!(stdout, MoveTo(0, y))?;

        for x in 0..cols {
            if let Some(cell) = grid.get(x, y) {
                if !cell.is_wide_continuation {
                    set_style(&mut stdout, &cell.style)?;
                    if cell.grapheme.is_empty() {
                        execute!(stdout, Print(" "))?;
                    } else {
                        execute!(stdout, Print(&cell.grapheme))?;
                    }
                }
            }
        }
    }

    // Render statusline.
    let status_y = rows.saturating_sub(2);
    execute!(stdout, MoveTo(0, status_y))?;
    execute!(stdout, ResetColor)?;

    let mode_str = snapshot.mode.name();
    let status = format!(" {} ", mode_str);
    execute!(stdout, SetBackgroundColor(CtColor::Blue))?;
    execute!(stdout, SetForegroundColor(CtColor::White))?;
    execute!(stdout, Print(&status))?;
    execute!(stdout, ResetColor)?;

    // Fill rest of statusline.
    let remaining = (cols as usize).saturating_sub(status.len());
    execute!(stdout, Print(" ".repeat(remaining)))?;

    // Render cmdline if in command mode.
    let cmd_y = rows.saturating_sub(1);
    execute!(stdout, MoveTo(0, cmd_y))?;
    execute!(stdout, Clear(ClearType::CurrentLine))?;

    if let Some(prefix) = snapshot.cmdline.prefix {
        execute!(stdout, Print(prefix))?;
        execute!(stdout, Print(&snapshot.cmdline.content))?;
    }

    // Position cursor.
    if let Some(tab) = snapshot.tabs.get(snapshot.active_tab) {
        if let Some(_window_id) = tab.focused_window {
            // Find window and cursor position.
            // Simplified: just put cursor at top-left for now.
            execute!(stdout, MoveTo(0, 0))?;
        }
    }

    // Show cursor.
    execute!(stdout, crossterm::cursor::Show)?;

    stdout.flush()?;
    Ok(())
}

/// Build a cell grid from snapshot.
fn build_grid(snapshot: &EditorSnapshot, cols: u16, rows: u16) -> CellGrid {
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
fn render_line_to_grid(grid: &mut CellGrid, line: &str, y: u16, cols: u16) {
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

/// Set terminal style.
fn set_style(stdout: &mut io::Stdout, style: &Style) -> io::Result<()> {
    let fg = convert_color(&style.fg);
    let bg = convert_color(&style.bg);
    execute!(stdout, SetForegroundColor(fg), SetBackgroundColor(bg))?;
    Ok(())
}

/// Convert our color to crossterm color.
fn convert_color(color: &Color) -> CtColor {
    match color {
        Color::Default => CtColor::Reset,
        Color::Named(named) => match named {
            NamedColor::Black => CtColor::Black,
            NamedColor::Red => CtColor::DarkRed,
            NamedColor::Green => CtColor::DarkGreen,
            NamedColor::Yellow => CtColor::DarkYellow,
            NamedColor::Blue => CtColor::DarkBlue,
            NamedColor::Magenta => CtColor::DarkMagenta,
            NamedColor::Cyan => CtColor::DarkCyan,
            NamedColor::White => CtColor::Grey,
            NamedColor::BrightBlack => CtColor::DarkGrey,
            NamedColor::BrightRed => CtColor::Red,
            NamedColor::BrightGreen => CtColor::Green,
            NamedColor::BrightYellow => CtColor::Yellow,
            NamedColor::BrightBlue => CtColor::Blue,
            NamedColor::BrightMagenta => CtColor::Magenta,
            NamedColor::BrightCyan => CtColor::Cyan,
            NamedColor::BrightWhite => CtColor::White,
        },
        Color::Indexed(i) => CtColor::AnsiValue(*i),
        Color::Rgb(r, g, b) => CtColor::Rgb { r: *r, g: *g, b: *b },
    }
}
