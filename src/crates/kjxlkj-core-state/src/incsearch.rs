/// Incremental search: highlight first match while typing.
use crate::editor::EditorState;

impl EditorState {
    /// Update incremental search highlight based on current cmdline content.
    pub(crate) fn update_incsearch(&mut self) {
        if !self.options.get_bool("incsearch") {
            return;
        }
        let pat = &self.cmdline.content;
        if pat.is_empty() {
            self.search.highlight_ranges.clear();
            return;
        }
        let buf_id = self.current_buffer_id();
        let buf = match self.buffers.get(buf_id) {
            Some(b) => b,
            None => return,
        };
        // Find first match from cursor position forward.
        let cursor = self.windows.focused().cursor;
        let text: String = buf.content.to_string();
        let start_off = self.inc_line_col_to_byte(&text, cursor.line, cursor.grapheme);
        // Try from cursor forward, then wrap.
        let found = text[start_off..]
            .find(pat.as_str())
            .map(|o| start_off + o)
            .or_else(|| text[..start_off].find(pat.as_str()));
        self.search.highlight_ranges.clear();
        if let Some(off) = found {
            let (line, col) = self.inc_byte_to_line_col(&text, off);
            self.search
                .highlight_ranges
                .push((line, col, col + pat.len()));
        }
    }

    fn inc_line_col_to_byte(&self, text: &str, line: usize, col: usize) -> usize {
        let mut off = 0;
        for (i, l) in text.split('\n').enumerate() {
            if i == line {
                return off + col.min(l.len());
            }
            off += l.len() + 1;
        }
        text.len()
    }

    fn inc_byte_to_line_col(&self, text: &str, offset: usize) -> (usize, usize) {
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

    /// Compute search match count [current/total] and store in search state.
    pub(crate) fn update_search_count(&mut self) {
        let pattern = match &self.search.pattern {
            Some(p) if !p.is_empty() => p.clone(),
            _ => { self.search.match_count = None; return; }
        };
        let buf_id = self.current_buffer_id();
        let buf = match self.buffers.get(buf_id) {
            Some(b) => b,
            None => return,
        };
        let text: String = buf.content.to_string();
        let cursor = self.windows.focused().cursor;
        let cursor_off = self.inc_line_col_to_byte(&text, cursor.line, cursor.grapheme);
        let mut total = 0usize;
        let mut current = 0usize;
        let mut pos = 0;
        while let Some(idx) = text[pos..].find(&*pattern) {
            let abs = pos + idx;
            total += 1;
            if abs <= cursor_off { current = total; }
            pos = abs + pattern.len().max(1);
        }
        if total > 0 {
            self.search.match_count = Some((current, total));
        } else {
            self.search.match_count = None;
        }
    }
}
