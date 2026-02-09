//! Operator-pending mode dispatch.
//!
//! Parses a motion or doubled operator, computes the affected
//! range, and applies the pending operator.

use kjxlkj_core_edit::{resolve_motion, Motion, MotionKind};
use kjxlkj_core_types::{CursorPosition, Key, KeyCode, Mode, Modifier, Operator};

use crate::editor::EditorState;
use crate::op_pending_helpers::{is_doubled, key_to_motion};

fn ordered(a: CursorPosition, b: CursorPosition) -> (CursorPosition, CursorPosition) {
    if (a.line, a.grapheme) <= (b.line, b.grapheme) {
        (a, b)
    } else {
        (b, a)
    }
}

impl EditorState {
    /// Dispatch a key while in operator-pending mode.
    pub(crate) fn dispatch_op_pending(&mut self, key: Key, op: Operator) {
        // Text object prefix (i/a) or mark/findchar handling.
        if let Some(prefix) = self.text_obj_prefix.take() {
            if let KeyCode::Char(obj) = &key.code {
                match prefix {
                    'i' | 'a' => self.apply_text_object(op, prefix, *obj),
                    '\'' => {
                        let buf_id = self.current_buffer_id().0 as usize;
                        if let Some(pos) = self.marks.get(*obj, buf_id) {
                            let dest = CursorPosition::new(pos.line, 0);
                            let cursor = self.windows.focused().cursor;
                            let (s, e) = ordered(cursor, dest);
                            self.apply_linewise_op(op, s.line, e.line);
                        }
                    }
                    '`' => {
                        let buf_id = self.current_buffer_id().0 as usize;
                        if let Some(pos) = self.marks.get(*obj, buf_id) {
                            let dest = CursorPosition::new(pos.line, pos.col);
                            let cursor = self.windows.focused().cursor;
                            let (s, e) = ordered(cursor, dest);
                            self.apply_charwise_op(op, s, e, true);
                        }
                    }
                    'f' | 'F' | 't' | 'T' => {
                        let m = match prefix {
                            'f' => Motion::FindCharForward(*obj),
                            'F' => Motion::FindCharBackward(*obj),
                            't' => Motion::TillCharForward(*obj),
                            _ => Motion::TillCharBackward(*obj),
                        };
                        self.last_ft = Some((prefix, *obj));
                        self.apply_op_motion(op, &m, 1);
                    }
                    _ => {}
                }
                if !matches!(self.mode, Mode::Insert) {
                    self.mode = Mode::Normal;
                }
                return;
            }
            self.mode = Mode::Normal;
            return;
        }
        // Accumulate motion count digits.
        if let KeyCode::Char(c) = &key.code {
            if key.modifiers == Modifier::NONE
                && (('1'..='9').contains(c) || (*c == '0' && self.motion_count.is_some()))
            {
                let d = (*c as usize) - ('0' as usize);
                self.motion_count = Some(self.motion_count.unwrap_or(0) * 10 + d);
                return;
            }
        }
        // Handle g prefix for gg motion.
        if key == Key::char('g') && !self.g_prefix {
            self.g_prefix = true;
            return;
        }
        let mc = self.motion_count.unwrap_or(1);
        let total = self.op_count * mc;
        self.motion_count = None;
        let was_g = self.g_prefix;
        self.g_prefix = false;

        // Doubled operator (dd, cc, yy, >>, <<, ==).
        if !was_g && is_doubled(op, &key) {
            self.apply_line_operator(op, total);
            if !matches!(self.mode, Mode::Insert) {
                self.mode = Mode::Normal;
            }
            return;
        }
        // g-prefix motion (gg).
        if was_g {
            if key == Key::char('g') {
                self.apply_op_motion(op, &Motion::FirstLine, total);
            }
            self.mode = Mode::Normal;
            return;
        }
        // Standard motion.
        if let Some(motion) = key_to_motion(&key, total) {
            self.apply_op_motion(op, &motion, 1);
            if !matches!(self.mode, Mode::Insert) {
                self.mode = Mode::Normal;
            }
            return;
        }
        // Text object prefix: i or a. Mark/findchar pending: '/`/f/F/t/T.
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                if *c == 'i'
                    || *c == 'a'
                    || *c == '\''
                    || *c == '`'
                    || *c == 'f'
                    || *c == 'F'
                    || *c == 't'
                    || *c == 'T'
                {
                    self.text_obj_prefix = Some(*c);
                    return;
                }
            }
        }
        // Unrecognised key — cancel.
        self.mode = Mode::Normal;
    }

    fn apply_op_motion(&mut self, op: Operator, motion: &Motion, _count: usize) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let vh = self.viewport_height();
        let (dest, kind) = {
            let buf = match self.buffers.get(buf_id) {
                Some(b) => b,
                None => return,
            };
            resolve_motion(motion, cursor, &buf.content, vh)
        };
        let (start, end) = if (cursor.line, cursor.grapheme) <= (dest.line, dest.grapheme) {
            (cursor, dest)
        } else {
            (dest, cursor)
        };
        match kind {
            MotionKind::Linewise => {
                self.apply_linewise_op(op, start.line, end.line);
            }
            MotionKind::Inclusive => {
                self.apply_charwise_op(op, start, end, true);
            }
            MotionKind::Exclusive => {
                self.apply_charwise_op(op, start, end, false);
            }
        }
    }

    fn apply_line_operator(&mut self, op: Operator, count: usize) {
        let line = self.windows.focused().cursor.line;
        let end = line + count.saturating_sub(1);
        self.apply_linewise_op(op, line, end);
    }

    fn apply_text_object(&mut self, op: Operator, prefix: char, obj: char) {
        crate::text_objects::apply_text_object(self, op, prefix, obj);
    }

    fn apply_linewise_op(&mut self, op: Operator, start: usize, end: usize) {
        let count = end - start + 1;
        match op {
            Operator::Delete => self.delete_lines(count),
            Operator::Yank => self.yank_lines(count),
            Operator::Change => {
                self.delete_lines(count);
                self.open_above_impl();
                self.enter_insert();
            }
            Operator::Indent => self.indent_lines_range(start, end),
            Operator::Dedent => self.dedent_lines_range(start, end),
            Operator::Lowercase => self.lowercase_lines(start, end),
            Operator::Uppercase => self.uppercase_lines(start, end),
            Operator::Format => self.format_lines(start, end),
            _ => {}
        }
    }
}
