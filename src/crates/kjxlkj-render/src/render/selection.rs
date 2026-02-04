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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_selection_within_range() {
        let sel = Selection::new(
            Position::new(1, 0),
            Position::new(3, 5),
            SelectionKind::Line,
        );
        // Line 2 is within 1..=3
        let mut buf = Vec::new();
        let res = render_line_selection(&mut buf, "hello", 2, sel.start(), sel.end());
        assert!(res.is_ok());
    }

    #[test]
    fn char_selection_before_range() {
        let sel = Selection::new(
            Position::new(5, 0),
            Position::new(5, 5),
            SelectionKind::Char,
        );
        let mut buf = Vec::new();
        let res = render_char_selection(&mut buf, "hello", 0, sel.start(), sel.end());
        assert!(res.is_ok());
    }

    #[test]
    fn block_selection_renders() {
        let sel = Selection::new(
            Position::new(0, 2),
            Position::new(2, 5),
            SelectionKind::Block,
        );
        let mut buf = Vec::new();
        let res = render_block_selection(&mut buf, "hello world", 1, sel.start(), sel.end());
        assert!(res.is_ok());
    }

    #[test]
    fn selection_kind_routing() {
        let sel = Selection::new(
            Position::new(0, 0),
            Position::new(0, 5),
            SelectionKind::Char,
        );
        let mut buf = Vec::new();
        let res = render_with_selection(&mut buf, "hello", 0, &sel);
        assert!(res.is_ok());
    }

    #[test]
    fn line_selection_route() {
        let sel = Selection::new(
            Position::new(0, 0),
            Position::new(2, 0),
            SelectionKind::Line,
        );
        let mut buf = Vec::new();
        let res = render_with_selection(&mut buf, "test line", 1, &sel);
        assert!(res.is_ok());
    }

    #[test]
    fn block_selection_route() {
        let sel = Selection::new(
            Position::new(0, 1),
            Position::new(2, 4),
            SelectionKind::Block,
        );
        let mut buf = Vec::new();
        let res = render_with_selection(&mut buf, "block text", 1, &sel);
        assert!(res.is_ok());
    }

    #[test]
    fn char_selection_empty_line() {
        let sel = Selection::new(
            Position::new(0, 0),
            Position::new(0, 5),
            SelectionKind::Char,
        );
        let mut buf = Vec::new();
        let res = render_char_selection(&mut buf, "", 0, sel.start(), sel.end());
        assert!(res.is_ok());
    }
}
