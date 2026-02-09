//! Format (gq) operator: join and re-wrap lines.
//!
//! Basic implementation that joins a range of lines
//! into a single paragraph and wraps at `textwidth`.

use crate::editor::EditorState;

impl EditorState {
    /// Format (re-wrap) a range of lines at textwidth.
    pub(crate) fn format_lines(&mut self, start: usize, end: usize) {
        let tw = self.get_textwidth();
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let total = buf.content.len_lines();
            let end = end.min(total.saturating_sub(1));
            if start > end {
                return;
            }
            // Collect lines.
            let mut words: Vec<String> = Vec::new();
            for line_idx in start..=end {
                if line_idx >= buf.content.len_lines() {
                    break;
                }
                let ls = buf.content.line(line_idx);
                let s: std::borrow::Cow<str> = ls.into();
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    // Paragraph break: push empty sentinel.
                    words.push(String::new());
                } else {
                    for w in trimmed.split_whitespace() {
                        words.push(w.to_string());
                    }
                }
            }
            // Rebuild wrapped text.
            let wrapped = wrap_words(&words, tw);
            // Remove old lines and insert wrapped text.
            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end + 1 < buf.content.len_lines() {
                buf.content.line_to_byte(end + 1)
            } else {
                buf.content.len_bytes()
            };
            let sc = buf.content.byte_to_char(start_byte);
            let ec = buf.content.byte_to_char(end_byte);
            buf.content.remove(sc..ec);
            buf.content.insert(sc, &wrapped);
            buf.increment_version();
        }
        self.clamp_cursor();
    }

    fn get_textwidth(&self) -> usize {
        let tw = self.options.get_int("textwidth");
        if tw >= 10 {
            tw
        } else {
            79
        }
    }
}

fn wrap_words(words: &[String], width: usize) -> String {
    let mut result = String::new();
    let mut col = 0usize;
    for w in words {
        if w.is_empty() {
            // Paragraph break.
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push('\n');
            col = 0;
            continue;
        }
        if col == 0 {
            result.push_str(w);
            col = w.len();
        } else if col + 1 + w.len() > width {
            result.push('\n');
            result.push_str(w);
            col = w.len();
        } else {
            result.push(' ');
            result.push_str(w);
            col += 1 + w.len();
        }
    }
    if col > 0 {
        result.push('\n');
    }
    result
}
