use crate::text_objects::{TextObjectKind, TextObjectRange};
/// Argument text objects: ia (inner argument), aa (around argument).
/// Selects comma-delimited arguments inside parentheses.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

/// Resolve an argument text object at the cursor position.
pub fn resolve_argument(
    kind: TextObjectKind,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    if pos.line >= rope.len_lines() {
        return None;
    }
    // Gather text: we scan the current line for enclosing parens.
    let line_s: String = rope.line(pos.line).chars().collect();
    let chars: Vec<char> = line_s.chars().collect();
    let col = pos.grapheme.min(chars.len().saturating_sub(1));

    // Find opening paren before or at cursor, tracking depth.
    let mut open = None;
    let mut depth = 0i32;
    for i in (0..=col).rev() {
        match chars[i] {
            ')' => depth += 1,
            '(' => {
                if depth == 0 {
                    open = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let open_pos = open?;

    // Find matching close paren.
    depth = 0;
    let mut close = None;
    for (off, &ch) in chars[open_pos + 1..].iter().enumerate() {
        let i = open_pos + 1 + off;
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    close = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let close_pos = close?;

    // Split arguments inside parens by commas (top-level only).
    let inner = &chars[open_pos + 1..close_pos];
    let mut arg_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    depth = 0;
    for (i, &ch) in inner.iter().enumerate() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                arg_ranges.push((start, i));
                start = i + 1;
            }
            _ => {}
        }
    }
    arg_ranges.push((start, inner.len()));

    // Find which argument the cursor is in.
    let cursor_in_inner = col.saturating_sub(open_pos + 1);
    let mut arg_idx = 0;
    for (i, &(s, e)) in arg_ranges.iter().enumerate() {
        if cursor_in_inner >= s && cursor_in_inner < e {
            arg_idx = i;
            break;
        }
        if i == arg_ranges.len() - 1 {
            arg_idx = i;
        }
    }

    let (arg_start, arg_end) = arg_ranges[arg_idx];
    let abs_start = open_pos + 1 + arg_start;
    let abs_end = open_pos + 1 + arg_end;

    match kind {
        TextObjectKind::Inner => {
            // Trim leading/trailing whitespace from argument.
            let mut s = abs_start;
            let mut e = abs_end.saturating_sub(1);
            while s < abs_end && chars[s] == ' ' {
                s += 1;
            }
            while e > s && chars[e] == ' ' {
                e -= 1;
            }
            Some(TextObjectRange {
                start: CursorPosition::new(pos.line, s),
                end: CursorPosition::new(pos.line, e),
                linewise: false,
            })
        }
        TextObjectKind::Around => {
            // Include trailing comma+space, or leading comma+space if last arg.
            let (s, e) = if arg_idx < arg_ranges.len() - 1 {
                let next_start = open_pos + 1 + arg_ranges[arg_idx + 1].0;
                (abs_start, next_start.saturating_sub(1))
            } else if arg_idx > 0 {
                let prev_end = open_pos + 1 + arg_ranges[arg_idx - 1].1;
                (prev_end, abs_end.saturating_sub(1))
            } else {
                (abs_start, abs_end.saturating_sub(1))
            };
            Some(TextObjectRange {
                start: CursorPosition::new(pos.line, s),
                end: CursorPosition::new(pos.line, e),
                linewise: false,
            })
        }
    }
}
