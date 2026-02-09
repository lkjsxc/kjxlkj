//! Range parsing utilities for ex commands.

/// Parse a simple key string to Key events.
pub(crate) fn parse_key_string(
    s: &str,
) -> Vec<kjxlkj_core_types::Key> {
    let mut keys = Vec::new();
    for ch in s.chars() {
        keys.push(kjxlkj_core_types::Key::char(ch));
    }
    keys
}

/// Parse a range specification from a command string.
///
/// Returns (start_line, end_line, remaining_args).
/// Lines are 0-indexed. Range end is exclusive.
pub(crate) fn parse_range(
    args: &str,
    current_line: usize,
    last_line: usize,
) -> (usize, usize, String) {
    let args = args.trim();
    if args.is_empty() {
        return (
            current_line,
            current_line + 1,
            String::new(),
        );
    }
    if args.starts_with('%') {
        let rest = args[1..].trim().to_string();
        return (0, last_line + 1, rest);
    }
    if let Some(comma) = args.find(',') {
        let a = &args[..comma];
        let rest = &args[comma + 1..];
        let s = parse_line_spec(
            a, current_line, last_line,
        );
        let end_str: String = rest
            .chars()
            .take_while(|c| {
                c.is_ascii_digit()
                    || *c == '$'
                    || *c == '.'
            })
            .collect();
        let e = parse_line_spec(
            &end_str, current_line, last_line,
        );
        let remaining =
            rest[end_str.len()..].trim().to_string();
        (s, e + 1, remaining)
    } else {
        let line_str: String = args
            .chars()
            .take_while(|c| {
                c.is_ascii_digit()
                    || *c == '$'
                    || *c == '.'
            })
            .collect();
        if line_str.is_empty() {
            return (
                current_line,
                current_line + 1,
                args.to_string(),
            );
        }
        let l = parse_line_spec(
            &line_str, current_line, last_line,
        );
        let remaining =
            args[line_str.len()..].trim().to_string();
        (l, l + 1, remaining)
    }
}

fn parse_line_spec(
    s: &str,
    current: usize,
    last: usize,
) -> usize {
    let s = s.trim();
    match s {
        "." => current,
        "$" => last,
        _ => {
            s.parse::<usize>()
                .map(|n| n.saturating_sub(1))
                .unwrap_or(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_range_percent() {
        let (s, e, rest) = parse_range("% d", 5, 20);
        assert_eq!(s, 0);
        assert_eq!(e, 21);
        assert_eq!(rest, "d");
    }

    #[test]
    fn parse_range_explicit() {
        let (s, e, rest) = parse_range("3,7", 0, 20);
        assert_eq!(s, 2);
        assert_eq!(e, 7);
        assert!(rest.is_empty());
    }
}
