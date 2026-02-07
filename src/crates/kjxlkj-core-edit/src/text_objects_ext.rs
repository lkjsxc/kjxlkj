//! Extended text objects: argument, indent level, entire buffer, number.

use kjxlkj_core_types::{Position, Range};
use serde::{Deserialize, Serialize};

/// A range with helper methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRange {
    pub start: Position,
    pub end: Position,
}

impl TextRange {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Find an argument text object at the cursor position (comma-delimited).
pub fn find_argument(lines: &[&str], pos: Position, inner: bool) -> Option<Range> {
    if pos.line >= lines.len() {
        return None;
    }
    let ch: Vec<char> = lines[pos.line].chars().collect();
    if pos.col >= ch.len() {
        return None;
    }
    let mut depth = 0i32;
    let mut start = pos.col;
    loop {
        let c = ch[start];
        if matches!(c, ')' | ']' | '}') {
            depth += 1;
        }
        if matches!(c, '(' | '[' | '{') {
            if depth > 0 {
                depth -= 1;
            } else {
                start += 1;
                break;
            }
        }
        if c == ',' && depth == 0 {
            start += 1;
            break;
        }
        if start == 0 {
            break;
        }
        start -= 1;
    }
    depth = 0;
    let mut end = pos.col;
    while end < ch.len() {
        let c = ch[end];
        if matches!(c, '(' | '[' | '{') {
            depth += 1;
        }
        if matches!(c, ')' | ']' | '}') {
            if depth > 0 {
                depth -= 1;
            } else {
                break;
            }
        }
        if c == ',' && depth == 0 {
            if !inner {
                end += 1;
            }
            break;
        }
        end += 1;
    }
    if inner {
        while start < end && ch.get(start).is_some_and(|c| c.is_whitespace()) {
            start += 1;
        }
        while end > start && ch.get(end - 1).is_some_and(|c| c.is_whitespace()) {
            end -= 1;
        }
    }
    if start >= end {
        return None;
    }
    Some(Range::new(
        Position::new(pos.line, start),
        Position::new(pos.line, end),
    ))
}

/// Find lines at the same or deeper indent level.
pub fn find_indent_level(lines: &[&str], pos: Position, inner: bool) -> Option<Range> {
    if pos.line >= lines.len() {
        return None;
    }
    let base = leading_ws(lines[pos.line]);
    let mut s = pos.line;
    while s > 0 && (lines[s - 1].trim().is_empty() || leading_ws(lines[s - 1]) >= base) {
        s -= 1;
    }
    let mut e = pos.line;
    while e + 1 < lines.len()
        && (lines[e + 1].trim().is_empty() || leading_ws(lines[e + 1]) >= base)
    {
        e += 1;
    }
    if inner {
        while s < e && lines[s].trim().is_empty() {
            s += 1;
        }
        while e > s && lines[e].trim().is_empty() {
            e -= 1;
        }
    }
    Some(Range::new(
        Position::new(s, 0),
        Position::new(e, lines[e].len()),
    ))
}

/// Select the entire buffer.
pub fn find_entire_buffer(line_count: usize, _inner: bool) -> Range {
    if line_count == 0 {
        return Range::new(Position::ZERO, Position::ZERO);
    }
    Range::new(Position::new(0, 0), Position::new(line_count - 1, 0))
}

/// Find a number at the given column position within the line.
pub fn find_number(line: &str, col: usize) -> Option<Range> {
    let ch: Vec<char> = line.chars().collect();
    if col >= ch.len() {
        return None;
    }
    if !ch[col].is_ascii_digit() && !matches!(ch[col], '-' | 'x' | 'X') {
        return None;
    }
    let mut s = col;
    while s > 0 && is_num(ch[s - 1]) {
        s -= 1;
    }
    if s > 0 && ch[s - 1] == '-' {
        s -= 1;
    }
    let mut e = col;
    while e + 1 < ch.len() && is_num(ch[e + 1]) {
        e += 1;
    }
    Some(Range::new(Position::new(0, s), Position::new(0, e + 1)))
}

fn is_num(c: char) -> bool {
    c.is_ascii_hexdigit() || matches!(c, 'x' | 'X' | 'o' | 'b')
}
fn leading_ws(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_range_contains() {
        let tr = TextRange::new(Position::new(0, 0), Position::new(0, 5));
        assert!(tr.contains(Position::new(0, 3)));
        assert!(!tr.contains(Position::new(0, 5)));
    }
    #[test]
    fn find_number_basic() {
        assert!(find_number("abc 123 def", 5).is_some());
    }
    #[test]
    fn find_argument_basic() {
        assert!(find_argument(&["fn(a, b, c)"], Position::new(0, 6), true).is_some());
    }
    #[test]
    fn find_entire() {
        let r = find_entire_buffer(10, false);
        assert_eq!(r.start, Position::ZERO);
        assert_eq!(r.end.line, 9);
    }
}
