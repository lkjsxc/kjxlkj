//! Ex command range and address parsing.
//!
//! Parses range specifiers like `%`, `.`, `$`, `'a`, `/pattern/`,
//! line numbers, and offsets used in Ex commands.

/// A parsed address component.
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

/// A parsed range for an Ex command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    None,
    Single(Address),
    FromTo(Address, Address),
    Entire,
}

/// Resolve an address to a concrete line number.
pub fn resolve_address(addr: &Address, current_line: usize, last_line: usize) -> usize {
    match addr {
        Address::CurrentLine => current_line,
        Address::LastLine => last_line,
        Address::LineNumber(n) => (*n).min(last_line),
        Address::Offset(base, off) => {
            let base_line = resolve_address(base, current_line, last_line) as i64;
            let result = (base_line + off).max(0) as usize;
            result.min(last_line)
        }
        Address::Mark(_) | Address::ForwardSearch(_) | Address::BackwardSearch(_) => current_line,
    }
}

/// Resolve a range to (start_line, end_line) inclusive.
pub fn resolve_range(range: &Range, current: usize, last: usize) -> (usize, usize) {
    match range {
        Range::None => (current, current),
        Range::Single(addr) => {
            let line = resolve_address(addr, current, last);
            (line, line)
        }
        Range::FromTo(from, to) => {
            let s = resolve_address(from, current, last);
            let e = resolve_address(to, current, last);
            if s <= e { (s, e) } else { (e, s) }
        }
        Range::Entire => (0, last),
    }
}

/// Parse a range string into a Range value.
pub fn parse_range(input: &str) -> Range {
    let input = input.trim();
    if input.is_empty() { return Range::None; }
    if input == "%" { return Range::Entire; }
    if let Some((left, right)) = input.split_once(',') {
        let from = parse_address(left.trim());
        let to = parse_address(right.trim());
        return Range::FromTo(from, to);
    }
    Range::Single(parse_address(input))
}

/// Parse a single address string.
pub fn parse_address(input: &str) -> Address {
    let input = input.trim();
    if input == "." { return Address::CurrentLine; }
    if input == "$" { return Address::LastLine; }
    if let Some(rest) = input.strip_prefix('\'') {
        if let Some(c) = rest.chars().next() {
            return Address::Mark(c);
        }
    }
    if input.starts_with('/') && input.len() > 2 && input.ends_with('/') {
        return Address::ForwardSearch(input[1..input.len() - 1].to_string());
    }
    if input.starts_with('?') && input.len() > 2 && input.ends_with('?') {
        return Address::BackwardSearch(input[1..input.len() - 1].to_string());
    }
    // Check for offset: <base>+N or <base>-N
    if let Some(pos) = input.rfind('+') {
        if pos > 0 {
            let base = parse_address(&input[..pos]);
            let off: i64 = input[pos + 1..].parse().unwrap_or(1);
            return Address::Offset(Box::new(base), off);
        }
    }
    if let Some(pos) = input.rfind('-') {
        if pos > 0 {
            let base = parse_address(&input[..pos]);
            let off: i64 = input[pos + 1..].parse().unwrap_or(1);
            return Address::Offset(Box::new(base), -off);
        }
    }
    if let Ok(n) = input.parse::<usize>() {
        return Address::LineNumber(n);
    }
    Address::CurrentLine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_percent() {
        assert_eq!(parse_range("%"), Range::Entire);
    }

    #[test]
    fn parse_single_line() {
        assert_eq!(parse_range("5"), Range::Single(Address::LineNumber(5)));
    }

    #[test]
    fn parse_current_and_last() {
        assert_eq!(parse_range(".,$"), Range::FromTo(Address::CurrentLine, Address::LastLine));
    }

    #[test]
    fn parse_mark_address() {
        assert_eq!(parse_address("'a"), Address::Mark('a'));
    }

    #[test]
    fn parse_forward_search() {
        assert_eq!(parse_address("/foo/"), Address::ForwardSearch("foo".into()));
    }

    #[test]
    fn resolve_entire() {
        assert_eq!(resolve_range(&Range::Entire, 5, 100), (0, 100));
    }

    #[test]
    fn resolve_single_current() {
        let r = Range::Single(Address::CurrentLine);
        assert_eq!(resolve_range(&r, 10, 50), (10, 10));
    }

    #[test]
    fn resolve_offset() {
        let addr = Address::Offset(Box::new(Address::CurrentLine), 3);
        assert_eq!(resolve_address(&addr, 5, 100), 8);
    }

    #[test]
    fn resolve_negative_offset_clamp() {
        let addr = Address::Offset(Box::new(Address::LineNumber(2)), -5);
        assert_eq!(resolve_address(&addr, 0, 100), 0);
    }
}
