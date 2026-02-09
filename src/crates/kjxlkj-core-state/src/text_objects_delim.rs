//! Delimiter-based text objects: pairs (parentheses, brackets)
//! and quotes (", ', `).

use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

use crate::text_objects::{TextObjectKind, TextObjectRange};

/// Dispatch delimiter object resolution.
pub fn resolve_delim_object(
    kind: TextObjectKind,
    obj: char,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    match obj {
        '(' | ')' | 'b' => resolve_pair(kind, '(', ')', pos, rope),
        '[' | ']' => resolve_pair(kind, '[', ']', pos, rope),
        '{' | '}' | 'B' => resolve_pair(kind, '{', '}', pos, rope),
        '<' | '>' => resolve_pair(kind, '<', '>', pos, rope),
        '"' => resolve_quote(kind, '"', pos, rope),
        '\'' => resolve_quote(kind, '\'', pos, rope),
        '`' => resolve_quote(kind, '`', pos, rope),
        _ => None,
    }
}

fn resolve_pair(
    kind: TextObjectKind,
    open: char,
    close: char,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    let total = rope.len_lines();
    let mut flat: Vec<(usize, usize, char)> = Vec::new();
    let mut cursor_idx = 0;
    for l in 0..total {
        let s: String = rope.line(l).chars().collect();
        for (g, ch) in s.chars().enumerate() {
            if l == pos.line && g == pos.grapheme {
                cursor_idx = flat.len();
            }
            flat.push((l, g, ch));
        }
    }
    if flat.is_empty() {
        return None;
    }
    // Search backward for open.
    let mut depth = 0i32;
    let mut open_idx = None;
    for i in (0..=cursor_idx).rev() {
        if flat[i].2 == close && i != cursor_idx {
            depth += 1;
        } else if flat[i].2 == open {
            if depth == 0 {
                open_idx = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    let open_idx = open_idx?;
    // Search forward for close.
    depth = 0;
    let mut close_idx = None;
    for (i, item) in flat.iter().enumerate().skip(open_idx + 1) {
        if item.2 == open {
            depth += 1;
        } else if item.2 == close {
            if depth == 0 {
                close_idx = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    let close_idx = close_idx?;
    let (sl, sg) = match kind {
        TextObjectKind::Inner => {
            if open_idx + 1 < flat.len() {
                (flat[open_idx + 1].0, flat[open_idx + 1].1)
            } else {
                return None;
            }
        }
        TextObjectKind::Around => (flat[open_idx].0, flat[open_idx].1),
    };
    let (el, eg) = match kind {
        TextObjectKind::Inner => {
            if close_idx > 0 {
                (flat[close_idx - 1].0, flat[close_idx - 1].1)
            } else {
                return None;
            }
        }
        TextObjectKind::Around => (flat[close_idx].0, flat[close_idx].1),
    };
    Some(TextObjectRange {
        start: CursorPosition::new(sl, sg),
        end: CursorPosition::new(el, eg),
        linewise: false,
    })
}

fn resolve_quote(
    kind: TextObjectKind,
    quote: char,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    if pos.line >= rope.len_lines() {
        return None;
    }
    let s: String = rope.line(pos.line).chars().collect();
    let chars: Vec<char> = s.chars().collect();
    let g = pos.grapheme;
    let mut quotes: Vec<usize> = Vec::new();
    for (i, c) in chars.iter().enumerate() {
        if *c == quote {
            quotes.push(i);
        }
    }
    for pair in quotes.chunks(2) {
        if pair.len() == 2 && pair[0] <= g && g <= pair[1] {
            let (sl, el) = match kind {
                TextObjectKind::Inner => (pair[0] + 1, pair[1] - 1),
                TextObjectKind::Around => (pair[0], pair[1]),
            };
            if sl <= el {
                return Some(TextObjectRange {
                    start: CursorPosition::new(pos.line, sl),
                    end: CursorPosition::new(pos.line, el),
                    linewise: false,
                });
            }
        }
    }
    None
}
