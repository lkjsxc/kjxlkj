//! Rich range parsing with pattern and mark support.
//!
//! Extends basic range parsing to handle `/pattern/`,
//! `?pattern?`, and `'{mark}` range addresses.

use crate::ex_parse::{parse_offset, ExRange};

/// Context needed for rich range parsing.
pub struct RangeContext<'a> {
    pub current_line: usize,
    pub total_lines: usize,
    /// Buffer text lines (0-indexed).
    pub lines: &'a [&'a str],
    /// Mark lookup: given a mark char, return its line.
    pub mark_line: Option<&'a dyn Fn(char) -> Option<usize>>,
}

/// Parse a range with full context (patterns, marks).
pub fn parse_range_ctx<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<ExRange>, &'a str) {
    let input = input.trim_start();
    if input.is_empty() {
        return (None, input);
    }
    if let Some(rest) = input.strip_prefix('%') {
        return (Some(ExRange::all(ctx.total_lines)), rest);
    }
    let (addr1, rest1) = parse_address_ctx(input, ctx);
    if let Some(start) = addr1 {
        let rest1 = rest1.trim_start();
        if let Some(rest2) = rest1.strip_prefix(',') {
            let rest2 = rest2.trim_start();
            let (addr2, rest3) = parse_address_ctx(rest2, ctx);
            if let Some(end) = addr2 {
                return (Some(ExRange { start, end }.clamp(ctx.total_lines)), rest3);
            }
            return (
                Some(
                    ExRange {
                        start,
                        end: ctx.current_line,
                    }
                    .clamp(ctx.total_lines),
                ),
                rest2,
            );
        }
        return (Some(ExRange::single(start).clamp(ctx.total_lines)), rest1);
    }
    (None, input)
}

fn parse_address_ctx<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let input = input.trim_start();
    if input.is_empty() {
        return (None, input);
    }
    let first = input.as_bytes()[0];

    if first == b'/' {
        return parse_pattern_forward(input, ctx);
    }
    if first == b'?' {
        return parse_pattern_backward(input, ctx);
    }
    if first == b'\'' && input.len() >= 2 {
        let mark_ch = input.as_bytes()[1] as char;
        let rest = &input[2..];
        if let Some(ref f) = ctx.mark_line {
            if let Some(line) = f(mark_ch) {
                let (offset, rest) = parse_offset(rest);
                let l = (line as isize + offset).max(0) as usize;
                return (Some(l), rest);
            }
        }
        return (None, rest);
    }
    // Delegate to basic address parsing for . $ digits.
    if first == b'.' {
        let (offset, rest) = parse_offset(&input[1..]);
        let l = (ctx.current_line as isize + offset).max(0) as usize;
        return (Some(l), rest);
    }
    if first == b'$' {
        let (offset, rest) = parse_offset(&input[1..]);
        let last = ctx.total_lines.saturating_sub(1) as isize;
        let l = (last + offset).max(0) as usize;
        return (Some(l), rest);
    }
    if first.is_ascii_digit() {
        let end = input
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input.len());
        if let Ok(n) = input[..end].parse::<usize>() {
            let line = n.saturating_sub(1);
            let (offset, rest) = parse_offset(&input[end..]);
            let l = (line as isize + offset).max(0) as usize;
            return (Some(l), rest);
        }
    }
    (None, input)
}

fn parse_pattern_forward<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let rest = &input[1..];
    let end = rest.find('/').unwrap_or(rest.len());
    let pattern = &rest[..end];
    let after = if end < rest.len() {
        &rest[end + 1..]
    } else {
        &rest[end..]
    };
    let (offset, after) = parse_offset(after);
    for i in 1..ctx.total_lines {
        let idx = (ctx.current_line + i) % ctx.total_lines;
        if idx < ctx.lines.len() && ctx.lines[idx].contains(pattern) {
            let l = (idx as isize + offset).max(0) as usize;
            return (Some(l), after);
        }
    }
    (None, after)
}

fn parse_pattern_backward<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let rest = &input[1..];
    let end = rest.find('?').unwrap_or(rest.len());
    let pattern = &rest[..end];
    let after = if end < rest.len() {
        &rest[end + 1..]
    } else {
        &rest[end..]
    };
    let (offset, after) = parse_offset(after);
    for i in 1..ctx.total_lines {
        let idx = (ctx.current_line + ctx.total_lines - i) % ctx.total_lines;
        if idx < ctx.lines.len() && ctx.lines[idx].contains(pattern) {
            let l = (idx as isize + offset).max(0) as usize;
            return (Some(l), after);
        }
    }
    (None, after)
}
