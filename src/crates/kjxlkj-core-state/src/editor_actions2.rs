//! Editor actions: undo/redo, put, insert register.

use crate::EditorState;

impl EditorState {
    pub(crate) fn do_undo(&mut self) {
        if let Some(buf) = self.active_buffer_mut() {
            buf.undo();
        }
    }

    pub(crate) fn do_redo(&mut self) {
        if let Some(buf) = self.active_buffer_mut() {
            buf.redo();
        }
    }

    pub(crate) fn do_put(&mut self, before: bool) {
        let text = self
            .register_file
            .unnamed()
            .map(|r| r.content.clone())
            .unwrap_or_default();
        if text.is_empty() {
            return;
        }
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            let insert_at = if before {
                off
            } else {
                off + 1
            };
            let insert_at = insert_at
                .min(buf.content.len_chars());
            for (i, ch) in
                text.chars().enumerate()
            {
                buf.content
                    .insert_char(insert_at + i, ch);
            }
            buf.modified = true;
        }
        let gc = text.chars().count();
        if !before {
            if let Some(w) =
                self.focused_window_mut()
            {
                w.cursor.grapheme_offset += gc;
            }
        }
    }

    /// Insert register contents at cursor
    /// (insert mode Ctrl-R).
    pub(crate) fn do_insert_register(
        &mut self,
        reg: char,
    ) {
        let name =
            crate::register_name_from_char(reg);
        let text = name
            .and_then(|n| {
                self.register_file
                    .get(n)
                    .map(|r| r.content.clone())
            })
            .unwrap_or_default();
        if text.is_empty() {
            return;
        }
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            for (i, ch) in
                text.chars().enumerate()
            {
                buf.content
                    .insert_char(off + i, ch);
            }
            buf.modified = true;
        }
        let gc = text.chars().count();
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += gc;
        }
    }
}
