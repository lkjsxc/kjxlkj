//! Sentence text objects (is/as).
//!
//! A sentence ends with '.', '!', or '?' followed by optional
//! closing delimiters and whitespace/newline.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

use crate::text_objects::{TextObjectKind, TextObjectRange};

/// Sentence-ending punctuation.
fn is_sentence_end(c: char) -> bool {
    c == '.' || c == '!' || c == '?'
}

/// Resolve sentence text object at cursor.
pub fn resolve_sentence(
    kind: TextObjectKind,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    let total = rope.len_lines();
    // Flatten buffer into (line, col, char) triples.
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
    // Find sentence start: scan backward for sentence-ending
    // punctuation of the *previous* sentence, then skip whitespace.
    let mut start = find_sentence_start(&flat, cursor_idx);
    // Find sentence end: scan forward for sentence-ending punctuation.
    let end = find_sentence_end(&flat, cursor_idx);
    let mut end_idx = end;
    if matches!(kind, TextObjectKind::Around) {
        // Include trailing whitespace after the sentence.
        while end_idx + 1 < flat.len() && flat[end_idx + 1].2.is_whitespace() {
            end_idx += 1;
        }
    } else {
        // Inner: trim leading whitespace.
        while start < end_idx && flat[start].2.is_whitespace() {
            start += 1;
        }
    }
    if start > end_idx || end_idx >= flat.len() || start >= flat.len() {
        return None;
    }
    Some(TextObjectRange {
        start: CursorPosition::new(flat[start].0, flat[start].1),
        end: CursorPosition::new(flat[end_idx].0, flat[end_idx].1),
        linewise: false,
    })
}

fn find_sentence_start(flat: &[(usize, usize, char)], cursor: usize) -> usize {
    if cursor == 0 {
        return 0;
    }
    let mut i = cursor;
    // Skip whitespace before cursor.
    while i > 0 && flat[i - 1].2.is_whitespace() {
        i -= 1;
    }
    // Now scan backward until we find sentence-ending punctuation.
    while i > 0 {
        if is_sentence_end(flat[i - 1].2) {
            return i;
        }
        i -= 1;
    }
    0
}

fn find_sentence_end(flat: &[(usize, usize, char)], cursor: usize) -> usize {
    let mut i = cursor;
    while i < flat.len() {
        if is_sentence_end(flat[i].2) {
            // Include closing chars like ) ] " '
            while i + 1 < flat.len() && matches!(flat[i + 1].2, ')' | ']' | '"' | '\'') {
                i += 1;
            }
            return i;
        }
        i += 1;
    }
    flat.len().saturating_sub(1)
}
