/// Line yank, delete-lines, put-after, put-before operations.
use kjxlkj_core_edit::{Register, RegisterName};
use kjxlkj_core_types::CursorPosition;

use crate::editor::EditorState;

impl EditorState {
    /// Take pending register and clear it, or return None.
    #[allow(dead_code)]
    pub(crate) fn take_register(&mut self) -> Option<char> {
        self.pending_register.take()
    }

    /// Store content into the appropriate register.
    pub(crate) fn store_register(&mut self, content: String, linewise: bool) {
        if let Some(reg_name) = self.pending_register.take() {
            let name = RegisterName::Named(reg_name);
            self.registers.set(name, Register::new(content, linewise));
        } else {
            self.registers.set_unnamed(content, linewise);
        }
    }

    /// Store content, rotating numbered regs 1→9 on delete.
    pub(crate) fn store_register_delete(&mut self, content: String, linewise: bool) {
        if let Some(reg_name) = self.pending_register.take() {
            let name = RegisterName::Named(reg_name);
            self.registers.set(name, Register::new(content, linewise));
        } else {
            for i in (2..=9u8).rev() {
                let prev = RegisterName::Numbered(i - 1);
                if let Some(r) = self.registers.get(prev).cloned() {
                    self.registers.set(RegisterName::Numbered(i), r);
                }
            }
            self.registers.set(
                RegisterName::Numbered(1),
                Register::new(content.clone(), linewise),
            );
            self.registers.set_unnamed(content, linewise);
        }
    }

    /// Read from pending register or unnamed.
    fn read_register(&mut self) -> Option<Register> {
        let reg = self.pending_register.take();
        if let Some(reg_name) = reg {
            let name = RegisterName::Named(reg_name);
            self.registers.get(name).cloned()
        } else {
            self.registers.get_unnamed().cloned()
        }
    }

    pub(crate) fn delete_lines(&mut self, count: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let bid = buf_id.0 as usize;
        // Set change marks for lines being deleted.
        let sm = crate::marks::MarkPosition {
            buffer_id: bid,
            line: cursor.line,
            col: 0,
        };
        let end_l = cursor.line + count.saturating_sub(1);
        let em = crate::marks::MarkPosition {
            buffer_id: bid,
            line: end_l,
            col: 0,
        };
        self.marks.set_change_start(sm);
        self.marks.set_change_end(em);
        let yanked = if let Some(buf) = self.buffers.get_mut(buf_id) {
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
                let text = buf.content.slice(start_char..end_char).to_string();
                buf.content.remove(start_char..end_char);
                buf.increment_version();
                Some(text)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(text) = yanked {
            self.store_register_delete(text, true);
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
                self.store_register(yanked, true);
            }
        }
    }

    pub(crate) fn put_after(&mut self) {
        if let Some(reg) = self.read_register() {
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
        if let Some(reg) = self.read_register() {
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
