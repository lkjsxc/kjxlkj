use kjxlkj_core_edit::Edit;
use kjxlkj_core_types::{CursorPos, TextRange};

use super::EditorState;

impl EditorState {
    pub(super) fn insert_text(&mut self, text: &str) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let at = match buf.text.cursor_to_char(win.cursor) {
            Ok(i) => i,
            Err(_) => return,
        };
        if buf.text.insert(at, text).is_err() {
            return;
        }
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo
            .push_transaction(vec![Edit::Insert { at, text: text.to_string() }]);
        win.cursor = advance_cursor(win.cursor, text);
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }

    pub(super) fn backspace(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let at = match buf.text.cursor_to_char(win.cursor) {
            Ok(i) => i,
            Err(_) => return,
        };
        if at == 0 {
            return;
        }
        let start = at - 1;
        let range = TextRange { start, end: at };
        let deleted = match buf.text.remove(range) {
            Ok(s) => s,
            Err(_) => return,
        };
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo.push_transaction(vec![Edit::Delete { range, deleted }]);
        win.cursor = buf.text.char_to_cursor(start).unwrap_or_default();
    }

    pub(super) fn delete_char_under_cursor(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let at = match buf.text.cursor_to_char(win.cursor) {
            Ok(i) => i,
            Err(_) => return,
        };
        match buf.text.char_at(at) {
            Some('\n') | None => return,
            Some(_) => {}
        }
        let range = TextRange { start: at, end: at + 1 };
        let deleted = match buf.text.remove(range) {
            Ok(s) => s,
            Err(_) => return,
        };
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo.push_transaction(vec![Edit::Delete { range, deleted }]);
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }

    pub(super) fn delete_current_line(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let line = buf.text.clamp_cursor(win.cursor).line;
        let Some(range) = buf.text.line_char_range(line) else {
            return;
        };
        let deleted = match buf.text.remove(range) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.yank = deleted.clone();
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo.push_transaction(vec![Edit::Delete { range, deleted }]);

        let last_line = buf.text.line_count().saturating_sub(1);
        win.cursor.line = line.min(last_line);
        win.cursor.col = 0;
    }

    pub(super) fn yank_current_line(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let line = buf.text.clamp_cursor(win.cursor).line;
        let Some(range) = buf.text.line_char_range(line) else {
            return;
        };
        if let Ok(s) = buf.text.slice(range) {
            self.yank = s;
        }
    }

    pub(super) fn paste_after(&mut self) {
        if self.yank.is_empty() {
            return;
        }
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let at = match buf.text.cursor_to_char(win.cursor) {
            Ok(i) => i,
            Err(_) => return,
        };
        let insert_at = match buf.text.char_at(at) {
            Some('\n') | None => at,
            Some(_) => at + 1,
        };
        if buf.text.insert(insert_at, &self.yank).is_err() {
            return;
        }
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo
            .push_transaction(vec![Edit::Insert { at: insert_at, text: self.yank.clone() }]);
        let end = insert_at.saturating_add(self.yank.chars().count());
        win.cursor = buf.text.char_to_cursor(end).unwrap_or(win.cursor);
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }

    pub(super) fn delete_visual_selection(&mut self) {
        let Some(anchor) = self.visual_anchor else {
            return;
        };
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let a = buf.text.cursor_to_char(anchor).ok();
        let b = buf.text.cursor_to_char(win.cursor).ok();
        let (Some(a), Some(b)) = (a, b) else {
            return;
        };
        let len = buf.text.len_chars();
        let start = a.min(b);
        let end = a.max(b).saturating_add(1).min(len);
        if start >= end {
            return;
        }
        let range = TextRange { start, end };
        let deleted = match buf.text.remove(range) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.yank = deleted.clone();
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo.push_transaction(vec![Edit::Delete { range, deleted }]);
        win.cursor = buf.text.char_to_cursor(start).unwrap_or_default();
    }

    pub(super) fn yank_visual_selection(&mut self) {
        let Some(anchor) = self.visual_anchor else {
            return;
        };
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let a = buf.text.cursor_to_char(anchor).ok();
        let b = buf.text.cursor_to_char(win.cursor).ok();
        let (Some(a), Some(b)) = (a, b) else {
            return;
        };
        let len = buf.text.len_chars();
        let start = a.min(b);
        let end = a.max(b).saturating_add(1).min(len);
        if start >= end {
            return;
        }
        let range = TextRange { start, end };
        if let Ok(s) = buf.text.slice(range) {
            self.yank = s;
        }
    }
}

fn advance_cursor(mut cursor: CursorPos, inserted: &str) -> CursorPos {
    if let Some((_, tail)) = inserted.rsplit_once('\n') {
        cursor.line = cursor.line.saturating_add(inserted.matches('\n').count());
        cursor.col = tail.chars().count();
        return cursor;
    }
    cursor.col = cursor.col.saturating_add(inserted.chars().count());
    cursor
}
