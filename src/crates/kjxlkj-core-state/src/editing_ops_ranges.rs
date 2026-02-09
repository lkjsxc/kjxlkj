//! Range-based editing operations: delete, yank, change
//! for charwise and linewise ranges.

use kjxlkj_core_types::{CursorPosition, Mode, Operator};

use crate::editor::EditorState;

impl EditorState {
    /// Apply an operator to a charwise range.
    pub(crate) fn apply_charwise_op(
        &mut self,
        op: Operator,
        start: CursorPosition,
        end: CursorPosition,
        inclusive: bool,
    ) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let end_g = if inclusive {
            end.grapheme + 1
        } else {
            end.grapheme
        };
        if start == end && !inclusive {
            return;
        }
        match op {
            Operator::Delete => {
                if let Some(buf) = self.buffers.get_mut(buf_id) {
                    buf.save_undo_checkpoint(cursor);
                    let text = self.extract_range(buf_id, start, end, inclusive);
                    self.store_register(text, false);
                }
                self.delete_range_raw(buf_id, start.line, start.grapheme, end.line, end_g);
                self.windows.focused_mut().cursor = start;
                self.clamp_cursor();
            }
            Operator::Yank => {
                let text = self.extract_range(buf_id, start, end, inclusive);
                self.store_register(text, false);
            }
            Operator::Change => {
                if let Some(buf) = self.buffers.get_mut(buf_id) {
                    buf.save_undo_checkpoint(cursor);
                    let text = self.extract_range(buf_id, start, end, inclusive);
                    self.store_register(text, false);
                }
                self.delete_range_raw(buf_id, start.line, start.grapheme, end.line, end_g);
                self.windows.focused_mut().cursor = start;
                self.mode = Mode::Insert;
            }
            Operator::Lowercase => {
                self.case_range(buf_id, start, end, inclusive, false);
            }
            Operator::Uppercase => {
                self.case_range(buf_id, start, end, inclusive, true);
            }
            _ => {}
        }
    }

    /// Apply an operator to a visual selection.
    pub(crate) fn apply_visual_op(
        &mut self,
        op: Operator,
        start: CursorPosition,
        end: CursorPosition,
        linewise: bool,
    ) {
        if linewise {
            self.apply_linewise_op_impl(op, start.line, end.line);
        } else {
            self.apply_charwise_op(op, start, end, true);
        }
    }

    fn apply_linewise_op_impl(&mut self, op: Operator, start: usize, end: usize) {
        let count = end - start + 1;
        let saved = self.windows.focused().cursor;
        self.windows.focused_mut().cursor.line = start;
        match op {
            Operator::Delete => self.delete_lines(count),
            Operator::Yank => {
                self.yank_lines(count);
                self.windows.focused_mut().cursor = saved;
            }
            Operator::Change => {
                self.delete_lines(count);
                self.open_above_impl();
                self.enter_insert();
            }
            Operator::Indent => self.indent_lines_range(start, end),
            Operator::Dedent => self.dedent_lines_range(start, end),
            Operator::Lowercase => self.lowercase_lines(start, end),
            Operator::Uppercase => self.uppercase_lines(start, end),
            _ => {}
        }
    }

    pub(crate) fn indent_lines_range(&mut self, start: usize, end: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            for line in start..=end {
                if line < buf.content.len_lines() {
                    let byte = buf.content.line_to_byte(line);
                    let ci = buf.content.byte_to_char(byte);
                    buf.content.insert(ci, "    ");
                }
            }
            buf.increment_version();
        }
    }

    pub(crate) fn dedent_lines_range(&mut self, start: usize, end: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            for line in start..=end {
                if line >= buf.content.len_lines() {
                    continue;
                }
                let ls = buf.content.line(line);
                let s: std::borrow::Cow<str> = ls.into();
                let spaces = s.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    let byte = buf.content.line_to_byte(line);
                    let sc = buf.content.byte_to_char(byte);
                    let ec = buf.content.byte_to_char(byte + spaces);
                    buf.content.remove(sc..ec);
                }
            }
            buf.increment_version();
        }
    }

    pub(crate) fn extract_range(
        &self,
        buf_id: kjxlkj_core_types::BufferId,
        start: CursorPosition,
        end: CursorPosition,
        inclusive: bool,
    ) -> String {
        if let Some(buf) = self.buffers.get(buf_id) {
            use kjxlkj_core_text::RopeExt;
            let end_g = if inclusive {
                end.grapheme + 1
            } else {
                end.grapheme
            };
            let sb = buf.content.grapheme_pos_to_byte(start.line, start.grapheme);
            let eb = buf.content.grapheme_pos_to_byte(end.line, end_g);
            if sb < eb {
                let sc = buf.content.byte_to_char(sb);
                let ec = buf.content.byte_to_char(eb);
                buf.content.slice(sc..ec).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    fn delete_range_raw(
        &mut self,
        buf_id: kjxlkj_core_types::BufferId,
        sl: usize,
        sg: usize,
        el: usize,
        eg: usize,
    ) {
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            use kjxlkj_core_text::RopeExt;
            buf.content.delete_grapheme_range(sl, sg, el, eg);
            buf.increment_version();
        }
    }
}
