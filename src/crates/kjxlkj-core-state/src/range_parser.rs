//! Range and address parsing for Ex commands.

/// An address in a range specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address {
    CurrentLine,
    LastLine,
    LineNumber(usize),
    Mark(char),
    ForwardSearch(String),
    BackwardSearch(String),
    Offset(Box<Address>, i64),
}

/// A range specification for Ex commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    None,
    Single(Address),
    FromTo(Address, Address),
    Entire,
}

/// Parse a range prefix from an Ex command string.
/// Returns (range, remaining_input).
pub fn parse_range(input: &str) -> (Range, &str) {
    let input = input.trim_start();
    if input.starts_with('%') {
        return (Range::Entire, &input[1..]);
    }
    if let Some((addr1, rest)) = parse_address(input) {
        let rest = rest.trim_start();
        if let Some(rest) = rest.strip_prefix(',') {
            if let Some((addr2, rest2)) = parse_address(rest.trim_start()) {
                return (Range::FromTo(addr1, addr2), rest2);
            }
            return (Range::FromTo(addr1, Address::CurrentLine), rest);
        }
        if let Some(rest) = rest.strip_prefix(';') {
            if let Some((addr2, rest2)) = parse_address(rest.trim_start()) {
                return (Range::FromTo(addr1, addr2), rest2);
            }
        }
        return (Range::Single(addr1), rest);
    }
    (Range::None, input)
}

/// Parse a single address from input.
pub fn parse_address(input: &str) -> Option<(Address, &str)> {
    let input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    let first = input.as_bytes()[0];
    // Current line
    if first == b'.' {
        let rest = &input[1..];
        return Some(apply_offset(Address::CurrentLine, rest));
    }
    // Last line
    if first == b'$' {
        let rest = &input[1..];
        return Some(apply_offset(Address::LastLine, rest));
    }
    // Mark
    if first == b'\'' && input.len() > 1 {
        let mark = input.as_bytes()[1] as char;
        let rest = &input[2..];
        return Some(apply_offset(Address::Mark(mark), rest));
    }
    // Forward search
    if first == b'/' {
        if let Some(end) = input[1..].find('/') {
            let pat = &input[1..1 + end];
            let rest = &input[2 + end..];
            return Some(apply_offset(Address::ForwardSearch(pat.to_string()), rest));
        }
    }
    // Backward search
    if first == b'?' {
        if let Some(end) = input[1..].find('?') {
            let pat = &input[1..1 + end];
            let rest = &input[2 + end..];
            return Some(apply_offset(Address::BackwardSearch(pat.to_string()), rest));
        }
    }
    // Line number
    if first.is_ascii_digit() {
        let mut end = 0;
        while end < input.len() && input.as_bytes()[end].is_ascii_digit() {
            end += 1;
        }
        let num: usize = input[..end].parse().ok()?;
        let rest = &input[end..];
        return Some(apply_offset(Address::LineNumber(num), rest));
    }
    None
}

/// Parse a +N or -N offset after an address.
fn apply_offset(addr: Address, rest: &str) -> (Address, &str) {
    let rest = rest.trim_start();
    if let Some(r) = rest.strip_prefix('+') {
        let (n, r2) = parse_number(r);
        return (Address::Offset(Box::new(addr), n as i64), r2);
    }
    if let Some(r) = rest.strip_prefix('-') {
        let (n, r2) = parse_number(r);
        return (Address::Offset(Box::new(addr), -(n as i64)), r2);
    }
    (addr, rest)
}

fn parse_number(input: &str) -> (usize, &str) {
    let mut end = 0;
    while end < input.len() && input.as_bytes()[end].is_ascii_digit() {
        end += 1;
    }
    if end == 0 {
        return (1, input);
    }
    let n: usize = input[..end].parse().unwrap_or(1);
    (n, &input[end..])
}

/// Resolve a range to (start_line, end_line) 0-indexed.
pub fn resolve_range(
    range: &Range,
    current_line: usize,
    last_line: usize,
) -> (usize, usize) {
    match range {
        Range::None => (current_line, current_line),
        Range::Entire => (0, last_line),
        Range::Single(addr) => {
            let l = resolve_address(addr, current_line, last_line);
            (l, l)
        }
        Range::FromTo(a, b) => {
            let start = resolve_address(a, current_line, last_line);
            let end = resolve_address(b, current_line, last_line);
            (start, end)
        }
    }
}

fn resolve_address(addr: &Address, current: usize, last: usize) -> usize {
    match addr {
        Address::CurrentLine => current,
        Address::LastLine => last,
        Address::LineNumber(n) => n.saturating_sub(1).min(last),
        Address::Mark(_) => current, // marks resolved externally
        Address::ForwardSearch(_) | Address::BackwardSearch(_) => current,
        Address::Offset(base, off) => {
            let base = resolve_address(base, current, last);
            let result = base as i64 + off;
            result.max(0) as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_entire() {
        let (r, rest) = parse_range("%d");
        assert_eq!(r, Range::Entire);
        assert_eq!(rest, "d");
    }

    #[test]
    fn parse_line_number() {
        let (r, _) = parse_range("42d");
        assert!(matches!(r, Range::Single(Address::LineNumber(42))));
    }

    #[test]
    fn parse_from_to() {
        let (r, rest) = parse_range("1,5d");
        assert!(matches!(r, Range::FromTo(Address::LineNumber(1), Address::LineNumber(5))));
        assert_eq!(rest, "d");
    }

    #[test]
    fn resolve_entire() {
        let (s, e) = resolve_range(&Range::Entire, 5, 100);
        assert_eq!((s, e), (0, 100));
    }

    #[test]
    fn parse_offset() {
        let (addr, _) = parse_address(".+3").unwrap();
        assert!(matches!(addr, Address::Offset(_, 3)));
    }
}
