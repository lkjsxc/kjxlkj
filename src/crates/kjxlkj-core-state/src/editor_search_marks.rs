//! Search navigation and mark/register actions for EditorState.

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn set_mark_at_cursor(&mut self, name: char) {
        let cursor = self.windows.focused().cursor;
        let buf_id = self.current_buffer_id();
        let pos = crate::marks::MarkPosition {
            buffer_id: buf_id.0 as usize,
            line: cursor.line,
            col: cursor.grapheme,
        };
        self.marks.set(name, pos);
    }

    pub(crate) fn jump_to_mark(&mut self, name: char) {
        let buf_id = self.current_buffer_id();
        if let Some(pos) = self.marks.get(name, buf_id.0 as usize).copied() {
            self.push_jumplist();
            self.windows.focused_mut().cursor =
                kjxlkj_core_types::CursorPosition::new(pos.line, pos.col);
            self.clamp_cursor();
            self.ensure_cursor_visible();
        }
    }

    pub(crate) fn jump_to_mark_line(&mut self, name: char) {
        let buf_id = self.current_buffer_id();
        if let Some(pos) = self.marks.get(name, buf_id.0 as usize).copied() {
            self.push_jumplist();
            self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(pos.line, 0);
            self.move_to_first_non_blank();
            self.ensure_cursor_visible();
        }
    }

    pub(crate) fn search_next(&mut self) {
        let pattern = match &self.search.pattern {
            Some(p) if !p.is_empty() => p.clone(),
            _ => return,
        };
        self.push_jumplist();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get(buf_id) {
            let start = cursor.grapheme + 1;
            let text: String = buf.content.to_string();
            let sf = self.line_col_to_byte_offset(&text, cursor.line, start);
            if let Some((abs, _len)) = find_pattern(&text, sf, &pattern, true) {
                let (l, c) = self.byte_offset_to_line_col(&text, abs);
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                self.ensure_cursor_visible();
            } else if let Some((off, _len)) = find_pattern(&text, 0, &pattern, true) {
                if off < sf {
                    let (l, c) = self.byte_offset_to_line_col(&text, off);
                    self.windows.focused_mut().cursor =
                        kjxlkj_core_types::CursorPosition::new(l, c);
                    self.ensure_cursor_visible();
                }
            }
        }
    }

    pub(crate) fn search_prev(&mut self) {
        let pattern = match &self.search.pattern {
            Some(p) if !p.is_empty() => p.clone(),
            _ => return,
        };
        self.push_jumplist();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get(buf_id) {
            let text: String = buf.content.to_string();
            let cur = self.line_col_to_byte_offset(&text, cursor.line, cursor.grapheme);
            if let Some((off, _)) = rfind_pattern(&text, cur, &pattern) {
                let (l, c) = self.byte_offset_to_line_col(&text, off);
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                self.ensure_cursor_visible();
            } else if let Some((off, _)) = rfind_pattern(&text, text.len(), &pattern) {
                if off >= cur {
                    let (l, c) = self.byte_offset_to_line_col(&text, off);
                    self.windows.focused_mut().cursor =
                        kjxlkj_core_types::CursorPosition::new(l, c);
                    self.ensure_cursor_visible();
                }
            }
        }
    }

    fn line_col_to_byte_offset(&self, text: &str, line: usize, col: usize) -> usize {
        let mut offset = 0;
        for (i, l) in text.split('\n').enumerate() {
            if i == line {
                return offset + col.min(l.len());
            }
            offset += l.len() + 1;
        }
        text.len()
    }

    fn byte_offset_to_line_col(&self, text: &str, offset: usize) -> (usize, usize) {
        let mut line = 0;
        let mut line_start = 0;
        for (i, ch) in text.char_indices() {
            if i == offset {
                return (line, i - line_start);
            }
            if ch == '\n' {
                line += 1;
                line_start = i + 1;
            }
        }
        (line, offset.saturating_sub(line_start))
    }
}

/// Strip `\v` (very magic), `\m` (magic), `\M` (nomagic) prefix.
/// Returns (pattern, mode) where mode: 'v'=very-magic, 'M'=nomagic, 'm'=magic/literal.
fn strip_magic_prefix(pattern: &str) -> (&str, char) {
    if let Some(rest) = pattern.strip_prefix("\\v") {
        (rest, 'v')
    } else if let Some(rest) = pattern.strip_prefix("\\M") {
        (rest, 'M')
    } else if let Some(rest) = pattern.strip_prefix("\\m") {
        (rest, 'm')
    } else {
        (pattern, 'm')
    }
}

/// Escape a nomagic pattern: only ^ and $ are special.
fn nomagic_to_literal(pat: &str) -> String {
    let mut out = String::new();
    for ch in pat.chars() {
        if ch == '^' || ch == '$' {
            out.push(ch);
        } else if ".+*?{}[]()\\|".contains(ch) {
            out.push('\\');
            out.push(ch);
        } else {
            out.push(ch);
        }
    }
    out
}

/// Find pattern forward from `from` in `text`. Returns (offset, len).
fn find_pattern(text: &str, from: usize, pattern: &str, _fwd: bool) -> Option<(usize, usize)> {
    let (pat, mode) = strip_magic_prefix(pattern);
    match mode {
        'v' => {
            if let Ok(re) = regex::Regex::new(pat) {
                if let Some(m) = re.find(&text[from..]) {
                    return Some((from + m.start(), m.len()));
                }
            }
            None
        }
        'M' => {
            let escaped = nomagic_to_literal(pat);
            if let Ok(re) = regex::Regex::new(&escaped) {
                if let Some(m) = re.find(&text[from..]) {
                    return Some((from + m.start(), m.len()));
                }
            }
            None
        }
        _ => text[from..].find(pat).map(|off| (from + off, pat.len())),
    }
}

/// Find pattern backward (last match before `before`).
fn rfind_pattern(text: &str, before: usize, pattern: &str) -> Option<(usize, usize)> {
    let (pat, mode) = strip_magic_prefix(pattern);
    let slice = &text[..before.min(text.len())];
    match mode {
        'v' | 'M' => {
            let rpat = if mode == 'M' {
                nomagic_to_literal(pat)
            } else {
                pat.to_string()
            };
            if let Ok(re) = regex::Regex::new(&rpat) {
                let mut last = None;
                for m in re.find_iter(slice) {
                    last = Some((m.start(), m.len()));
                }
                return last;
            }
            None
        }
        _ => slice.rfind(pat).map(|off| (off, pat.len())),
    }
}
