//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::TextBuffer;
use kjxlkj_core_types::Position;

/// Display width of a grapheme cluster string.
pub fn grapheme_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Move to the next grapheme boundary from a position.
pub fn next_grapheme_boundary(buf: &TextBuffer, pos: Position) -> Position {
    let line_len = buf.line_len(pos.line);
    if pos.col < line_len {
        let line_str = buf.line_to_string(pos.line);
        let graphemes: Vec<&str> = line_str.graphemes(true).collect();
        let mut col = 0;
        for g in &graphemes {
            let gc = g.chars().count();
            if col >= pos.col {
                return Position::new(pos.line, col + gc);
            }
            col += gc;
        }
        Position::new(pos.line, line_len)
    } else if pos.line + 1 < buf.line_count() {
        Position::new(pos.line + 1, 0)
    } else {
        pos
    }
}

/// Move to the previous grapheme boundary from a position.
pub fn prev_grapheme_boundary(buf: &TextBuffer, pos: Position) -> Position {
    if pos.col > 0 {
        let line_str = buf.line_to_string(pos.line);
        let graphemes: Vec<&str> = line_str.graphemes(true).collect();
        let mut col = 0;
        let mut prev_col = 0;
        for g in &graphemes {
            let gc = g.chars().count();
            if col + gc >= pos.col && col < pos.col {
                return Position::new(pos.line, col);
            }
            prev_col = col;
            col += gc;
            if col >= pos.col {
                return Position::new(pos.line, prev_col);
            }
        }
        Position::new(pos.line, 0)
    } else if pos.line > 0 {
        let prev_len = buf.line_len(pos.line - 1);
        Position::new(pos.line - 1, prev_len.saturating_sub(1))
    } else {
        pos
    }
}

/// Compute the display width of a line up to a column.
pub fn display_width_to_col(line: &str, col: usize) -> usize {
    let mut w = 0;
    let mut c = 0;
    for g in line.graphemes(true) {
        if c >= col {
            break;
        }
        w += grapheme_width(g);
        c += g.chars().count();
    }
    w
}

/// Compute the total display width of a line.
pub fn line_display_width(line: &str) -> usize {
    line.graphemes(true).map(|g| grapheme_width(g)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_grapheme_width() {
        assert_eq!(grapheme_width("a"), 1);
        assert_eq!(grapheme_width(" "), 1);
    }

    #[test]
    fn next_boundary_simple() {
        let buf = TextBuffer::from_text("hello");
        let p = next_grapheme_boundary(&buf, Position::new(0, 0));
        assert_eq!(p, Position::new(0, 1));
    }

    #[test]
    fn prev_boundary_simple() {
        let buf = TextBuffer::from_text("hello");
        let p = prev_grapheme_boundary(&buf, Position::new(0, 3));
        assert_eq!(p, Position::new(0, 2));
    }

    #[test]
    fn next_boundary_end_of_line() {
        let buf = TextBuffer::from_text("ab\ncd");
        let p = next_grapheme_boundary(&buf, Position::new(0, 2));
        assert_eq!(p, Position::new(1, 0));
    }

    #[test]
    fn prev_boundary_start_of_line() {
        let buf = TextBuffer::from_text("ab\ncd");
        let p = prev_grapheme_boundary(&buf, Position::new(1, 0));
        assert_eq!(p, Position::new(0, 1));
    }

    #[test]
    fn display_width() {
        assert_eq!(line_display_width("hello"), 5);
    }
}
