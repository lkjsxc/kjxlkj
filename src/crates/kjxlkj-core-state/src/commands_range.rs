//! Ex command range parsing: addresses, spans, marks.

use crate::EditorState;

/// A resolved line range (0-indexed, inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineRange {
    pub start: usize,
    pub end: usize,
}

impl LineRange {
    pub fn single(line: usize) -> Self { Self { start: line, end: line } }
    pub fn new(start: usize, end: usize) -> Self { Self { start, end } }
    pub fn line_count(&self) -> usize { self.end - self.start + 1 }
}

/// Parse a range prefix from a command string.
/// Returns (range, remaining_command) or (None, full_cmd).
pub(crate) fn parse_range(
    state: &EditorState, cmd: &str,
) -> (Option<LineRange>, String) {
    let trimmed = cmd.trim();
    if trimmed.is_empty() { return (None, String::new()); }
    let s = trimmed.strip_prefix(':').unwrap_or(trimmed);
    let cursor_line = current_line(state);
    let last_line = last_line_idx(state);

    if s.starts_with('%') {
        let rest = s[1..].trim();
        return (Some(LineRange::new(0, last_line)), format!(":{}", rest));
    }

    let (addr1, after1) = parse_address(s, cursor_line, last_line);
    let start = match addr1 { Some(a) => a, None => return (None, format!(":{}", s)) };

    let after1 = after1.trim_start();
    if after1.starts_with(',') || after1.starts_with(';') {
        let rest = after1[1..].trim_start();
        let (addr2, after2) = parse_address(rest, cursor_line, last_line);
        let end = addr2.unwrap_or(start);
        let rng = LineRange::new(start.min(last_line), end.min(last_line));
        return (Some(rng), format!(":{}", after2.trim()));
    }

    let rng = LineRange::single(start.min(last_line));
    (Some(rng), format!(":{}", after1.trim()))
}

/// Parse a single address. Returns (line_number, remaining_string).
pub(crate) fn parse_address(
    s: &str, cursor_line: usize, last_line: usize,
) -> (Option<usize>, &str) {
    if s.is_empty() { return (None, s); }
    match s.as_bytes()[0] {
        b'.' => {
            let (off, rest) = parse_offset(&s[1..]);
            (Some(apply_offset(cursor_line, off, last_line)), rest)
        }
        b'$' => {
            let (off, rest) = parse_offset(&s[1..]);
            (Some(apply_offset(last_line, off, last_line)), rest)
        }
        b'\'' if s.len() >= 2 => {
            let (off, rest) = parse_offset(&s[2..]);
            (Some(apply_offset(cursor_line, off, last_line)), rest)
        }
        b'<' | b'>' => (Some(cursor_line), &s[1..]),
        b'0'..=b'9' => {
            let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
            let num: usize = s[..end].parse().unwrap_or(1);
            let line = if num == 0 { 0 } else { num - 1 };
            let (off, rest) = parse_offset(&s[end..]);
            (Some(apply_offset(line, off, last_line)), rest)
        }
        _ => (None, s),
    }
}

/// Parse an optional +N or -N offset.
pub(crate) fn parse_offset(s: &str) -> (isize, &str) {
    if s.is_empty() { return (0, s); }
    let first = s.as_bytes()[0];
    if first == b'+' || first == b'-' {
        let sign: isize = if first == b'+' { 1 } else { -1 };
        let rest = &s[1..];
        let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
        if end == 0 { return (sign, rest); }
        let num: isize = rest[..end].parse().unwrap_or(1);
        (sign * num, &rest[end..])
    } else {
        (0, s)
    }
}

/// Apply offset, clamping to [0, max_line].
pub(crate) fn apply_offset(base: usize, offset: isize, max_line: usize) -> usize {
    let result = base as isize + offset;
    if result < 0 { 0 } else if result as usize > max_line { max_line } else { result as usize }
}

pub(crate) fn current_line(state: &EditorState) -> usize {
    state.active_window.and_then(|wid| state.windows.get(&wid)).map(|w| w.cursor_line).unwrap_or(0)
}

pub(crate) fn last_line_idx(state: &EditorState) -> usize {
    state.active_window.and_then(|wid| {
        let w = state.windows.get(&wid)?;
        let buf = state.buffers.get(&w.buffer_id)?;
        Some(buf.text.line_count().saturating_sub(1))
    }).unwrap_or(0)
}
