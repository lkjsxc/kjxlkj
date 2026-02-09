/// Line yank, delete-lines, put-after, put-before operations.
use kjxlkj_core_types::CursorPosition;

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn delete_lines(&mut self, count: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            let start_line = cursor.line;
            let end_line = (start_line + count).min(buf.content.len_lines());
            let start_byte = buf.content.line_to_byte(start_line);
            let end_byte = if end_line >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end_line)
            };
            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
                buf.content.remove(start_char..end_char);
                buf.increment_version();
            }
        }
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub(crate) fn yank_lines(&mut self, count: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get(buf_id) {
            let start_line = cursor.line;
            let end_line = (start_line + count).min(buf.content.len_lines());
            let start_byte = buf.content.line_to_byte(start_line);
            let end_byte = if end_line >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end_line)
            };
            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
            }
        }
    }

    pub(crate) fn put_after(&mut self) {
        if let Some(reg) = self.registers.get_unnamed().cloned() {
            if reg.linewise {
                let cursor = self.windows.focused().cursor;
                let buf_id = self.current_buffer_id();
                if let Some(buf) = self.buffers.get_mut(buf_id) {
                    buf.save_undo_checkpoint(cursor);
                    let next_line = cursor.line + 1;
                    let byte_pos = if next_line < buf.content.len_lines() {
                        buf.content.line_to_byte(next_line)
                    } else {
                        buf.content.len_bytes()
                    };
                    let char_idx = buf.content.byte_to_char(byte_pos);
                    let text =
                        if byte_pos == buf.content.len_bytes() && !reg.content.starts_with('\n') {
                            format!("\n{}", reg.content.trim_end_matches('\n'))
                        } else {
                            reg.content.clone()
                        };
                    buf.content.insert(char_idx, &text);
                    buf.increment_version();
                }
                self.windows.focused_mut().cursor = CursorPosition::new(cursor.line + 1, 0);
            } else {
                self.move_cursor_right(1);
                self.insert_text(&reg.content);
            }
        }
    }

    pub(crate) fn put_before(&mut self) {
        if let Some(reg) = self.registers.get_unnamed().cloned() {
            if reg.linewise {
                let cursor = self.windows.focused().cursor;
                let buf_id = self.current_buffer_id();
                if let Some(buf) = self.buffers.get_mut(buf_id) {
                    buf.save_undo_checkpoint(cursor);
                    let byte_pos = buf.content.line_to_byte(cursor.line);
                    let char_idx = buf.content.byte_to_char(byte_pos);
                    buf.content.insert(char_idx, &reg.content);
                    buf.increment_version();
                }
            } else {
                self.insert_text(&reg.content);
            }
        }
    }
}
