//! Bracket text object helpers: find matching open/close.

use kjxlkj_core_text::BufferContent;

use crate::cursor::CursorPosition;

pub(crate) fn find_open_bracket(
    cursor: &CursorPosition,
    content: &BufferContent,
    open: char,
    close: char,
) -> Option<(usize, usize)> {
    let mut depth = 0i32;
    for line in (0..=cursor.line).rev() {
        let lc = content.line_content(line);
        let lg =
            kjxlkj_core_text::LineGraphemes::from_str(
                &lc,
            );
        let start_idx = if line == cursor.line {
            cursor.grapheme_offset
        } else {
            lg.count().saturating_sub(1)
        };
        for i in (0..=start_idx).rev() {
            if let Some(g) = lg.get(i) {
                let c =
                    g.chars().next().unwrap_or(' ');
                if c == close {
                    depth += 1;
                } else if c == open {
                    if depth == 0 {
                        return Some((line, i));
                    }
                    depth -= 1;
                }
            }
        }
    }
    None
}

pub(crate) fn find_close_bracket(
    start_line: usize,
    start_col: usize,
    content: &BufferContent,
    open: char,
    close: char,
) -> Option<(usize, usize)> {
    let mut depth = 0i32;
    for line in start_line..content.line_count() {
        let lc = content.line_content(line);
        let lg =
            kjxlkj_core_text::LineGraphemes::from_str(
                &lc,
            );
        let si = if line == start_line {
            start_col
        } else {
            0
        };
        for i in si..lg.count() {
            if let Some(g) = lg.get(i) {
                let c =
                    g.chars().next().unwrap_or(' ');
                if c == open
                    && !(line == start_line
                        && i == start_col)
                {
                    depth += 1;
                } else if c == close {
                    if depth == 0 {
                        return Some((line, i));
                    }
                    depth -= 1;
                }
            }
        }
    }
    None
}
