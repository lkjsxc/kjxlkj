//! Additional editor actions: operator on text object,
//! buffer switch by name, and bracketed paste.

use kjxlkj_core_edit::{
    resolve_text_object, CursorPosition,
};
use kjxlkj_core_types::{Mode, Operator, TextObject};

use crate::EditorState;

impl EditorState {
    /// Apply an operator to a text object.
    pub(crate) fn do_operator_text_object(
        &mut self,
        op: Operator,
        text_obj: TextObject,
        count: u32,
    ) {
        let bid = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let cursor =
            match self.focused_window() {
                Some(w) => w.cursor,
                None => return,
            };

        let range = {
            let buf =
                match self.buffers.get(&bid) {
                    Some(b) => b,
                    None => return,
                };
            resolve_text_object(
                text_obj, &cursor, &buf.content,
                count,
            )
        };

        let range = match range {
            Some(r) => r,
            None => return,
        };

        let mut enter_insert = false;

        if let Some(buf) =
            self.buffers.get_mut(&bid)
        {
            let s = buf.content.line_grapheme_to_offset(
                range.start.line,
                range.start.grapheme_offset,
            );
            let e = buf.content.line_grapheme_to_offset(
                range.end.line,
                range.end.grapheme_offset,
            );
            let (s, e) = (s.min(e), s.max(e) + 1);
            let e = e.min(buf.content.len_chars());

            match op {
                Operator::Delete => {
                    buf.content.delete_range(s, e);
                    buf.modified = true;
                }
                Operator::Change => {
                    buf.content.delete_range(s, e);
                    buf.modified = true;
                    enter_insert = true;
                }
                Operator::Yank => { /* stored in unnamed */ }
                _ => {}
            }
        }

        if enter_insert {
            self.mode = Mode::Insert;
        }
    }

    /// Switch to a buffer by name or number.
    pub(crate) fn do_switch_buffer(
        &mut self,
        name: &str,
    ) {
        if let Ok(n) = name.parse::<u64>() {
            let bid =
                kjxlkj_core_types::BufferId(n);
            if self.buffers.contains_key(&bid) {
                if let Some(cur) =
                    self.active_buffer_id()
                {
                    self.alternate_buffer = Some(cur);
                }
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.set_buffer(bid);
                }
                return;
            }
        }
        for (&id, buf) in &self.buffers {
            let name_match = buf.name == name;
            let path_match = buf
                .path
                .as_ref()
                .map(|p| {
                    p.to_string_lossy()
                        .ends_with(name)
                })
                .unwrap_or(false);
            if name_match || path_match {
                if let Some(cur) =
                    self.active_buffer_id()
                {
                    self.alternate_buffer = Some(cur);
                }
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.set_buffer(id);
                }
                return;
            }
        }
    }

    /// Handle bracketed paste: insert text at cursor.
    pub(crate) fn do_paste_text(
        &mut self,
        text: &str,
    ) {
        for ch in text.chars() {
            self.insert_char(ch);
        }
    }
}

