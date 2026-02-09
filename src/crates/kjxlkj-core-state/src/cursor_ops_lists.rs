use crate::editor::EditorState;
/// Changelist and jump list navigation for EditorState.
use kjxlkj_core_types::CursorPosition;

impl EditorState {
    /// Push a position onto the changelist.
    pub(crate) fn push_changelist(&mut self, line: usize, col: usize) {
        let bid = self.current_buffer_id().0 as usize;
        self.changelist.push((bid, line, col));
        self.changelist_idx = self.changelist.len();
    }

    /// Navigate to older changelist entry (g;).
    pub(crate) fn changelist_older(&mut self) {
        if self.changelist.is_empty() || self.changelist_idx == 0 {
            self.notify_error("E662: At start of changelist");
            return;
        }
        self.changelist_idx -= 1;
        let (_, line, col) = self.changelist[self.changelist_idx];
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }

    /// Navigate to newer changelist entry (g,).
    pub(crate) fn changelist_newer(&mut self) {
        if self.changelist_idx >= self.changelist.len().saturating_sub(1) {
            self.notify_error("E663: At end of changelist");
            return;
        }
        self.changelist_idx += 1;
        let (_, line, col) = self.changelist[self.changelist_idx];
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }

    /// Push current position onto the jump list.
    pub(crate) fn push_jumplist(&mut self) {
        let bid = self.current_buffer_id().0 as usize;
        let c = self.windows.focused().cursor;
        self.jumplist.truncate(self.jumplist_idx);
        self.jumplist.push((bid, c.line, c.grapheme));
        self.jumplist_idx = self.jumplist.len();
    }

    /// Navigate to older jump list entry (Ctrl-O).
    pub(crate) fn jump_older(&mut self) {
        if self.jumplist.is_empty() || self.jumplist_idx == 0 {
            self.notify_error("E662: At start of jumplist");
            return;
        }
        if self.jumplist_idx == self.jumplist.len() {
            let bid = self.current_buffer_id().0 as usize;
            let c = self.windows.focused().cursor;
            self.jumplist.push((bid, c.line, c.grapheme));
        }
        self.jumplist_idx -= 1;
        let (bid, line, col) = self.jumplist[self.jumplist_idx];
        self.switch_to_buffer_id(bid);
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }

    /// Navigate to newer jump list entry (Ctrl-I).
    pub(crate) fn jump_newer(&mut self) {
        if self.jumplist_idx >= self.jumplist.len().saturating_sub(1) {
            self.notify_error("E663: At end of jumplist");
            return;
        }
        self.jumplist_idx += 1;
        let (bid, line, col) = self.jumplist[self.jumplist_idx];
        self.switch_to_buffer_id(bid);
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }

    /// Switch the focused window to a buffer by raw buffer ID.
    pub(crate) fn switch_to_buffer_id(&mut self, bid: usize) {
        let cur = self.current_buffer_id().0 as usize;
        if bid != cur {
            use kjxlkj_core_types::{BufferId, ContentSource};
            let target = BufferId(bid as u64);
            if self.buffers.get(target).is_some() {
                self.windows.focused_mut().content = ContentSource::Buffer(target);
            }
        }
    }
}
