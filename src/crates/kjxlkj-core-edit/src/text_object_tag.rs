//! Tag text objects: `it` (inner tag), `at` (around tag).
//!
//! Matches XML/HTML tag pairs like `<div>content</div>`.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::TextObjectScope;

use crate::cursor::CursorPosition;
use crate::text_object_exec::TextObjectRange;

/// Resolve a tag text object at cursor.
pub(crate) fn resolve_tag(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<TextObjectRange> {
    let (open_start, open_end, close_start, close_end) = find_enclosing_tag(cursor, content)?;
    match scope {
        TextObjectScope::Inner => {
            let s = advance_past(content, open_end);
            let e = retreat_before(content, close_start);
            if s.line > e.line || (s.line == e.line && s.grapheme_offset > e.grapheme_offset) {
                return None; // Empty tag content
            }
            Some(TextObjectRange {
                start: s,
                end: e,
                linewise: false,
            })
        }
        TextObjectScope::Around => Some(TextObjectRange {
            start: open_start,
            end: close_end,
            linewise: false,
        }),
    }
}

/// Advance one grapheme past a position.
fn advance_past(content: &BufferContent, pos: CursorPosition) -> CursorPosition {
    let line = content.line_content(pos.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
    if pos.grapheme_offset + 1 < lg.count() {
        CursorPosition::new(pos.line, pos.grapheme_offset + 1)
    } else if pos.line + 1 < content.line_count() {
        CursorPosition::new(pos.line + 1, 0)
    } else {
        pos
    }
}

/// Retreat one grapheme before a position.
fn retreat_before(_content: &BufferContent, pos: CursorPosition) -> CursorPosition {
    if pos.grapheme_offset > 0 {
        CursorPosition::new(pos.line, pos.grapheme_offset - 1)
    } else if pos.line > 0 {
        CursorPosition::new(pos.line - 1, 0)
    } else {
        pos
    }
}

/// Find the enclosing tag pair around cursor.
/// Returns (open_tag_start, open_tag_end, close_tag_start, close_tag_end).
fn find_enclosing_tag(
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<(
    CursorPosition,
    CursorPosition,
    CursorPosition,
    CursorPosition,
)> {
    // Search backward for opening tag `<name...>`
    let (open_s, open_e, tag_name) = find_opening_tag_backward(cursor, content)?;
    // Search forward for matching closing tag `</name>`
    let search_from = advance_past(content, open_e);
    let (close_s, close_e) = find_closing_tag_forward(&search_from, content, &tag_name)?;
    Some((open_s, open_e, close_s, close_e))
}

/// Search backward for `<tagname...>` (not self-closing, not closing).
fn find_opening_tag_backward(
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<(CursorPosition, CursorPosition, String)> {
    let mut depth = 0i32;
    for line in (0..=cursor.line).rev() {
        let lc = content.line_content(line);
        let chars: Vec<char> = lc.chars().collect();
        let max_i = if line == cursor.line {
            chars.len().min(cursor.grapheme_offset + 1)
        } else {
            chars.len()
        };
        let mut i = max_i;
        while i > 0 {
            i -= 1;
            if chars[i] == '<' {
                // Check if it's a closing tag
                if i + 1 < chars.len() && chars[i + 1] == '/' {
                    depth += 1;
                    continue;
                }
                if depth > 0 {
                    depth -= 1;
                    continue;
                }
                // Extract tag name
                let tag_content: String = chars[i + 1..].iter().collect();
                let end_bracket = tag_content.find('>')?;
                let tag_str = &tag_content[..end_bracket];
                if tag_str.ends_with('/') {
                    continue; // Self-closing
                }
                let name = tag_str
                    .split(|c: char| c.is_whitespace())
                    .next()?
                    .to_string();
                if name.is_empty() {
                    continue;
                }
                let open_s = CursorPosition::new(line, i);
                let open_e = CursorPosition::new(line, i + 1 + end_bracket);
                return Some((open_s, open_e, name));
            }
        }
    }
    None
}

/// Search forward for `</tagname>`.
fn find_closing_tag_forward(
    start: &CursorPosition,
    content: &BufferContent,
    tag_name: &str,
) -> Option<(CursorPosition, CursorPosition)> {
    let target = format!("</{tag_name}>");
    let target_chars: Vec<char> = target.chars().collect();
    let tlen = target_chars.len();
    for line in start.line..content.line_count() {
        let lc = content.line_content(line);
        let chars: Vec<char> = lc.chars().collect();
        let si = if line == start.line {
            start.grapheme_offset
        } else {
            0
        };
        for i in si..chars.len() {
            if i + tlen <= chars.len() {
                let slice: Vec<char> = chars[i..i + tlen].to_vec();
                if slice == target_chars {
                    let close_s = CursorPosition::new(line, i);
                    let close_e = CursorPosition::new(line, i + tlen - 1);
                    return Some((close_s, close_e));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_tag_simple() {
        let c = BufferContent::from_str("<div>hello</div>\n");
        let cursor = CursorPosition::new(0, 6);
        let r = resolve_tag(TextObjectScope::Inner, &cursor, &c);
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start.grapheme_offset, 5);
        assert_eq!(r.end.grapheme_offset, 9);
    }

    #[test]
    fn around_tag_simple() {
        let c = BufferContent::from_str("<div>hello</div>\n");
        let cursor = CursorPosition::new(0, 6);
        let r = resolve_tag(TextObjectScope::Around, &cursor, &c);
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.start.grapheme_offset, 0);
        assert_eq!(r.end.grapheme_offset, 15);
    }
}
