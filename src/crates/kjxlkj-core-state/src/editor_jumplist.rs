//! Jump list for EditorState.
//!
//! Tracks cursor positions for Ctrl-O (older) and
//! Ctrl-I (newer) navigation.

use crate::EditorState;

impl EditorState {
    /// Push current position onto the jump list.
    pub(crate) fn push_jump(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let cursor =
            match self.focused_window() {
                Some(w) => w.cursor,
                None => return,
            };

        // Truncate forward entries.
        if self.jump_list_pos
            < self.jump_list.len()
        {
            self.jump_list
                .truncate(self.jump_list_pos);
        }

        // Avoid consecutive duplicates.
        if let Some(last) = self.jump_list.last() {
            if last.0 == buf_id && last.1 == cursor {
                return;
            }
        }

        self.jump_list.push((buf_id, cursor));
        if self.jump_list.len() > 100 {
            self.jump_list.remove(0);
        }
        self.jump_list_pos = self.jump_list.len();
    }

    /// Jump to older position (Ctrl-O).
    pub(crate) fn do_jump_older(&mut self) {
        if self.jump_list_pos == 0
            || self.jump_list.is_empty()
        {
            return;
        }
        self.jump_list_pos -= 1;
        let (buf_id, cursor) =
            self.jump_list[self.jump_list_pos];
        if self.active_buffer_id() != Some(buf_id) {
            if self.buffers.contains_key(&buf_id) {
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.set_buffer(buf_id);
                }
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor = cursor;
        }
    }

    /// Jump to newer position (Ctrl-I / Tab).
    pub(crate) fn do_jump_newer(&mut self) {
        if self.jump_list_pos
            >= self.jump_list.len()
        {
            return;
        }
        let (buf_id, cursor) =
            self.jump_list[self.jump_list_pos];
        self.jump_list_pos += 1;
        if self.active_buffer_id() != Some(buf_id) {
            if self.buffers.contains_key(&buf_id) {
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.set_buffer(buf_id);
                }
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor = cursor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_list_push_and_older() {
        let mut ed = EditorState::new(80, 24);
        ed.push_jump();
        assert_eq!(ed.jump_list.len(), 1);
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 5;
        }
        ed.push_jump();
        assert_eq!(ed.jump_list.len(), 2);
        ed.do_jump_older();
        assert_eq!(ed.jump_list_pos, 1);
    }

    #[test]
    fn jump_list_newer() {
        let mut ed = EditorState::new(80, 24);
        ed.push_jump();
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 10;
        }
        ed.push_jump();
        ed.do_jump_older();
        ed.do_jump_newer();
        assert_eq!(ed.jump_list_pos, 2);
    }
}
