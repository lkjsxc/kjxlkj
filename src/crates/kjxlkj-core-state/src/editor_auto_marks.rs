//! Automatic mark management: update `.` `^` `[` `]`
//! marks after edits and insert mode operations.

use kjxlkj_core_edit::CursorPosition;

use crate::editor_types::MarkEntry;
use crate::EditorState;

impl EditorState {
    /// Update the `.` mark (last change position).
    pub(crate) fn update_dot_mark(&mut self) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (line, col) = self.cursor_pos();
        self.marks.insert(
            '.',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Update the `^` mark (last insert mode exit).
    pub(crate) fn update_caret_mark(&mut self) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (line, col) = self.cursor_pos();
        self.marks.insert(
            '^',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Update the `[` mark (start of last change).
    pub(crate) fn update_bracket_start_mark(
        &mut self,
        line: usize,
        col: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        self.marks.insert(
            '[',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Update the `]` mark (end of last change).
    pub(crate) fn update_bracket_end_mark(
        &mut self,
        line: usize,
        col: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        self.marks.insert(
            ']',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Update visual selection marks `<` and `>`.
    pub(crate) fn update_visual_marks(
        &mut self,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        self.marks.insert(
            '<',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(
                    start_line, start_col,
                ),
            },
        );
        self.marks.insert(
            '>',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(
                    end_line, end_col,
                ),
            },
        );
    }

    /// Update the backtick mark (position before last jump).
    pub(crate) fn update_jump_mark(&mut self) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (line, col) = self.cursor_pos();
        self.marks.insert(
            '`',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
        // Also set ' (single quote) to same line
        self.marks.insert(
            '\'',
            MarkEntry {
                buffer: bid,
                cursor: CursorPosition::new(line, col),
            },
        );
    }

    /// Populate read-only registers with dynamic values.
    pub(crate) fn populate_readonly_registers(
        &mut self,
    ) {
        use kjxlkj_core_types::{Register, RegisterName};

        // `%` — current file name.
        if let Some(buf) = self.active_buffer() {
            let name = buf.name.clone();
            self.register_file.registers_mut().insert(
                RegisterName::FileName,
                Register::new(name, false),
            );

            // `.` — last inserted text (placeholder).
            // Already stored by insert exit if implemented
        }

        // `#` — alternate file name.
        if let Some(alt_id) = self.alternate_buffer {
            if let Some(alt_buf) =
                self.buffers.get(&alt_id)
            {
                let name = alt_buf.name.clone();
                self.register_file
                    .registers_mut()
                    .insert(
                        RegisterName::AlternateFileName,
                        Register::new(name, false),
                    );
            }
        }

        // `:` — last ex command.
        if let Some(cs) = &self.command_state {
            if !cs.history.is_empty() {
                let last = cs
                    .history
                    .last()
                    .unwrap()
                    .clone();
                self.register_file
                    .registers_mut()
                    .insert(
                        RegisterName::LastCommand,
                        Register::new(last, false),
                    );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_mark_updated() {
        let mut ed = EditorState::new(80, 24);
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 3;
            w.cursor.grapheme_offset = 7;
        }
        ed.update_dot_mark();
        let m = ed.marks.get(&'.').unwrap();
        assert_eq!(m.cursor.line, 3);
        assert_eq!(m.cursor.grapheme_offset, 7);
    }

    #[test]
    fn caret_mark_updated() {
        let mut ed = EditorState::new(80, 24);
        ed.update_caret_mark();
        assert!(ed.marks.contains_key(&'^'));
    }
}
