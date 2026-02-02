//! Miscellaneous motion implementations.

use kjxlkj_core_types::motion::MotionResult;
use super::MotionContext;

/// Executes paragraph forward motion.
pub fn paragraph_forward<C: MotionContext>(ctx: &C, mut line: usize) -> MotionResult {
    let max_line = ctx.line_count().saturating_sub(1);

    // Skip non-empty lines
    while line < max_line && !ctx.line_content(line).trim().is_empty() {
        line += 1;
    }
    // Skip empty lines
    while line < max_line && ctx.line_content(line).trim().is_empty() {
        line += 1;
    }

    MotionResult { line, column: 0, wrapped: false, hit_boundary: line >= max_line }
}

/// Executes paragraph backward motion.
pub fn paragraph_backward<C: MotionContext>(ctx: &C, mut line: usize) -> MotionResult {
    while line > 0 && ctx.line_content(line).trim().is_empty() {
        line -= 1;
    }
    while line > 0 && !ctx.line_content(line - 1).trim().is_empty() {
        line -= 1;
    }
    MotionResult { line, column: 0, wrapped: false, hit_boundary: line == 0 }
}

/// Executes matching bracket motion.
pub fn matching_bracket<C: MotionContext>(ctx: &C, line: usize, col: usize) -> MotionResult {
    let content = ctx.line_content(line);
    let chars: Vec<char> = content.chars().collect();

    if col >= chars.len() {
        return MotionResult { line, column: col, wrapped: false, hit_boundary: true };
    }

    let c = chars[col];
    let (target, forward) = match c {
        '(' => (')', true),
        ')' => ('(', false),
        '[' => (']', true),
        ']' => ('[', false),
        '{' => ('}', true),
        '}' => ('{', false),
        _ => return MotionResult { line, column: col, wrapped: false, hit_boundary: true },
    };

    let mut depth = 1;
    let mut search_col = col;

    if forward {
        search_col += 1;
        while search_col < chars.len() && depth > 0 {
            if chars[search_col] == c { depth += 1; }
            else if chars[search_col] == target { depth -= 1; }
            if depth > 0 { search_col += 1; }
        }
    } else if search_col > 0 {
        search_col -= 1;
        while search_col > 0 && depth > 0 {
            if chars[search_col] == c { depth += 1; }
            else if chars[search_col] == target { depth -= 1; }
            if depth > 0 { search_col -= 1; }
        }
        if depth > 0 && chars[0] == target { depth -= 1; }
    }

    if depth == 0 {
        MotionResult { line, column: search_col, wrapped: false, hit_boundary: false }
    } else {
        MotionResult { line, column: col, wrapped: false, hit_boundary: true }
    }
}

/// Moves to document start.
pub fn document_start() -> MotionResult {
    MotionResult { line: 0, column: 0, wrapped: false, hit_boundary: false }
}

/// Moves to document end.
pub fn document_end<C: MotionContext>(ctx: &C) -> MotionResult {
    let line = ctx.line_count().saturating_sub(1);
    MotionResult { line, column: 0, wrapped: false, hit_boundary: false }
}

/// Goes to a specific line.
pub fn go_to_line<C: MotionContext>(ctx: &C, target_line: usize) -> MotionResult {
    let line = target_line.min(ctx.line_count().saturating_sub(1));
    MotionResult { line, column: 0, wrapped: false, hit_boundary: false }
}

/// Goes to a specific column.
pub fn go_to_column<C: MotionContext>(ctx: &C, line: usize, target_col: usize) -> MotionResult {
    let line_len = ctx.line_len(line);
    let col = target_col.min(line_len.saturating_sub(1));
    MotionResult { line, column: col, wrapped: false, hit_boundary: false }
}
