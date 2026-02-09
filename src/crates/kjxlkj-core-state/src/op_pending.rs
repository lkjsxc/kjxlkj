//! Operator-pending mode dispatch.
//!
//! Parses a motion or doubled operator, computes the affected
//! range, and applies the pending operator.

use kjxlkj_core_edit::{resolve_motion, Motion, MotionKind};
use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier, Operator};

use crate::editor::EditorState;

impl EditorState {
    /// Dispatch a key while in operator-pending mode.
    pub(crate) fn dispatch_op_pending(
        &mut self, key: Key, op: Operator,
    ) {
        // Accumulate motion count digits.
        if let KeyCode::Char(c) = &key.code {
            if key.modifiers == Modifier::NONE
                && (('1'..='9').contains(c)
                    || (*c == '0' && self.motion_count.is_some()))
            {
                let d = (*c as usize) - ('0' as usize);
                self.motion_count =
                    Some(self.motion_count.unwrap_or(0) * 10 + d);
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
        // Unrecognised key — cancel.
        self.mode = Mode::Normal;
    }

    fn apply_op_motion(
        &mut self, op: Operator, motion: &Motion, _count: usize,
    ) {
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
        let (start, end) = if (cursor.line, cursor.grapheme)
            <= (dest.line, dest.grapheme)
        {
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

    fn apply_linewise_op(
        &mut self, op: Operator, start: usize, end: usize,
    ) {
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
            _ => {}
        }
    }
}

fn is_doubled(op: Operator, key: &Key) -> bool {
    if key.modifiers != Modifier::NONE {
        return false;
    }
    matches!(
        (op, &key.code),
        (Operator::Delete, KeyCode::Char('d'))
            | (Operator::Change, KeyCode::Char('c'))
            | (Operator::Yank, KeyCode::Char('y'))
            | (Operator::Indent, KeyCode::Char('>'))
            | (Operator::Dedent, KeyCode::Char('<'))
            | (Operator::Reindent, KeyCode::Char('='))
    )
}

fn key_to_motion(key: &Key, count: usize) -> Option<Motion> {
    if key.modifiers != Modifier::NONE {
        return match &key.code {
            KeyCode::Left => Some(Motion::Left(count)),
            KeyCode::Right => Some(Motion::Right(count)),
            KeyCode::Up => Some(Motion::Up(count)),
            KeyCode::Down => Some(Motion::Down(count)),
            _ => None,
        };
    }
    match &key.code {
        KeyCode::Char('h') | KeyCode::Left => Some(Motion::Left(count)),
        KeyCode::Char('j') | KeyCode::Down => Some(Motion::Down(count)),
        KeyCode::Char('k') | KeyCode::Up => Some(Motion::Up(count)),
        KeyCode::Char('l') | KeyCode::Right => {
            Some(Motion::Right(count))
        }
        KeyCode::Char('w') => Some(Motion::WordForward(count)),
        KeyCode::Char('b') => Some(Motion::WordBackward(count)),
        KeyCode::Char('e') => Some(Motion::WordEndForward(count)),
        KeyCode::Char('0') => Some(Motion::LineStart),
        KeyCode::Char('^') => Some(Motion::FirstNonBlank),
        KeyCode::Char('$') => Some(Motion::LineEnd),
        KeyCode::Char('G') => Some(Motion::LastLine),
        _ => None,
    }
}
