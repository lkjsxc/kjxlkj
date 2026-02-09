//! Operator execution helpers: text extraction,
//! indentation, case operations.

use kjxlkj_core_text::BufferContent;

use crate::cursor::CursorPosition;

pub(crate) fn normalize_range(
    a: CursorPosition,
    b: CursorPosition,
) -> (CursorPosition, CursorPosition) {
    if a.line < b.line || (a.line == b.line && a.grapheme_offset <= b.grapheme_offset) {
        (a, b)
    } else {
        (b, a)
    }
}

pub(crate) fn extract_text(
    content: &BufferContent,
    start: &CursorPosition,
    end: &CursorPosition,
    linewise: bool,
) -> String {
    if linewise {
        let mut result = String::new();
        for line in start.line..=end.line.min(content.line_count() - 1) {
            result.push_str(&content.line_str(line));
        }
        result
    } else if start.line == end.line {
        let line = content.line_content(start.line);
        let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
        let mut result = String::new();
        for i in start.grapheme_offset..=end.grapheme_offset {
            if let Some(g) = lg.get(i) {
                result.push_str(g);
            }
        }
        result
    } else {
        let mut result = String::new();
        let first = content.line_str(start.line);
        let lg = kjxlkj_core_text::LineGraphemes::from_str(&first);
        for i in start.grapheme_offset..lg.count() {
            if let Some(g) = lg.get(i) {
                result.push_str(g);
            }
        }
        result.push('\n');
        for line in (start.line + 1)..end.line {
            result.push_str(&content.line_str(line));
        }
        if end.line > start.line {
            let last = content.line_content(end.line);
            let lg = kjxlkj_core_text::LineGraphemes::from_str(&last);
            for i in 0..=end.grapheme_offset {
                if let Some(g) = lg.get(i) {
                    result.push_str(g);
                }
            }
        }
        result
    }
}

pub(crate) fn strip_indent(s: &str) -> String {
    if s.starts_with("    ") {
        s[4..].to_string()
    } else if s.starts_with('\t') {
        s[1..].to_string()
    } else {
        let trimmed = s.trim_start();
        trimmed.to_string()
    }
}

pub(crate) fn replace_line_content(content: &mut BufferContent, line: usize, new_content: &str) {
    let old_lg = content.line_graphemes(line);
    let gc = old_lg.count();
    if gc > 0 {
        content.delete(line, 0, line, gc);
    }
    content.insert(line, 0, new_content);
}
