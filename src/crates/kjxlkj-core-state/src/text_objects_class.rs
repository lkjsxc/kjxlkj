//! Class/function text objects (ic/ac/if/af) — stub implementation.
//!
//! Without a real AST, we approximate using brace-matching:
//! a class/function is the innermost `{...}` block containing the cursor.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

use crate::text_objects::{TextObjectKind, TextObjectRange};

/// Resolve a class or function text object.
/// `obj_char`: 'c' for class, 'f' for function.
/// Both use brace-matching as a heuristic.
pub fn resolve_class_or_func(
    kind: TextObjectKind,
    _obj_char: char,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    let total = rope.len_lines();
    if total == 0 {
        return None;
    }
    // Find enclosing { } pair.
    let (brace_line_start, brace_line_end) = find_enclosing_braces(pos, rope)?;
    let (start, end) = match kind {
        TextObjectKind::Inner => (brace_line_start + 1, brace_line_end.saturating_sub(1)),
        TextObjectKind::Around => (brace_line_start, brace_line_end),
    };
    if start > end {
        return None;
    }
    let end_col = {
        let s: String = rope.line(end).chars().collect();
        s.trim_end_matches('\n').len().saturating_sub(1)
    };
    Some(TextObjectRange {
        start: CursorPosition::new(start, 0),
        end: CursorPosition::new(end, end_col),
        linewise: true,
    })
}

/// Find the nearest enclosing `{`...`}` pair by line.
fn find_enclosing_braces(pos: CursorPosition, rope: &Rope) -> Option<(usize, usize)> {
    let total = rope.len_lines();
    // Walk backwards to find opening brace.
    let mut depth: i32 = 0;
    let mut open_line = None;
    let mut line = pos.line;
    loop {
        let s: String = rope.line(line).chars().collect();
        for ch in s.chars().rev() {
            match ch {
                '}' => depth += 1,
                '{' => {
                    if depth == 0 {
                        open_line = Some(line);
                        break;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        if open_line.is_some() {
            break;
        }
        if line == 0 {
            break;
        }
        line -= 1;
    }
    let open_line = open_line?;
    // Walk forward from open_line to find closing brace.
    depth = 0;
    for l in open_line..total {
        let s: String = rope.line(l).chars().collect();
        for ch in s.chars() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((open_line, l));
                    }
                }
                _ => {}
            }
        }
    }
    None
}
