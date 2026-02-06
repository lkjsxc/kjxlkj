//! Ex command range parsing: addresses, spans, marks.

use crate::EditorState;

/// A resolved line range (0-indexed, inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineRange {
    pub start: usize,
    pub end: usize,
}

impl LineRange {
    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn line_count(&self) -> usize {
        self.end - self.start + 1
    }
}

/// Parse a range prefix from a command string.
/// Returns (range, remaining_command) or (None, full_cmd).
pub(crate) fn parse_range(
    state: &EditorState,
    cmd: &str,
) -> (Option<LineRange>, String) {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return (None, String::new());
    }

    // Skip leading ':'
    let s = if trimmed.starts_with(':') {
        &trimmed[1..]
    } else {
        trimmed
    };

    let cursor_line = current_line(state);
    let last_line = last_line_idx(state);

    // Check for % (all lines)
    if s.starts_with('%') {
        let rest = s[1..].trim();
        return (
            Some(LineRange::new(0, last_line)),
            format!(":{}", rest),
        );
    }

    // Try to parse address,address or single address
    let (addr1, after1) = parse_address(s, cursor_line, last_line);
    if addr1.is_none() {
        return (None, format!(":{}", s));
    }
    let start = addr1.unwrap();

    let after1 = after1.trim_start();
    if after1.starts_with(',') || after1.starts_with(';') {
        let rest = after1[1..].trim_start();
        let (addr2, after2) =
            parse_address(rest, cursor_line, last_line);
        let end = addr2.unwrap_or(start);
        let rng = LineRange::new(
            start.min(last_line),
            end.min(last_line),
        );
        let remaining = after2.trim();
        return (Some(rng), format!(":{}", remaining));
    }

    // Single address — range is just that line
    let rng =
        LineRange::single(start.min(last_line));
    let remaining = after1.trim();
    (Some(rng), format!(":{}", remaining))
}

/// Parse a single address from a string.
/// Returns (line_number, remaining_string).
pub(crate) fn parse_address(
    s: &str,
    cursor_line: usize,
    last_line: usize,
) -> (Option<usize>, &str) {
    if s.is_empty() {
        return (None, s);
    }

    let first = s.as_bytes()[0];
    match first {
        // Current line
        b'.' => {
            let rest = &s[1..];
            let (offset, rest) = parse_offset(rest);
            (
                Some(apply_offset(cursor_line, offset, last_line)),
                rest,
            )
        }
        // Last line
        b'$' => {
            let rest = &s[1..];
            let (offset, rest) = parse_offset(rest);
            (
                Some(apply_offset(last_line, offset, last_line)),
                rest,
            )
        }
        // Mark
        b'\'' => {
            if s.len() >= 2 {
                let _mark = s.as_bytes()[1] as char;
                // Mark resolution would go here
                // For now, treat as current line
                let rest = &s[2..];
                let (offset, rest) = parse_offset(rest);
                (
                    Some(apply_offset(
                        cursor_line,
                        offset,
                        last_line,
                    )),
                    rest,
                )
            } else {
                (None, s)
            }
        }
        // Visual marks
        b'<' => {
            let rest = &s[1..];
            (Some(cursor_line), rest) // placeholder
        }
        b'>' => {
            let rest = &s[1..];
            (Some(cursor_line), rest) // placeholder
        }
        // Absolute line number
        b'0'..=b'9' => {
            let end = s
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(s.len());
            let num: usize =
                s[..end].parse().unwrap_or(1);
            // Convert to 0-indexed
            let line = if num == 0 { 0 } else { num - 1 };
            let rest = &s[end..];
            let (offset, rest) = parse_offset(rest);
            (
                Some(apply_offset(line, offset, last_line)),
                rest,
            )
        }
        _ => (None, s),
    }
}

/// Parse an optional +N or -N offset.
pub(crate) fn parse_offset(s: &str) -> (isize, &str) {
    if s.is_empty() {
        return (0, s);
    }
    let first = s.as_bytes()[0];
    if first == b'+' || first == b'-' {
        let sign: isize = if first == b'+' { 1 } else { -1 };
        let rest = &s[1..];
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if end == 0 {
            return (sign, rest); // +/- alone means +1/-1
        }
        let num: isize =
            rest[..end].parse().unwrap_or(1);
        (sign * num, &rest[end..])
    } else {
        (0, s)
    }
}

/// Apply an offset to a line number, clamping to bounds.
pub(crate) fn apply_offset(
    base: usize,
    offset: isize,
    max_line: usize,
) -> usize {
    let result = base as isize + offset;
    if result < 0 {
        0
    } else if result as usize > max_line {
        max_line
    } else {
        result as usize
    }
}

pub(crate) fn current_line(state: &EditorState) -> usize {
    state
        .active_window
        .and_then(|wid| state.windows.get(&wid))
        .map(|w| w.cursor_line)
        .unwrap_or(0)
}

pub(crate) fn last_line_idx(state: &EditorState) -> usize {
    state
        .active_window
        .and_then(|wid| {
            let w = state.windows.get(&wid)?;
            let buf = state.buffers.get(&w.buffer_id)?;
            Some(buf.text.line_count().saturating_sub(1))
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Size;

    fn setup(text: &str) -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text(text);
        s.create_window(bid);
        s
    }

    #[test]
    fn parse_percent_range() {
        let s = setup("line1\nline2\nline3");
        let (rng, cmd) = parse_range(&s, ":%s/a/b");
        assert_eq!(rng, Some(LineRange::new(0, 2)));
        assert_eq!(cmd, ":s/a/b");
    }

    #[test]
    fn parse_line_number_range() {
        let s = setup("a\nb\nc\nd\ne");
        let (rng, cmd) = parse_range(&s, ":2,4d");
        assert_eq!(rng, Some(LineRange::new(1, 3)));
        assert_eq!(cmd, ":d");
    }

    #[test]
    fn parse_single_address() {
        let s = setup("a\nb\nc");
        let (rng, cmd) = parse_range(&s, ":3d");
        assert_eq!(rng, Some(LineRange::single(2)));
        assert_eq!(cmd, ":d");
    }

    #[test]
    fn parse_dot_address() {
        let s = setup("a\nb\nc");
        let (rng, cmd) = parse_range(&s, ":.,$d");
        assert_eq!(rng, Some(LineRange::new(0, 2)));
        assert_eq!(cmd, ":d");
    }

    #[test]
    fn parse_dollar_address() {
        let s = setup("a\nb\nc");
        let (rng, cmd) = parse_range(&s, ":$d");
        assert_eq!(rng, Some(LineRange::single(2)));
        assert_eq!(cmd, ":d");
    }

    #[test]
    fn parse_offset() {
        let s = setup("a\nb\nc\nd\ne");
        let (rng, cmd) = parse_range(&s, ":.+2d");
        assert_eq!(rng, Some(LineRange::single(2)));
        assert_eq!(cmd, ":d");
    }

    #[test]
    fn parse_no_range() {
        let s = setup("hello");
        let (rng, cmd) = parse_range(&s, ":w");
        assert!(rng.is_none());
        assert_eq!(cmd, ":w");
    }

    #[test]
    fn line_range_count() {
        let r = LineRange::new(2, 5);
        assert_eq!(r.line_count(), 4);
    }
}
