//! Rich range parsing: /pattern/, ?pattern?, '{mark}, +N/-N addresses.

use crate::ex_parse::{parse_offset, ExRange};

/// Context needed for rich range parsing.
pub struct RangeContext<'a> {
    pub current_line: usize,
    pub total_lines: usize,
    /// Buffer text lines (0-indexed).
    pub lines: &'a [&'a str],
    /// Mark lookup: given a mark char, return its line.
    pub mark_line: Option<&'a dyn Fn(char) -> Option<usize>>,
    /// Last search pattern for `\/` and `\?`.
    pub last_search: Option<&'a str>,
}

#[rustfmt::skip]
pub fn parse_range_ctx<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<ExRange>, &'a str) {
    let input = input.trim_start();
    if input.is_empty() { return (None, input); }
    if let Some(rest) = input.strip_prefix('%') { return (Some(ExRange::all(ctx.total_lines)), rest); }
    let (addr1, rest1) = parse_address_ctx(input, ctx);
    if let Some(start) = addr1 {
        let rest1 = rest1.trim_start();
        if let Some(rest2) = rest1.strip_prefix(',') {
            let rest2 = rest2.trim_start();
            let (addr2, rest3) = parse_address_ctx(rest2, ctx);
            if let Some(end) = addr2 { return (Some(ExRange { start, end }.clamp(ctx.total_lines)), rest3); }
            return (Some(ExRange { start, end: ctx.current_line }.clamp(ctx.total_lines)), rest2);
        }
        if let Some(rest2) = rest1.strip_prefix(';') {
            let rest2 = rest2.trim_start();
            let ctx2 = RangeContext { current_line: start, total_lines: ctx.total_lines, lines: ctx.lines, mark_line: ctx.mark_line, last_search: ctx.last_search };
            let (addr2, rest3) = parse_address_ctx(rest2, &ctx2);
            if let Some(end) = addr2 { return (Some(ExRange { start, end }.clamp(ctx.total_lines)), rest3); }
            return (Some(ExRange::single(start).clamp(ctx.total_lines)), rest2);
        }
        return (Some(ExRange::single(start).clamp(ctx.total_lines)), rest1);
    }
    (None, input)
}

fn parse_address_ctx<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let input = input.trim_start();
    if input.is_empty() { return (None, input); }
    let first = input.as_bytes()[0];
    if first == b'/' { return parse_pattern_forward(input, ctx); }
    if first == b'?' { return parse_pattern_backward(input, ctx); }
    if first == b'\\' && input.len() >= 2 {
        let second = input.as_bytes()[1];
        if second == b'/' || second == b'?' {
            let search_fn = if second == b'/' { search_forward } else { search_backward };
            if let Some(pat) = ctx.last_search {
                let (offset, after) = parse_offset(&input[2..]);
                return search_fn(ctx, pat, offset, after);
            }
            return (None, &input[2..]);
        }
    }
    if first == b'\'' && input.len() >= 2 {
        let mark_ch = input.as_bytes()[1] as char;
        let rest = &input[2..];
        if let Some(ref f) = ctx.mark_line {
            if let Some(line) = f(mark_ch) {
                let (offset, rest) = parse_offset(rest);
                return (Some((line as isize + offset).max(0) as usize), rest);
            }
        }
        return (None, rest);
    }
    if first == b'.' { let (offset, rest) = parse_offset(&input[1..]); return (Some((ctx.current_line as isize + offset).max(0) as usize), rest); }
    if first == b'$' { let (offset, rest) = parse_offset(&input[1..]); return (Some((ctx.total_lines.saturating_sub(1) as isize + offset).max(0) as usize), rest); }
    if first.is_ascii_digit() {
        let end = input.find(|c: char| !c.is_ascii_digit()).unwrap_or(input.len());
        if let Ok(n) = input[..end].parse::<usize>() {
            let (offset, rest) = parse_offset(&input[end..]);
            return (Some((n.saturating_sub(1) as isize + offset).max(0) as usize), rest);
        }
    }
    if first == b'+' || first == b'-' { let (offset, rest) = parse_offset(input); return (Some((ctx.current_line as isize + offset).max(0) as usize), rest); }
    // Expression address: (expr) evaluates to a line number.
    if first == b'(' {
        if let Some(close) = input.find(')') {
            let expr = &input[1..close];
            let rest = &input[close + 1..];
            if let Ok(val) = crate::expr_eval::eval_expression(expr) {
                if let Ok(n) = val.parse::<usize>() { return (Some(n.saturating_sub(1)), rest); }
            }
        }
    }
    (None, input)
}

#[rustfmt::skip]
fn parse_pattern_forward<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let rest = &input[1..]; let end = rest.find('/').unwrap_or(rest.len());
    let pattern = &rest[..end];
    let after = if end < rest.len() { &rest[end+1..] } else { &rest[end..] };
    let (offset, after) = parse_offset(after); search_forward(ctx, pattern, offset, after)
}
#[rustfmt::skip]
fn parse_pattern_backward<'a>(input: &'a str, ctx: &RangeContext<'_>) -> (Option<usize>, &'a str) {
    let rest = &input[1..]; let end = rest.find('?').unwrap_or(rest.len());
    let pattern = &rest[..end];
    let after = if end < rest.len() { &rest[end+1..] } else { &rest[end..] };
    let (offset, after) = parse_offset(after); search_backward(ctx, pattern, offset, after)
}

#[rustfmt::skip]
fn search_forward<'a>(ctx: &RangeContext<'_>, pattern: &str, offset: isize, after: &'a str) -> (Option<usize>, &'a str) {
    for i in 1..ctx.total_lines {
        let idx = (ctx.current_line + i) % ctx.total_lines;
        if idx < ctx.lines.len() && ctx.lines[idx].contains(pattern) { return (Some((idx as isize + offset).max(0) as usize), after); }
    } (None, after)
}
#[rustfmt::skip]
fn search_backward<'a>(ctx: &RangeContext<'_>, pattern: &str, offset: isize, after: &'a str) -> (Option<usize>, &'a str) {
    for i in 1..ctx.total_lines {
        let idx = (ctx.current_line + ctx.total_lines - i) % ctx.total_lines;
        if idx < ctx.lines.len() && ctx.lines[idx].contains(pattern) { return (Some((idx as isize + offset).max(0) as usize), after); }
    } (None, after)
}
