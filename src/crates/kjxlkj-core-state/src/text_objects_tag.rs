//! Tag text objects (it/at) for HTML/XML tag pairs.
//!
//! `it` selects content between matching open/close tags.
//! `at` selects including the tags themselves.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

use crate::text_objects::{TextObjectKind, TextObjectRange};

/// Resolve tag text object at cursor position.
pub fn resolve_tag_object(
    kind: TextObjectKind,
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
    let text: String = flat.iter().map(|t| t.2).collect();
    // Find the innermost tag pair enclosing cursor_idx.
    let (open_start, open_end, close_start, close_end) = find_enclosing_tags(&text, cursor_idx)?;
    let (sl, sg, el, eg) = match kind {
        TextObjectKind::Inner => {
            let s = open_end + 1;
            let e = close_start.saturating_sub(1);
            if s > e || s >= flat.len() || e >= flat.len() {
                return None;
            }
            (flat[s].0, flat[s].1, flat[e].0, flat[e].1)
        }
        TextObjectKind::Around => {
            let e = close_end.min(flat.len() - 1);
            (flat[open_start].0, flat[open_start].1, flat[e].0, flat[e].1)
        }
    };
    Some(TextObjectRange {
        start: CursorPosition::new(sl, sg),
        end: CursorPosition::new(el, eg),
        linewise: false,
    })
}

/// Find enclosing tag pair: returns (open_start, open_end, close_start, close_end).
fn find_enclosing_tags(text: &str, cursor: usize) -> Option<(usize, usize, usize, usize)> {
    let bytes = text.as_bytes();
    // Search backward for '<' that starts an open tag.
    let mut i = cursor;
    loop {
        while i > 0 && bytes[i] != b'<' {
            i -= 1;
        }
        if bytes[i] != b'<' {
            return None;
        }
        // Check it's an open tag (not </...).
        if i + 1 < bytes.len() && bytes[i + 1] != b'/' {
            let tag_start = i;
            let tag_end = find_char(bytes, b'>', tag_start)?;
            // Extract tag name.
            let name = extract_tag_name(&text[tag_start + 1..tag_end]);
            if name.is_empty() {
                if i == 0 {
                    return None;
                }
                i -= 1;
                continue;
            }
            // Search forward for matching close tag.
            let close_tag = format!("</{name}>");
            if let Some(ci) = text[tag_end + 1..].find(&close_tag) {
                let close_start = tag_end + 1 + ci;
                let close_end = close_start + close_tag.len() - 1;
                if close_end > cursor || tag_start <= cursor {
                    return Some((tag_start, tag_end, close_start, close_end));
                }
            }
        }
        if i == 0 {
            return None;
        }
        i -= 1;
    }
}

fn find_char(bytes: &[u8], target: u8, start: usize) -> Option<usize> {
    for (i, &b) in bytes.iter().enumerate().skip(start) {
        if b == target {
            return Some(i);
        }
    }
    None
}

fn extract_tag_name(s: &str) -> &str {
    let end = s
        .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
        .unwrap_or(s.len());
    &s[..end]
}
