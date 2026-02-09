//! Search navigation and mark/register actions.

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn set_mark_at_cursor(&mut self, name: char) {
        let cursor = self.windows.focused().cursor;
        let buf_id = self.current_buffer_id();
        let pos = crate::marks::MarkPosition::new(buf_id.0 as usize, cursor.line, cursor.grapheme);
        self.marks.set(name, pos);
    }

    pub(crate) fn jump_to_mark(&mut self, name: char) {
        let buf_id = self.current_buffer_id();
        if let Some(pos) = self.marks.get(name, buf_id.0 as usize).copied() {
            let cur = self.windows.focused().cursor;
            self.marks.push_mark_stack(crate::marks::MarkPosition::new(buf_id.0 as usize, cur.line, cur.grapheme));
            self.push_jumplist();
            if pos.buffer_id != buf_id.0 as usize {
                self.switch_to_buffer_id(pos.buffer_id);
            }
            self.windows.focused_mut().cursor =
                kjxlkj_core_types::CursorPosition::new(pos.line, pos.col);
            self.clamp_cursor();
            self.ensure_cursor_visible();
        }
    }

    pub(crate) fn jump_to_mark_line(&mut self, name: char) {
        let buf_id = self.current_buffer_id();
        if let Some(pos) = self.marks.get(name, buf_id.0 as usize).copied() {
            let cur = self.windows.focused().cursor;
            self.marks.push_mark_stack(crate::marks::MarkPosition::new(buf_id.0 as usize, cur.line, cur.grapheme));
            self.push_jumplist();
            if pos.buffer_id != buf_id.0 as usize {
                self.switch_to_buffer_id(pos.buffer_id);
            }
            self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(pos.line, 0);
            self.move_to_first_non_blank();
            self.ensure_cursor_visible();
        }
    }
    /// Pop mark stack and jump (g'/g`).
    pub(crate) fn jump_from_mark_stack(&mut self) {
        if let Some(pos) = self.marks.pop_mark_stack() {
            if pos.buffer_id != self.current_buffer_id().0 as usize { self.switch_to_buffer_id(pos.buffer_id); }
            self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(pos.line, pos.col);
            self.clamp_cursor(); self.ensure_cursor_visible();
        }
    }

    pub(crate) fn search_next(&mut self) {
        let pattern = match &self.search.pattern { Some(p) if !p.is_empty() => p.clone(), _ => return };
        self.push_jumplist();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get(buf_id) {
            let start = cursor.grapheme + 1;
            let text: String = buf.content.to_string();
            let sf = self.line_col_to_byte_offset(&text, cursor.line, start);
            if let Some((abs, len)) = find_pattern(&text, sf, &pattern, true) {
                let (l, c) = self.byte_offset_to_line_col(&text, abs);
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                self.apply_search_offset(&text, abs, len); self.ensure_cursor_visible();
            } else if let Some((off, len)) = find_pattern(&text, 0, &pattern, true) {
                if off < sf {
                    let (l, c) = self.byte_offset_to_line_col(&text, off);
                    self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                    self.apply_search_offset(&text, off, len); self.ensure_cursor_visible();
                }
            }
        }
        self.update_search_count();
    }

    pub(crate) fn search_prev(&mut self) {
        let pattern = match &self.search.pattern { Some(p) if !p.is_empty() => p.clone(), _ => return };
        self.push_jumplist();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get(buf_id) {
            let text: String = buf.content.to_string();
            let cur = self.line_col_to_byte_offset(&text, cursor.line, cursor.grapheme);
            if let Some((off, len)) = rfind_pattern(&text, cur, &pattern) {
                let (l, c) = self.byte_offset_to_line_col(&text, off);
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                self.apply_search_offset(&text, off, len); self.ensure_cursor_visible();
            } else if let Some((off, len)) = rfind_pattern(&text, text.len(), &pattern) {
                if off >= cur {
                    let (l, c) = self.byte_offset_to_line_col(&text, off);
                    self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
                    self.apply_search_offset(&text, off, len); self.ensure_cursor_visible();
                }
            }
        }
        self.update_search_count();
    }

    #[rustfmt::skip]
    fn line_col_to_byte_offset(&self, text: &str, line: usize, col: usize) -> usize {
        let mut offset = 0;
        for (i, l) in text.split('\n').enumerate() {
            if i == line { return offset + col.min(l.len()); }
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

    /// Apply search offset (e/s/+N) to cursor after finding match.
    #[rustfmt::skip]
    fn apply_search_offset(&mut self, text: &str, ms: usize, ml: usize) {
        use crate::search_types::SearchOffset;
        match self.search.offset {
            SearchOffset::None => {}
            SearchOffset::End(n) => {
                let t = ((ms + ml).saturating_sub(1) as i64 + n as i64).max(0) as usize;
                let (l, c) = self.byte_offset_to_line_col(text, t.min(text.len().saturating_sub(1)));
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
            }
            SearchOffset::Start(n) => {
                let t = (ms as i64 + n as i64).max(0) as usize;
                let (l, c) = self.byte_offset_to_line_col(text, t.min(text.len().saturating_sub(1)));
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(l, c);
            }
            SearchOffset::Lines(n) => {
                let nl = (self.windows.focused().cursor.line as i64 + n as i64).max(0) as usize;
                self.windows.focused_mut().cursor = kjxlkj_core_types::CursorPosition::new(nl, 0);
                self.clamp_cursor();
            }
        }
    }
}
#[rustfmt::skip]
fn strip_magic_prefix(pattern: &str) -> (&str, char) {
    for (pfx, mode) in [("\\v", 'v'), ("\\V", 'V'), ("\\M", 'M'), ("\\m", 'm')] {
        if let Some(rest) = pattern.strip_prefix(pfx) { return (rest, mode); }
    }
    (pattern, 'm')
}
fn nomagic_to_literal(pat: &str) -> String {
    pat.chars().fold(String::new(), |mut out, ch| {
        if ".+*?{}[]()\\|".contains(ch) && ch != '^' && ch != '$' { out.push('\\'); }
        out.push(ch); out
    })
}

/// Find pattern forward. Returns (offset, len).
#[rustfmt::skip]
fn find_pattern(text: &str, from: usize, pattern: &str, _fwd: bool) -> Option<(usize, usize)> {
    let (pat, mode) = strip_magic_prefix(pattern);
    match mode {
        'v' => regex_find_fwd(text, from, pat),
        'M' => regex_find_fwd(text, from, &nomagic_to_literal(pat)),
        'V' => text[from..].find(pat).map(|o| (from + o, pat.len())),
        _ if pat.contains('\\') => { let tr = crate::regex_translate::translate_vim_to_rust(pat); regex_find_fwd(text, from, &tr.pattern) }
        _ => text[from..].find(pat).map(|o| (from + o, pat.len())),
    }
}

/// Find pattern backward (last match before `before`).
#[rustfmt::skip]
fn rfind_pattern(text: &str, before: usize, pattern: &str) -> Option<(usize, usize)> {
    let (pat, mode) = strip_magic_prefix(pattern);
    let slice = &text[..before.min(text.len())];
    match mode {
        'v' => regex_find_last(slice, pat),
        'M' => regex_find_last(slice, &nomagic_to_literal(pat)),
        'V' => slice.rfind(pat).map(|o| (o, pat.len())),
        _ if pat.contains('\\') => { let tr = crate::regex_translate::translate_vim_to_rust(pat); regex_find_last(slice, &tr.pattern) }
        _ => slice.rfind(pat).map(|o| (o, pat.len())),
    }
}

#[rustfmt::skip]
fn regex_find_fwd(text: &str, from: usize, pat: &str) -> Option<(usize, usize)> {
    regex::Regex::new(pat).ok().and_then(|re| re.find(&text[from..]).map(|m| (from + m.start(), m.len())))
}
#[rustfmt::skip]
fn regex_find_last(slice: &str, pat: &str) -> Option<(usize, usize)> {
    regex::Regex::new(pat).ok().and_then(|re| { let mut l = None; for m in re.find_iter(slice) { l = Some((m.start(), m.len())); } l })
}
