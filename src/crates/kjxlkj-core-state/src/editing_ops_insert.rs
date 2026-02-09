/// Insert-mode entry and text insertion operations.
use kjxlkj_core_text::RopeExt;
use kjxlkj_core_types::{CursorPosition, Mode};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn enter_insert(&mut self) {
        self.mode = Mode::Insert;
        let cursor = self.windows.focused().cursor;
        let version = self.buffers.current().version;
        let content = self.buffers.current().content.clone();
        self.buffers.current_mut().undo_tree.begin_group(
            version,
            content,
            cursor.line,
            cursor.grapheme,
        );
    }

    pub(crate) fn enter_insert_after(&mut self) {
        self.move_cursor_right(1);
        self.enter_insert();
    }

    pub(crate) fn enter_insert_line_start(&mut self) {
        self.move_to_first_non_blank();
        self.enter_insert();
    }

    pub(crate) fn enter_insert_line_end(&mut self) {
        self.move_to_line_end_insert();
        self.enter_insert();
    }

    pub(crate) fn open_below(&mut self) {
        self.open_below_impl();
        self.enter_insert();
    }

    pub(crate) fn open_above(&mut self) {
        self.open_above_impl();
        self.enter_insert();
    }

    pub(crate) fn open_below_impl(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let line = cursor.line;
            let line_end_byte = if line + 1 < buf.content.len_lines() {
                buf.content.line_to_byte(line + 1)
            } else {
                buf.content.len_bytes()
            };
            let char_idx = buf.content.byte_to_char(line_end_byte);
            if line_end_byte == buf.content.len_bytes() {
                buf.content.insert(char_idx, "\n");
            } else {
                buf.content.insert(
                    buf.content.byte_to_char(buf.content.line_to_byte(line + 1)),
                    "\n",
                );
            }
            buf.increment_version();
        }
        self.windows.focused_mut().cursor = CursorPosition::new(cursor.line + 1, 0);
    }

    pub(crate) fn open_above_impl(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let line_start = buf.content.line_to_byte(cursor.line);
            let char_idx = buf.content.byte_to_char(line_start);
            buf.content.insert(char_idx, "\n");
            buf.increment_version();
        }
        self.windows.focused_mut().cursor = CursorPosition::new(cursor.line, 0);
    }

    pub(crate) fn insert_char(&mut self, c: char) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.content
                .insert_at_grapheme(cursor.line, cursor.grapheme, &c.to_string());
            buf.increment_version();
        }
        self.windows.focused_mut().cursor.grapheme += 1;
    }

    pub(crate) fn insert_newline(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.content
                .insert_at_grapheme(cursor.line, cursor.grapheme, "\n");
            buf.increment_version();
        }
        let new_cursor = CursorPosition::new(cursor.line + 1, 0);
        self.windows.focused_mut().cursor = new_cursor;
        self.ensure_cursor_visible();
    }

    pub(crate) fn insert_text(&mut self, text: &str) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let was_insert = matches!(self.mode, Mode::Insert);
            if !was_insert {
                buf.save_undo_checkpoint(cursor);
            }
            buf.content
                .insert_at_grapheme(cursor.line, cursor.grapheme, text);
            buf.increment_version();
        }
        let lines: Vec<&str> = text.split('\n').collect();
        if lines.len() > 1 {
            let new_line = cursor.line + lines.len() - 1;
            let last_len = lines
                .last()
                .map(|l| unicode_segmentation::UnicodeSegmentation::graphemes(*l, true).count())
                .unwrap_or(0);
            self.windows.focused_mut().cursor = CursorPosition::new(new_line, last_len);
        } else {
            let char_count =
                unicode_segmentation::UnicodeSegmentation::graphemes(text, true).count();
            self.windows.focused_mut().cursor.grapheme += char_count;
        }
    }

    pub(crate) fn replace_char(&mut self, c: char) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            buf.content.delete_grapheme_range(
                cursor.line,
                cursor.grapheme,
                cursor.line,
                cursor.grapheme + 1,
            );
            buf.content
                .insert_at_grapheme(cursor.line, cursor.grapheme, &c.to_string());
            buf.increment_version();
        }
    }
}
