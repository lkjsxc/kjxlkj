//! Visual mode key dispatch.
//!
//! Motions extend the selection, operators act on it,
//! and `o` swaps cursor with anchor.

use kjxlkj_core_edit::{resolve_motion, Motion};
use kjxlkj_core_types::{
    CursorPosition, Key, KeyCode, Mode, Modifier, Operator, VisualKind,
};

use crate::editor::EditorState;

impl EditorState {
    /// Dispatch a key while in visual mode.
    pub(crate) fn dispatch_visual(
        &mut self, key: Key, kind: VisualKind,
    ) {
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                // Operators act on selection.
                if let Some(op) = char_to_operator(*c) {
                    self.visual_apply_operator(op, kind);
                    return;
                }
                if *c == 'o' {
                    self.visual_swap_anchor();
                    return;
                }
            }
        }
        // Motions extend selection.
        if let Some(motion) = visual_key_to_motion(&key) {
            self.visual_move(motion);
        }
    }

    fn visual_move(&mut self, motion: Motion) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let cursor = self.windows.focused().cursor;
            let vh = self.viewport_height();
            let (dest, _) =
                resolve_motion(&motion, cursor, &buf.content, vh);
            self.windows.focused_mut().cursor = dest;
        }
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    fn visual_swap_anchor(&mut self) {
        if let Some(anchor) = self.visual_anchor {
            let cursor = self.windows.focused().cursor;
            self.visual_anchor = Some(cursor);
            self.windows.focused_mut().cursor = anchor;
        }
    }

    fn visual_apply_operator(
        &mut self, op: Operator, kind: VisualKind,
    ) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => {
                self.mode = Mode::Normal;
                return;
            }
        };
        let cursor = self.windows.focused().cursor;
        let (start, end) = if (anchor.line, anchor.grapheme)
            <= (cursor.line, cursor.grapheme)
        {
            (anchor, cursor)
        } else {
            (cursor, anchor)
        };
        let linewise = kind == VisualKind::Line;
        self.apply_visual_op(op, start, end, linewise);
        if !matches!(self.mode, Mode::Insert) {
            self.mode = Mode::Normal;
        }
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    /// Compute ordered selection range for snapshot.
    pub fn visual_selection(
        &self,
    ) -> Option<(CursorPosition, CursorPosition)> {
        let anchor = self.visual_anchor?;
        let cursor = self.windows.focused().cursor;
        if (anchor.line, anchor.grapheme)
            <= (cursor.line, cursor.grapheme)
        {
            Some((anchor, cursor))
        } else {
            Some((cursor, anchor))
        }
    }
}

fn char_to_operator(c: char) -> Option<Operator> {
    match c {
        'd' | 'x' => Some(Operator::Delete),
        'c' | 's' => Some(Operator::Change),
        'y' => Some(Operator::Yank),
        '>' => Some(Operator::Indent),
        '<' => Some(Operator::Dedent),
        _ => None,
    }
}

fn visual_key_to_motion(key: &Key) -> Option<Motion> {
    if key.modifiers != Modifier::NONE {
        return match &key.code {
            KeyCode::Left => Some(Motion::Left(1)),
            KeyCode::Right => Some(Motion::Right(1)),
            KeyCode::Up => Some(Motion::Up(1)),
            KeyCode::Down => Some(Motion::Down(1)),
            _ => None,
        };
    }
    match &key.code {
        KeyCode::Char('h') | KeyCode::Left => Some(Motion::Left(1)),
        KeyCode::Char('j') | KeyCode::Down => Some(Motion::Down(1)),
        KeyCode::Char('k') | KeyCode::Up => Some(Motion::Up(1)),
        KeyCode::Char('l') | KeyCode::Right => Some(Motion::Right(1)),
        KeyCode::Char('w') => Some(Motion::WordForward(1)),
        KeyCode::Char('b') => Some(Motion::WordBackward(1)),
        KeyCode::Char('e') => Some(Motion::WordEndForward(1)),
        KeyCode::Char('0') => Some(Motion::LineStart),
        KeyCode::Char('^') => Some(Motion::FirstNonBlank),
        KeyCode::Char('$') => Some(Motion::LineEnd),
        KeyCode::Char('G') => Some(Motion::LastLine),
        _ => None,
    }
}
