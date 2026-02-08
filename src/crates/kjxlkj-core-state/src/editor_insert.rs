//! Insert mode editing helpers for EditorState.

use kjxlkj_core_types::Mode;

use crate::EditorState;

impl EditorState {
    /// Insert a character in insert mode.
    pub(crate) fn insert_char(&mut self, ch: char) {
        if ch == '\n' {
            self.insert_newline_impl();
            return;
        }
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            buf.content.insert_char(off, ch);
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
    }

    fn insert_newline_impl(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            buf.content.insert_char(off, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line += 1;
            w.cursor.grapheme_offset = 0;
        }
    }

    /// Backspace in insert mode.
    pub(crate) fn do_backspace(&mut self) {
        let (line, col) = self.cursor_pos();
        if col == 0 && line == 0 {
            return;
        }
        let prev_gc = if col == 0 && line > 0 {
            self.active_buffer()
                .map(|b| {
                    b.content.line_grapheme_count(line - 1)
                })
                .unwrap_or(0)
        } else {
            0
        };
        if let Some(buf) = self.active_buffer_mut() {
            if col > 0 {
                let off = buf
                    .content
                    .line_grapheme_to_offset(
                        line,
                        col - 1,
                    );
                buf.content.delete_range(off, off + 1);
                buf.modified = true;
            } else {
                let off =
                    buf.content.line_start_offset(line);
                if off > 0 {
                    buf.content
                        .delete_range(off - 1, off);
                    buf.modified = true;
                }
            }
        }
        if let Some(w) = self.focused_window_mut() {
            if col > 0 {
                w.cursor.grapheme_offset -= 1;
            } else if line > 0 {
                w.cursor.line -= 1;
                w.cursor.grapheme_offset = prev_gc;
            }
        }
    }

    /// Delete character forward (`x`).
    pub(crate) fn delete_char_forward(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            if off < buf.content.len_chars() {
                buf.content.delete_range(off, off + 1);
                buf.modified = true;
            }
        }
    }

    /// Open a new line below current line.
    pub(crate) fn open_line_below(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            let end = buf.content.line_end_offset(line);
            buf.content.insert_char(end, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line += 1;
            w.cursor.grapheme_offset = 0;
        }
        self.mode = Mode::Insert;
        self.insert_state.reset();
    }

    /// Open a new line above current line.
    pub(crate) fn open_line_above(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            let start =
                buf.content.line_start_offset(line);
            buf.content.insert_char(start, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset = 0;
        }
        self.mode = Mode::Insert;
        self.insert_state.reset();
    }
}
