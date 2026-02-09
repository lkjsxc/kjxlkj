//! Extended motion helpers: find-char and bracket
//! matching.

use kjxlkj_core_text::BufferContent;

use crate::cursor::CursorPosition;

pub(crate) fn exec_find_char(
    cursor: &mut CursorPosition,
    content: &BufferContent,
    ch: char,
    forward: bool,
    till: bool,
) {
    let line = content.line_content(cursor.line);
    let kind = if forward {
        if till {
            crate::CharFindKind::ForwardTill
        } else {
            crate::CharFindKind::ForwardTo
        }
    } else if till {
        crate::CharFindKind::BackwardTill
    } else {
        crate::CharFindKind::BackwardTo
    };
    let find = crate::CharFind::new(ch, kind);
    if let Some(idx) = find.execute(&line, cursor.grapheme_offset) {
        cursor.grapheme_offset = idx;
    }
    cursor.clear_desired_col();
}

pub(crate) fn exec_matching_bracket(cursor: &mut CursorPosition, content: &BufferContent) {
    let line = content.line_content(cursor.line);
    let chars: Vec<char> = line.chars().collect();
    let idx = cursor.grapheme_offset;
    if idx >= chars.len() {
        return;
    }
    let ch = chars[idx];
    let (target, forward) = match ch {
        '(' => (')', true),
        ')' => ('(', false),
        '[' => (']', true),
        ']' => ('[', false),
        '{' => ('}', true),
        '}' => ('{', false),
        _ => return,
    };
    let mut depth: i32 = 1;
    let total = content.line_count();
    let mut l = cursor.line;
    let mut c = idx;
    loop {
        if forward {
            c += 1;
        } else if c == 0 {
            if l == 0 {
                return;
            }
            l -= 1;
            let nl = content.line_content(l);
            c = nl.chars().count();
            if c == 0 {
                continue;
            }
            c -= 1;
        } else {
            c -= 1;
        }
        if forward {
            let llen = content.line_content(l).chars().count();
            if c >= llen {
                l += 1;
                if l >= total {
                    return;
                }
                c = 0;
            }
        }
        let cur = content.line_content(l);
        let cv: Vec<char> = cur.chars().collect();
        if c >= cv.len() {
            continue;
        }
        if cv[c] == ch {
            depth += 1;
        } else if cv[c] == target {
            depth -= 1;
            if depth == 0 {
                cursor.line = l;
                cursor.grapheme_offset = c;
                cursor.clear_desired_col();
                return;
            }
        }
    }
}
