//! Visual mode operations: selection extraction, deletion, operator dispatch.

use kjxlkj_core_edit::{CursorPosition, RegisterType};
use kjxlkj_core_text::grapheme::line_graphemes;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Mode;

use crate::editor::EditorState;

impl EditorState {
    /// Apply an operator on the visual selection range.
    pub(crate) fn do_visual_operator(&mut self, op: kjxlkj_core_types::Operator) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => {
                self.mode = Mode::Normal;
                return;
            }
        };
        self.save_undo_checkpoint();
        let reg_target = self.pending_register.take();
        let mut reg_content = String::new();
        self.with_active_buffer(|buf, cur| {
            let anchor_cur = CursorPosition::new(anchor.0, anchor.1);
            let (s, e) = if (anchor_cur.line, anchor_cur.grapheme_offset)
                <= (cur.line, cur.grapheme_offset)
            {
                (anchor_cur, *cur)
            } else {
                (*cur, anchor_cur)
            };
            reg_content = visual_extract(buf, &s, &e);
            visual_delete(buf, &s, &e);
            *cur = s;
        });
        if !reg_content.is_empty() {
            let is_lw = reg_content.ends_with('\n');
            let rtype = if is_lw {
                RegisterType::Linewise
            } else {
                RegisterType::Charwise
            };
            match op {
                kjxlkj_core_types::Operator::Yank => {
                    self.do_undo();
                    self.registers.store_yank(reg_target, reg_content, rtype);
                }
                kjxlkj_core_types::Operator::Delete => {
                    self.registers
                        .store_delete(reg_target, reg_content, rtype, is_lw);
                }
                kjxlkj_core_types::Operator::Change => {
                    self.registers
                        .store_delete(reg_target, reg_content, rtype, is_lw);
                    self.mode = Mode::Insert;
                    return;
                }
                _ => {}
            }
        }
        self.mode = Mode::Normal;
    }
}

/// Extract text from a visual (charwise) selection.
fn visual_extract(buf: &TextBuffer, start: &CursorPosition, end: &CursorPosition) -> String {
    let s_char = pos_to_char(buf, start);
    let e_char = pos_to_char(buf, end);
    let end_line = buf.line(end.line).unwrap_or_default();
    let end_gs = line_graphemes(&end_line);
    let end_g_chars = end_gs
        .get(end.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(1);
    let actual_end = (e_char + end_g_chars).min(buf.content.len_chars());
    buf.content.slice(s_char..actual_end).to_string()
}

/// Delete text in a visual (charwise) selection.
fn visual_delete(buf: &mut TextBuffer, start: &CursorPosition, end: &CursorPosition) {
    let s_char = pos_to_char(buf, start);
    let e_char = pos_to_char(buf, end);
    let end_line = buf.line(end.line).unwrap_or_default();
    let end_gs = line_graphemes(&end_line);
    let end_g_chars = end_gs
        .get(end.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .unwrap_or(1);
    let actual_end = (e_char + end_g_chars).min(buf.content.len_chars());
    buf.remove_char_range(s_char, actual_end);
}

/// Convert a CursorPosition to a char index in the buffer.
fn pos_to_char(buf: &TextBuffer, pos: &CursorPosition) -> usize {
    let line_start = buf.line_to_char(pos.line);
    let line_str = buf.line(pos.line).unwrap_or_default();
    let gs = line_graphemes(&line_str);
    let offset: usize = gs
        .iter()
        .take(pos.grapheme_offset)
        .map(|g: &&str| g.chars().count())
        .sum();
    line_start + offset
}
