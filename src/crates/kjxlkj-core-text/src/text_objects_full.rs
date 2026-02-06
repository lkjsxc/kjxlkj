/// Full text objects — argument, indent-level, entire buffer, and more.

/// Text object kind for extended objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtTextObject { Argument, IndentLevel, EntireBuffer, Line, Number, Url }

/// A text range result from a text object search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange { pub start_line: usize, pub start_col: usize, pub end_line: usize, pub end_col: usize }

impl TextRange {
    pub fn new(sl: usize, sc: usize, el: usize, ec: usize) -> Self {
        Self { start_line: sl, start_col: sc, end_line: el, end_col: ec }
    }
    pub fn is_empty(&self) -> bool {
        self.start_line == self.end_line && self.start_col == self.end_col
    }
    pub fn contains(&self, line: usize, col: usize) -> bool {
        if line < self.start_line || line > self.end_line { return false; }
        if line == self.start_line && col < self.start_col { return false; }
        if line == self.end_line && col > self.end_col { return false; }
        true
    }
}

/// Find argument text object (comma-delimited within parens/brackets).
pub fn find_argument(lines: &[&str], line: usize, col: usize, inner: bool) -> Option<TextRange> {
    let text = lines.get(line)?;
    let bytes = text.as_bytes();
    // Find enclosing delimiter
    let (mut start, mut end) = (col, col);
    // Scan left for comma or opening delimiter
    let mut depth = 0i32;
    let mut i = col;
    loop {
        let ch = *bytes.get(i)? as char;
        if ch == ')' || ch == ']' { depth += 1; }
        if ch == '(' || ch == '[' { if depth == 0 { start = i + 1; break; } depth -= 1; }
        if ch == ',' && depth == 0 { start = if inner { i + 1 } else { i }; break; }
        if i == 0 { return None; }
        i -= 1;
    }
    // Scan right for comma or closing delimiter
    depth = 0;
    for j in col..bytes.len() {
        let ch = bytes[j] as char;
        if ch == '(' || ch == '[' { depth += 1; }
        if ch == ')' || ch == ']' { if depth == 0 { end = j; break; } depth -= 1; }
        if ch == ',' && depth == 0 { end = if inner { j } else { j + 1 }; break; }
    }
    // Trim whitespace for inner
    if inner {
        while start < end && bytes.get(start).map_or(false, |b| *b == b' ') { start += 1; }
        while end > start && bytes.get(end - 1).map_or(false, |b| *b == b' ') { end -= 1; }
    }
    Some(TextRange::new(line, start, line, end))
}

/// Find indent-level text object — all contiguous lines with same or deeper indent.
pub fn find_indent_level(lines: &[&str], line: usize, inner: bool) -> Option<TextRange> {
    if line >= lines.len() { return None; }
    let base_indent = indent_of(lines[line]);
    if base_indent == 0 && !inner { return None; }
    let mut start = line;
    while start > 0 && indent_of(lines[start - 1]) >= base_indent && !lines[start - 1].trim().is_empty() {
        start -= 1;
    }
    let mut end = line;
    while end + 1 < lines.len() && indent_of(lines[end + 1]) >= base_indent && !lines[end + 1].trim().is_empty() {
        end += 1;
    }
    if !inner { // Include blank lines around
        if start > 0 && lines[start - 1].trim().is_empty() { start -= 1; }
        if end + 1 < lines.len() && lines[end + 1].trim().is_empty() { end += 1; }
    }
    let last_col = lines[end].len().saturating_sub(1);
    Some(TextRange::new(start, 0, end, last_col))
}

fn indent_of(s: &str) -> usize {
    s.len() - s.trim_start().len()
}

/// Find entire buffer text object.
pub fn find_entire_buffer(lines: &[&str], inner: bool) -> Option<TextRange> {
    if lines.is_empty() { return None; }
    let (mut start, mut end) = (0, lines.len() - 1);
    if inner {
        while start < end && lines[start].trim().is_empty() { start += 1; }
        while end > start && lines[end].trim().is_empty() { end -= 1; }
    }
    let last_col = lines[end].len().saturating_sub(1);
    Some(TextRange::new(start, 0, end, last_col))
}

/// Find number under cursor.
pub fn find_number(text: &str, col: usize) -> Option<TextRange> {
    let bytes = text.as_bytes();
    let mut start = col;
    while start > 0 && (bytes[start - 1] as char).is_ascii_digit() { start -= 1; }
    if !bytes.get(start).map_or(false, |b| (*b as char).is_ascii_digit()) { return None; }
    let mut end = start;
    while end < bytes.len() && (bytes[end] as char).is_ascii_digit() { end += 1; }
    Some(TextRange::new(0, start, 0, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argument_inner() {
        let lines = vec!["foo(a, bar, c)"];
        let r = find_argument(&lines, 0, 7, true).unwrap();
        assert_eq!((r.start_col, r.end_col), (7, 10));
    }

    #[test]
    fn argument_outer() {
        let lines = vec!["foo(a, bar, c)"];
        let r = find_argument(&lines, 0, 7, false).unwrap();
        assert!(r.start_col <= 7 && r.end_col >= 10);
    }

    #[test]
    fn indent_level_inner() {
        let lines = vec!["def foo:", "    a = 1", "    b = 2", "end"];
        let r = find_indent_level(&lines, 1, true).unwrap();
        assert_eq!(r.start_line, 1);
        assert_eq!(r.end_line, 2);
    }

    #[test]
    fn entire_buffer_inner() {
        let lines = vec!["", "hello", "world", ""];
        let r = find_entire_buffer(&lines, true).unwrap();
        assert_eq!(r.start_line, 1);
        assert_eq!(r.end_line, 2);
    }

    #[test]
    fn entire_buffer_outer() {
        let lines = vec!["a", "b"];
        let r = find_entire_buffer(&lines, false).unwrap();
        assert_eq!((r.start_line, r.end_line), (0, 1));
    }

    #[test]
    fn find_number_basic() {
        let r = find_number("abc 42 def", 5).unwrap();
        assert_eq!((r.start_col, r.end_col), (4, 6));
    }

    #[test]
    fn find_number_none() {
        assert!(find_number("abc", 1).is_none());
    }

    #[test]
    fn text_range_contains() {
        let r = TextRange::new(1, 5, 3, 10);
        assert!(r.contains(2, 0));
        assert!(!r.contains(0, 0));
        assert!(!r.contains(1, 3));
    }

    #[test]
    fn text_range_empty() {
        let r = TextRange::new(1, 5, 1, 5);
        assert!(r.is_empty());
    }
}
