//! Mark and dot-repeat operations for EditorState.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_types::{Action, Mode, Motion};

use crate::editor::MarkEntry;
use crate::EditorState;

impl EditorState {
    /// Set a mark at the current cursor position.
    pub(crate) fn do_set_mark(&mut self, ch: char) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (line, col) = self.cursor_pos();
        self.marks.insert(
            ch,
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Jump to a mark's exact position.
    pub(crate) fn do_jump_to_mark(&mut self, ch: char) {
        let entry = match self.marks.get(&ch) {
            Some(e) => *e,
            None => return,
        };
        // Switch buffer if needed.
        if self.active_buffer_id() != Some(entry.buffer)
        {
            if let Some(w) = self.focused_window_mut()
            {
                w.content = crate::WindowContent::Buffer(
                    entry.buffer,
                );
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line = entry.cursor.line;
            w.cursor.grapheme_offset =
                entry.cursor.grapheme_offset;
            w.viewport.follow_cursor(
                entry.cursor.line,
                3,
                0,
            );
        }
    }

    /// Jump to a mark's line (first non-blank).
    pub(crate) fn do_jump_to_mark_line(
        &mut self,
        ch: char,
    ) {
        let entry = match self.marks.get(&ch) {
            Some(e) => *e,
            None => return,
        };
        if self.active_buffer_id() != Some(entry.buffer)
        {
            if let Some(w) = self.focused_window_mut()
            {
                w.content = crate::WindowContent::Buffer(
                    entry.buffer,
                );
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line = entry.cursor.line;
            w.cursor.grapheme_offset = 0;
            w.viewport.follow_cursor(
                entry.cursor.line,
                3,
                0,
            );
        }
        // Move to first non-blank.
        self.do_motion(Motion::FirstNonBlank, 1);
    }

    /// Switch to the alternate buffer.
    pub(crate) fn do_alternate_file(&mut self) {
        let alt = match self.alternate_buffer {
            Some(b) => b,
            None => return,
        };
        if !self.buffers.contains_key(&alt) {
            return;
        }
        let current = self.active_buffer_id();
        if let Some(w) = self.focused_window_mut() {
            w.content =
                crate::WindowContent::Buffer(alt);
            w.cursor.line = 0;
            w.cursor.grapheme_offset = 0;
        }
        self.alternate_buffer = current;
    }

    /// Dot repeat: re-dispatch the last repeatable action.
    pub(crate) fn do_dot_repeat(&mut self) {
        if let Some(action) =
            self.last_repeatable.clone()
        {
            self.dispatch(action);
        }
    }

    /// Store a repeatable action.
    pub(crate) fn store_repeatable(
        &mut self,
        action: &Action,
    ) {
        match action {
            Action::InsertChar(_)
            | Action::DeleteCharForward
            | Action::DeleteCharBackward
            | Action::Delete(_, _)
            | Action::Change(_, _)
            | Action::Put(_)
            | Action::ReplaceChar(_)
            | Action::DoubleOperator(_, _)
            | Action::JoinLines
            | Action::ToggleCaseChar
            | Action::Increment(_)
            | Action::SubstituteChar
            | Action::SubstituteLine
            | Action::ChangeToEnd => {
                self.last_repeatable =
                    Some(action.clone());
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_jump_mark() {
        let mut ed = EditorState::new(80, 24);
        // Move cursor to line 0, col 5 (simulated)
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.grapheme_offset = 5;
        }
        ed.do_set_mark('a');
        // Move cursor away
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 3;
            w.cursor.grapheme_offset = 0;
        }
        ed.do_jump_to_mark('a');
        let (l, c) = ed.cursor_pos();
        assert_eq!(l, 0);
        assert_eq!(c, 5);
    }

    #[test]
    fn dot_repeat_stores_and_replays() {
        let mut ed = EditorState::new(80, 24);
        let action = Action::DeleteCharForward;
        ed.store_repeatable(&action);
        assert!(ed.last_repeatable.is_some());
    }
}
