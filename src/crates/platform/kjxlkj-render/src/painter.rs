//! Terminal painting.

use crate::color::convert_color;
use crate::grid::build_grid;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use kjxlkj_core_ui::{EditorSnapshot, LayoutNode, Style, WindowContentSnapshot};
use std::io::{self, Write};

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
    execute!(
        stdout,
        SetBackgroundColor(crossterm::style::Color::Blue)
    )?;
    execute!(
        stdout,
        SetForegroundColor(crossterm::style::Color::White)
    )?;
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

    // Position cursor based on mode and window.
    let cursor_pos = get_cursor_position(snapshot);
    execute!(stdout, MoveTo(cursor_pos.0, cursor_pos.1))?;

    // Show cursor.
    execute!(stdout, crossterm::cursor::Show)?;

    stdout.flush()?;
    Ok(())
}

/// Get cursor position for terminal.
fn get_cursor_position(snapshot: &EditorSnapshot) -> (u16, u16) {
    let (cols, rows) = snapshot.terminal_size;

    // In command mode, cursor is on cmdline.
    if snapshot.mode.is_command() {
        let x = 1 + snapshot.cmdline.cursor as u16;
        let y = rows.saturating_sub(1);
        return (x.min(cols - 1), y);
    }

    // Otherwise, find focused window cursor.
    if let Some(tab) = snapshot.tabs.get(snapshot.active_tab) {
        if let Some(pos) = find_cursor_in_layout(&tab.layout) {
            return pos;
        }
    }

    (0, 0)
}

/// Find cursor position in layout tree.
fn find_cursor_in_layout(node: &LayoutNode) -> Option<(u16, u16)> {
    match node {
        LayoutNode::Leaf(window) => {
            if let WindowContentSnapshot::Buffer {
                cursor_line,
                cursor_grapheme,
                top_line,
                ..
            } = &window.content
            {
                let x = window.rect.x + *cursor_grapheme as u16;
                let y = window.rect.y + (*cursor_line - *top_line) as u16;
                Some((x, y))
            } else {
                None
            }
        }
        LayoutNode::Horizontal(children) | LayoutNode::Vertical(children) => {
            for child in children {
                if let Some(pos) = find_cursor_in_layout(child) {
                    return Some(pos);
                }
            }
            None
        }
    }
}

/// Set terminal style.
fn set_style(stdout: &mut io::Stdout, style: &Style) -> io::Result<()> {
    let fg = convert_color(&style.fg);
    let bg = convert_color(&style.bg);
    execute!(stdout, SetForegroundColor(fg), SetBackgroundColor(bg))?;
    Ok(())
}
