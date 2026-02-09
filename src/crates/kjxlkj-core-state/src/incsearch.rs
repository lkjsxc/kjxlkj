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
}
