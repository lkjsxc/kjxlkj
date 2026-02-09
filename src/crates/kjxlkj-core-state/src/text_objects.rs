//! Text object resolution: iw, aw, ip, ap, is, as, it, at, i{delim}, a{delim}.
use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

/// Text object kind.
#[derive(Debug, Clone, Copy)]
pub enum TextObjectKind {
    Inner,
    Around,
}

/// Resolved text object range.
#[derive(Debug, Clone)]
pub struct TextObjectRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
    pub linewise: bool,
}

/// Resolve a text object at the cursor position.
pub fn resolve_text_object(
    kind: TextObjectKind,
    obj: char,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    match obj {
        'w' => resolve_word(kind, pos, rope, false),
        'W' => resolve_word(kind, pos, rope, true),
        'p' => resolve_paragraph(kind, pos, rope),
        's' => crate::text_objects_sentence::resolve_sentence(kind, pos, rope),
        't' => crate::text_objects_tag::resolve_tag_object(kind, pos, rope),
        '(' | ')' | 'b' | '[' | ']' | '{' | '}' | 'B' | '<' | '>' | '"' | '\'' | '`' => {
            crate::text_objects_delim::resolve_delim_object(kind, obj, pos, rope)
        }
        _ => None,
    }
}

fn resolve_word(
    kind: TextObjectKind,
    pos: CursorPosition,
    rope: &Rope,
    bigword: bool,
) -> Option<TextObjectRange> {
    if pos.line >= rope.len_lines() {
        return None;
    }
    let line_s: String = rope.line(pos.line).chars().collect();
    let chars: Vec<char> = line_s.chars().collect();
    let g = pos.grapheme.min(chars.len().saturating_sub(1));
    if chars.is_empty() {
        return None;
    }
    let is_word_char = |c: char| -> bool {
        if bigword {
            !c.is_whitespace()
        } else {
            c.is_alphanumeric() || c == '_'
        }
    };
    let cur = chars[g];
    // Find word boundary.
    let (mut ws, mut we) = (g, g);
    if is_word_char(cur) {
        while ws > 0 && is_word_char(chars[ws - 1]) {
            ws -= 1;
        }
        while we + 1 < chars.len() && is_word_char(chars[we + 1]) {
            we += 1;
        }
    } else if !cur.is_whitespace() {
        // Punctuation word
        while ws > 0 && !chars[ws - 1].is_whitespace() && !is_word_char(chars[ws - 1]) {
            ws -= 1;
        }
        while we + 1 < chars.len() && !chars[we + 1].is_whitespace() && !is_word_char(chars[we + 1])
        {
            we += 1;
        }
    } else {
        // Whitespace
        while ws > 0 && chars[ws - 1].is_whitespace() && chars[ws - 1] != '\n' {
            ws -= 1;
        }
        while we + 1 < chars.len() && chars[we + 1].is_whitespace() && chars[we + 1] != '\n' {
            we += 1;
        }
    }
    // Around: include trailing whitespace.
    if matches!(kind, TextObjectKind::Around) {
        while we + 1 < chars.len() && chars[we + 1] == ' ' {
            we += 1;
        }
    }
    Some(TextObjectRange {
        start: CursorPosition::new(pos.line, ws),
        end: CursorPosition::new(pos.line, we),
        linewise: false,
    })
}

fn resolve_paragraph(
    kind: TextObjectKind,
    pos: CursorPosition,
    rope: &Rope,
) -> Option<TextObjectRange> {
    let total = rope.len_lines();
    if total == 0 {
        return None;
    }
    let is_blank = |l: usize| -> bool {
        let s: String = rope.line(l).chars().collect();
        s.trim().is_empty()
    };
    let cur_blank = is_blank(pos.line);
    // Find paragraph start.
    let mut start = pos.line;
    if cur_blank {
        while start > 0 && is_blank(start - 1) {
            start -= 1;
        }
    } else {
        while start > 0 && !is_blank(start - 1) {
            start -= 1;
        }
    }
    // Find paragraph end.
    let mut end = pos.line;
    if cur_blank {
        while end + 1 < total && is_blank(end + 1) {
            end += 1;
        }
    } else {
        while end + 1 < total && !is_blank(end + 1) {
            end += 1;
        }
    }
    // Around: include trailing blank lines.
    if matches!(kind, TextObjectKind::Around) {
        while end + 1 < total && is_blank(end + 1) {
            end += 1;
        }
    }
    let last_col = {
        let s: String = rope.line(end).chars().collect();
        s.len().saturating_sub(1)
    };
    Some(TextObjectRange {
        start: CursorPosition::new(start, 0),
        end: CursorPosition::new(end, last_col),
        linewise: true,
    })
}

/// Apply a text object operator from op-pending mode.
pub fn apply_text_object(
    state: &mut crate::editor::EditorState,
    op: kjxlkj_core_types::Operator,
    prefix: char,
    obj: char,
) {
    let kind = match prefix {
        'i' => TextObjectKind::Inner,
        _ => TextObjectKind::Around,
    };
    let buf_id = state.current_buffer_id();
    let cursor = state.windows.focused().cursor;
    let range = {
        let buf = match state.buffers.get(buf_id) {
            Some(b) => b,
            None => return,
        };
        resolve_text_object(kind, obj, cursor, &buf.content)
    };
    if let Some(range) = range {
        if range.linewise {
            let count = range.end.line - range.start.line + 1;
            let saved = state.windows.focused().cursor;
            state.windows.focused_mut().cursor.line = range.start.line;
            match op {
                kjxlkj_core_types::Operator::Delete => {
                    state.delete_lines(count);
                }
                kjxlkj_core_types::Operator::Yank => {
                    state.yank_lines(count);
                    state.windows.focused_mut().cursor = saved;
                }
                kjxlkj_core_types::Operator::Change => {
                    state.delete_lines(count);
                    state.open_above_impl();
                    state.enter_insert();
                }
                _ => {}
            }
        } else {
            state.apply_charwise_op(op, range.start, range.end, true);
        }
    }
}
