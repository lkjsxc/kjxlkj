//! Change list: tracks positions of recent edits.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_types::BufferId;

use crate::EditorState;

/// Maximum change list entries.
const MAX_CHANGES: usize = 100;

impl EditorState {
    /// Push a change position. Deduplicates adjacent.
    pub(crate) fn push_change(&mut self) {
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (line, col) = self.cursor_pos();
        let pos = CursorPosition::new(line, col);
        let entry = (bid, pos);

        // Skip if same as last entry.
        if self.change_list.last() == Some(&entry) {
            return;
        }

        // Truncate forward entries if we're not
        // at the end.
        if self.change_list_pos < self.change_list.len() {
            self.change_list.truncate(self.change_list_pos);
        }

        self.change_list.push(entry);
        if self.change_list.len() > MAX_CHANGES {
            self.change_list.remove(0);
        }
        self.change_list_pos = self.change_list.len();
    }

    /// Jump to older change (`g;`).
    pub(crate) fn do_change_older(&mut self) {
        if self.change_list.is_empty() {
            return;
        }
        if self.change_list_pos == 0 {
            return;
        }
        self.change_list_pos -= 1;
        let (bid, pos) = self.change_list[self.change_list_pos];
        self.jump_to_change(bid, pos);
    }

    /// Jump to newer change (`g,`).
    pub(crate) fn do_change_newer(&mut self) {
        if self.change_list_pos >= self.change_list.len().saturating_sub(1) {
            return;
        }
        self.change_list_pos += 1;
        let (bid, pos) = self.change_list[self.change_list_pos];
        self.jump_to_change(bid, pos);
    }

    fn jump_to_change(&mut self, bid: BufferId, pos: CursorPosition) {
        if self.active_buffer_id() != Some(bid) {
            if self.buffers.contains_key(&bid) {
                if let Some(w) = self.focused_window_mut() {
                    w.content = crate::WindowContent::Buffer(bid);
                }
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor = pos;
            w.viewport.follow_cursor(pos.line, 3, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_navigate_changes() {
        let mut ed = EditorState::new(80, 24);
        // Make some "changes" at different positions.
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 5;
        }
        ed.push_change();
        if let Some(w) = ed.focused_window_mut() {
            w.cursor.line = 10;
        }
        ed.push_change();
        assert_eq!(ed.change_list.len(), 2);

        // First g; goes to most recent change (line 10).
        ed.do_change_older();
        assert_eq!(ed.cursor_pos().0, 10);
        // Second g; goes to older change (line 5).
        ed.do_change_older();
        assert_eq!(ed.cursor_pos().0, 5);
        // g, goes back to newer change (line 10).
        ed.do_change_newer();
        assert_eq!(ed.cursor_pos().0, 10);
    }

    #[test]
    fn change_list_deduplicates() {
        let mut ed = EditorState::new(80, 24);
        ed.push_change();
        ed.push_change(); // Same position.
        assert_eq!(ed.change_list.len(), 1);
    }
}
