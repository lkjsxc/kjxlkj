//! Selection rendering.

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};
use kjxlkj_core_types::{Position, Selection, SelectionKind};
use std::io::{self, Write};

/// Render a line with selection highlighting.
pub fn render_with_selection<W: Write>(
    writer: &mut W,
    line: &str,
    line_idx: usize,
    selection: &Selection,
) -> io::Result<()> {
    let start = selection.start();
    let end = selection.end();

    match selection.kind {
        SelectionKind::Char => render_char_selection(writer, line, line_idx, start, end),
        SelectionKind::Line => render_line_selection(writer, line, line_idx, start, end),
        SelectionKind::Block => render_block_selection(writer, line, line_idx, start, end),
    }
}

fn render_line_selection<W: Write>(
    writer: &mut W,
    line: &str,
    line_idx: usize,
    start: Position,
    end: Position,
) -> io::Result<()> {
    if line_idx >= start.line && line_idx <= end.line {
        execute!(writer, SetBackgroundColor(Color::DarkBlue), Print(line), ResetColor)
    } else {
        execute!(writer, Print(line))
    }
}

fn render_char_selection<W: Write>(
    writer: &mut W,
    line: &str,
    line_idx: usize,
    start: Position,
    end: Position,
) -> io::Result<()> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if line_idx < start.line || line_idx > end.line {
        return execute!(writer, Print(line));
    }

    let sel_start = if line_idx == start.line { start.col } else { 0 };
    let sel_end = if line_idx == end.line { end.col + 1 } else { len };

    // Before selection
    let before: String = chars[..sel_start.min(len)].iter().collect();
    execute!(writer, Print(&before))?;

    // Selection
    let selected: String = chars[sel_start.min(len)..sel_end.min(len)].iter().collect();
    execute!(writer, SetBackgroundColor(Color::DarkBlue), Print(&selected), ResetColor)?;

    // After selection
    let after: String = chars[sel_end.min(len)..].iter().collect();
    execute!(writer, Print(&after))
}

fn render_block_selection<W: Write>(
    writer: &mut W,
    line: &str,
    line_idx: usize,
    start: Position,
    end: Position,
) -> io::Result<()> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if line_idx < start.line || line_idx > end.line {
        return execute!(writer, Print(line));
    }

    let col_start = start.col.min(end.col);
    let col_end = start.col.max(end.col) + 1;

    let before: String = chars[..col_start.min(len)].iter().collect();
    execute!(writer, Print(&before))?;

    let selected: String = chars[col_start.min(len)..col_end.min(len)].iter().collect();
    execute!(writer, SetBackgroundColor(Color::DarkBlue), Print(&selected), ResetColor)?;

    let after: String = chars[col_end.min(len)..].iter().collect();
    execute!(writer, Print(&after))
}
