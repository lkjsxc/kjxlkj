//! Star search and sentence motion execution.

use kjxlkj_core_text::BufferContent;

use crate::cursor::CursorPosition;

pub(crate) fn exec_star_search(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    forward: bool,
) {
    let line = content.line_content(cursor.line);
    let chars: Vec<char> = line.chars().collect();
    if cursor.grapheme_offset >= chars.len() {
        return;
    }
    let mut start = cursor.grapheme_offset;
    let mut end = cursor.grapheme_offset;
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }
    if start == end {
        return;
    }
    let word: String = chars[start..end].iter().collect();
    let total = content.line_count();
    if forward {
        for off in 1..=(total * 2) {
            let l = (cursor.line + off) % total;
            let ln = content.line_content(l);
            if let Some(pos) = ln.find(&word) {
                cursor.line = l;
                cursor.grapheme_offset = ln[..pos].chars().count();
                cursor.clear_desired_col();
                return;
            }
        }
    } else {
        for off in 1..=(total * 2) {
            let l = (cursor.line + total - off % total) % total;
            let ln = content.line_content(l);
            if let Some(pos) = ln.rfind(&word) {
                cursor.line = l;
                cursor.grapheme_offset = ln[..pos].chars().count();
                cursor.clear_desired_col();
                return;
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub(crate) fn move_sentence_forward(cursor: &mut CursorPosition, content: &BufferContent) {
    let lc = content.line_count();
    let mut l = cursor.line;
    let mut c = cursor.grapheme_offset + 1;
    while l < lc {
        let line = content.line_content(l);
        let chars: Vec<char> = line.chars().collect();
        while c < chars.len() {
            if is_sentence_end(chars[c]) && (c + 1 >= chars.len() || chars[c + 1].is_whitespace()) {
                cursor.line = l;
                cursor.grapheme_offset = c + 1;
                if cursor.grapheme_offset >= chars.len() && l + 1 < lc {
                    cursor.line = l + 1;
                    cursor.grapheme_offset = 0;
                }
                return;
            }
            c += 1;
        }
        l += 1;
        c = 0;
    }
    cursor.line = lc.saturating_sub(1);
    cursor.grapheme_offset = 0;
}

pub(crate) fn move_sentence_backward(cursor: &mut CursorPosition, content: &BufferContent) {
    if cursor.line == 0 && cursor.grapheme_offset == 0 {
        return;
    }
    let mut l = cursor.line;
    let mut c = if cursor.grapheme_offset > 0 {
        cursor.grapheme_offset - 1
    } else {
        if l == 0 {
            return;
        }
        l -= 1;
        let prev = content.line_content(l);
        prev.chars().count().saturating_sub(1)
    };
    loop {
        let line = content.line_content(l);
        let chars: Vec<char> = line.chars().collect();
        while c > 0 {
            c -= 1;
            if is_sentence_end(chars[c]) && c + 1 < chars.len() && chars[c + 1].is_whitespace() {
                cursor.line = l;
                cursor.grapheme_offset = c + 2;
                return;
            }
        }
        if l == 0 {
            cursor.line = 0;
            cursor.grapheme_offset = 0;
            return;
        }
        l -= 1;
        let prev = content.line_content(l);
        c = prev.chars().count().saturating_sub(1);
    }
}

fn is_sentence_end(c: char) -> bool {
    c == '.' || c == '!' || c == '?'
}
