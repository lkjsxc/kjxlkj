//! Visual mode key dispatch.
use kjxlkj_core_edit::{resolve_motion, Motion};
use kjxlkj_core_types::{CursorPosition, Key, KeyCode, Mode, Modifier, Operator, VisualKind};

use crate::editor::EditorState;

impl EditorState {
    /// Dispatch a key while in visual mode.
    pub(crate) fn dispatch_visual(&mut self, key: Key, kind: VisualKind) {
        if key.modifiers == Modifier::NONE {
            if let KeyCode::Char(c) = &key.code {
                // : in visual mode opens cmdline with '<,'>
                if *c == ':' {
                    self.visual_set_marks_on_exit();
                    self.visual_anchor = None;
                    self.mode = Mode::Command(kjxlkj_core_types::CommandKind::Ex);
                    self.cmdline.open(':');
                    self.cmdline.content = "'<,'>".to_string();
                    self.cmdline.cursor_pos = 5;
                    return;
                }
                // Operators act on selection.
                if let Some(op) = char_to_operator(*c) {
                    self.visual_apply_operator(op, kind);
                    return;
                }
                if *c == 'o' {
                    self.visual_swap_anchor();
                    return;
                }
                if *c == 'p' || *c == 'P' {
                    self.visual_paste(kind);
                    return;
                }
                if kind == VisualKind::Block && (*c == 'I' || *c == 'A') {
                    self.handle_visual_block_ia(*c == 'A');
                    return;
                }
            }
        }
        // Motions extend selection.
        if let Some(motion) = visual_key_to_motion(&key) {
            self.visual_move(motion);
        }
    }

    fn visual_set_marks_on_exit(&mut self) {
        if let Some(anchor) = self.visual_anchor {
            let cursor = self.windows.focused().cursor;
            let bid = self.current_buffer_id().0 as usize;
            let (s, e) = if (anchor.line, anchor.grapheme) <= (cursor.line, cursor.grapheme) {
                (anchor, cursor)
            } else {
                (cursor, anchor)
            };
            let sm = crate::marks::MarkPosition {
                buffer_id: bid,
                line: s.line,
                col: s.grapheme,
            };
            let em = crate::marks::MarkPosition {
                buffer_id: bid,
                line: e.line,
                col: e.grapheme,
            };
            self.marks.set_visual_start(sm);
            self.marks.set_visual_end(em);
        }
    }

    fn visual_move(&mut self, motion: Motion) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let cursor = self.windows.focused().cursor;
            let vh = self.viewport_height();
            let (dest, _) = resolve_motion(&motion, cursor, &buf.content, vh);
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

    #[rustfmt::skip]
    fn visual_apply_operator(&mut self, op: Operator, kind: VisualKind) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => { self.mode = Mode::Normal; return; }
        };
        let cursor = self.windows.focused().cursor;
        let (start, end) = if (anchor.line, anchor.grapheme) <= (cursor.line, cursor.grapheme)
            { (anchor, cursor) } else { (cursor, anchor) };
        match kind {
            VisualKind::Block => self.apply_block_op(op, start, end),
            _ => {
                let linewise = kind == VisualKind::Line;
                self.apply_visual_op(op, start, end, linewise);
            }
        }
        if !matches!(self.mode, Mode::Insert) { self.mode = Mode::Normal; }
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    fn apply_block_op(&mut self, op: Operator, start: CursorPosition, end: CursorPosition) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        let col_start = start.grapheme.min(end.grapheme);
        let col_end = start.grapheme.max(end.grapheme);
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
        }
        match op {
            Operator::Delete | Operator::Change => {
                for line in (start.line..=end.line).rev() {
                    let s = CursorPosition::new(line, col_start);
                    let e = CursorPosition::new(line, col_end);
                    self.delete_range_raw(buf_id, s.line, s.grapheme, e.line, e.grapheme + 1);
                }
                self.windows.focused_mut().cursor = start;
                if op == Operator::Change {
                    self.block_insert_pending = Some((start.line, end.line, col_start, false));
                    self.last_inserted_text.clear();
                    self.mode = Mode::Insert;
                }
            }
            Operator::Yank => {
                let mut text = String::new();
                if let Some(buf) = self.buffers.get(buf_id) {
                    for line in start.line..=end.line {
                        let s = CursorPosition::new(line, col_start);
                        let e = CursorPosition::new(line, col_end);
                        let chunk = self.extract_range(buf_id, s, e, true);
                        let _ = buf;
                        text.push_str(&chunk);
                        text.push('\n');
                    }
                }
                self.store_register(text, false);
            }
            _ => {}
        }
    }

    /// Compute ordered selection range for snapshot.
    pub fn visual_selection(&self) -> Option<(CursorPosition, CursorPosition)> {
        let anchor = self.visual_anchor?;
        let cursor = self.windows.focused().cursor;
        if (anchor.line, anchor.grapheme) <= (cursor.line, cursor.grapheme) {
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

#[rustfmt::skip]
fn visual_key_to_motion(key: &Key) -> Option<Motion> {
    if key.modifiers != Modifier::NONE {
        return match &key.code {
            KeyCode::Left  => Some(Motion::Left(1)),
            KeyCode::Right => Some(Motion::Right(1)),
            KeyCode::Up    => Some(Motion::Up(1)),
            KeyCode::Down  => Some(Motion::Down(1)),
            _ => None,
        };
    }
    match &key.code {
        KeyCode::Char('h') | KeyCode::Left  => Some(Motion::Left(1)),
        KeyCode::Char('j') | KeyCode::Down  => Some(Motion::Down(1)),
        KeyCode::Char('k') | KeyCode::Up    => Some(Motion::Up(1)),
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
