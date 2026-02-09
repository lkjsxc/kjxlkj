//! Argument text objects: `ia` (inner argument),
//! `aa` (around argument).
//!
//! Operates on comma-separated items within brackets.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::TextObjectScope;

use crate::cursor::CursorPosition;
use crate::text_object_bracket::{find_close_bracket, find_open_bracket};
use crate::text_object_exec::TextObjectRange;

/// Resolve an argument text object at cursor.
pub(crate) fn resolve_argument(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<TextObjectRange> {
    // Find enclosing parentheses, brackets, or angles
    let (open_line, open_col) = find_any_open_bracket(cursor, content)?;
    let (close_line, close_col) = find_any_close_bracket(open_line, open_col, cursor, content)?;

    // Collect comma positions at this nesting level
    let commas = find_commas_at_level(open_line, open_col, close_line, close_col, content);

    // Find which argument the cursor is in
    let mut arg_start_line = open_line;
    let mut arg_start_col = open_col + 1;
    let mut arg_end_line = close_line;
    let mut arg_end_col = close_col.saturating_sub(1);
    let mut comma_before: Option<(usize, usize)> = None;
    let mut comma_after: Option<(usize, usize)> = None;

    for &(cl, cc) in &commas {
        if (cl, cc) < (cursor.line, cursor.grapheme_offset) {
            arg_start_line = cl;
            arg_start_col = cc + 1;
            comma_before = Some((cl, cc));
        } else {
            arg_end_line = cl;
            arg_end_col = cc.saturating_sub(1);
            comma_after = Some((cl, cc));
            break;
        }
    }

    // Skip leading whitespace for inner
    let (trimmed_start_line, trimmed_start_col) =
        skip_whitespace_forward(arg_start_line, arg_start_col, content);

    match scope {
        TextObjectScope::Inner => Some(TextObjectRange {
            start: CursorPosition::new(trimmed_start_line, trimmed_start_col),
            end: CursorPosition::new(arg_end_line, arg_end_col),
            linewise: false,
        }),
        TextObjectScope::Around => {
            // Include comma: prefer trailing, else leading
            let (s_line, s_col, e_line, e_col) = if let Some((cl, cc)) = comma_after {
                (trimmed_start_line, trimmed_start_col, cl, cc)
            } else if let Some((cl, cc)) = comma_before {
                (cl, cc, arg_end_line, arg_end_col)
            } else {
                (
                    trimmed_start_line,
                    trimmed_start_col,
                    arg_end_line,
                    arg_end_col,
                )
            };
            Some(TextObjectRange {
                start: CursorPosition::new(s_line, s_col),
                end: CursorPosition::new(e_line, e_col),
                linewise: false,
            })
        }
    }
}

/// Try to find the enclosing open bracket (paren, bracket, angle).
fn find_any_open_bracket(
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<(usize, usize)> {
    for &(o, c) in &[('(', ')'), ('[', ']'), ('<', '>')] {
        if let Some(pos) = find_open_bracket(cursor, content, o, c) {
            return Some(pos);
        }
    }
    None
}

/// Find matching close bracket.
fn find_any_close_bracket(
    open_line: usize,
    open_col: usize,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<(usize, usize)> {
    let line_content = content.line_content(open_line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line_content);
    let open_char = lg.get(open_col)?.chars().next()?;
    let close_char = match open_char {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => return None,
    };
    let _ = cursor; // Used to find context
    find_close_bracket(open_line, open_col, content, open_char, close_char)
}

/// Find commas at the same nesting depth.
fn find_commas_at_level(
    open_line: usize,
    open_col: usize,
    close_line: usize,
    close_col: usize,
    content: &BufferContent,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut depth = 0i32;
    for line in open_line..=close_line {
        let lc = content.line_content(line);
        let lg = kjxlkj_core_text::LineGraphemes::from_str(&lc);
        let si = if line == open_line { open_col + 1 } else { 0 };
        let ei = if line == close_line {
            close_col
        } else {
            lg.count()
        };
        for i in si..ei {
            if let Some(g) = lg.get(i) {
                let c = g.chars().next().unwrap_or(' ');
                match c {
                    '(' | '[' | '{' | '<' => depth += 1,
                    ')' | ']' | '}' | '>' => depth -= 1,
                    ',' if depth == 0 => result.push((line, i)),
                    _ => {}
                }
            }
        }
    }
    result
}

/// Skip whitespace forward from a position.
fn skip_whitespace_forward(line: usize, col: usize, content: &BufferContent) -> (usize, usize) {
    let lc = content.line_content(line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&lc);
    let mut c = col;
    while c < lg.count() {
        if let Some(g) = lg.get(c) {
            if !g.chars().next().map_or(false, |ch| ch == ' ') {
                break;
            }
        }
        c += 1;
    }
    (line, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_argument() {
        let c = BufferContent::from_str("func(a, b, c)\n");
        let cursor = CursorPosition::new(0, 9);
        let r = resolve_argument(TextObjectScope::Inner, &cursor, &c);
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start.grapheme_offset, 8);
        assert_eq!(r.end.grapheme_offset, 8);
    }

    #[test]
    fn around_argument() {
        let c = BufferContent::from_str("func(a, b, c)\n");
        let cursor = CursorPosition::new(0, 9);
        let r = resolve_argument(TextObjectScope::Around, &cursor, &c);
        assert!(r.is_some());
    }
}
