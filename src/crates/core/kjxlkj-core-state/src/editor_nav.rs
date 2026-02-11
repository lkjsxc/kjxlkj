//! Jumplist, changelist, mark, macro, and fold navigation for EditorState.

use kjxlkj_core_types::{Action, ContentKind, Key, KeyModifiers, RangeType};

use crate::editor::EditorState;
use crate::macros;
use crate::navlist::Position;

impl EditorState {
    /// Navigate jumplist (Ctrl-o / Ctrl-i).
    pub(crate) fn navigate_jumplist(&mut self, action: &Action) {
        let pos = match action {
            Action::JumpOlder => self.jumplist.go_older(),
            Action::JumpNewer => self.jumplist.go_newer(),
            _ => return,
        };
        self.apply_nav_position(pos);
    }

    /// Navigate changelist (g; / g,).
    pub(crate) fn navigate_changelist(&mut self, action: &Action) {
        let pos = match action {
            Action::ChangeOlder => self.changelist.go_older(),
            Action::ChangeNewer => self.changelist.go_newer(),
            _ => return,
        };
        self.apply_nav_position(pos);
    }

    /// Apply a navigation position to the focused window cursor.
    fn apply_nav_position(&mut self, pos: Option<Position>) {
        let p = match pos { Some(p) => p, None => return };
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            if let ContentKind::Buffer(buf_id) = win.content {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let line = p.line.min(buf.line_count().saturating_sub(1));
                    let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                    win.cursor.line = line;
                    win.cursor.col = p.col.min(cols.saturating_sub(1));
                }
            }
        }
    }

    pub(crate) fn record_jump(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.jumplist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }

    pub(crate) fn record_change(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.changelist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }

    pub(crate) fn set_mark_at_cursor(&mut self, c: char) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.marks.set(c, win.cursor.line, win.cursor.col);
        }
    }

    pub(crate) fn goto_mark_line(&mut self, c: char) {
        let pos = match self.marks.get(c) { Some(p) => p, None => return };
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            if let ContentKind::Buffer(buf_id) = win.content {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let line = pos.line.min(buf.line_count().saturating_sub(1));
                    win.cursor.line = line;
                    let col = buf.line(line).map(|l| l.find(|ch: char| !ch.is_ascii_whitespace()).unwrap_or(0)).unwrap_or(0);
                    win.cursor.col = col;
                }
            }
        }
    }

    pub(crate) fn goto_mark_exact(&mut self, c: char) {
        let pos = match self.marks.get(c) { Some(p) => p, None => return };
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            if let ContentKind::Buffer(buf_id) = win.content {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let line = pos.line.min(buf.line_count().saturating_sub(1));
                    let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                    win.cursor.line = line;
                    win.cursor.col = pos.col.min(cols.saturating_sub(1));
                }
            }
        }
    }

    pub(crate) fn start_macro_recording(&mut self, c: char) { self.macro_state.start(c); }

    pub(crate) fn stop_macro_recording(&mut self) {
        if let Some((reg, keys)) = self.macro_state.stop() {
            let text = macros::keys_to_string(&keys);
            self.registers.write(reg, text, RangeType::Characterwise);
        }
    }

    pub(crate) fn play_macro(&mut self, c: char) {
        let entry = match self.registers.get(c) { Some(e) => e, None => return };
        let text = entry.text.clone();
        for mk in &parse_macro_keys(&text) { self.handle_key(&mk.0, &mk.1); }
    }

    pub(crate) fn fold_open(&mut self) { let l = self.focused_cursor_line(); self.fold_state.open(l); }
    pub(crate) fn fold_close(&mut self) { let l = self.focused_cursor_line(); self.fold_state.close(l); }
    pub(crate) fn fold_toggle(&mut self) { let l = self.focused_cursor_line(); self.fold_state.toggle(l); }
    pub(crate) fn fold_close_all(&mut self) { self.fold_state.close_all(); }

    pub(crate) fn fold_next(&mut self) {
        let line = self.focused_cursor_line();
        if let Some(t) = self.fold_state.next_closed(line) {
            if let Some(w) = self.windows.get_mut(&self.focus.focused) { w.cursor.line = t; }
        }
    }

    pub(crate) fn fold_prev(&mut self) {
        let line = self.focused_cursor_line();
        if let Some(t) = self.fold_state.prev_closed(line) {
            if let Some(w) = self.windows.get_mut(&self.focus.focused) { w.cursor.line = t; }
        }
    }

    fn focused_cursor_line(&self) -> usize {
        self.windows.get(&self.focus.focused).map(|w| w.cursor.line).unwrap_or(0)
    }
}

fn parse_macro_keys(s: &str) -> Vec<(Key, KeyModifiers)> {
    let mut keys = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '^' => { if let Some(&next) = chars.peek() { chars.next(); keys.push((Key::Char(next), KeyModifiers { ctrl: true, ..Default::default() })); } }
            '\n' => keys.push((Key::Enter, KeyModifiers::default())),
            '\t' => keys.push((Key::Tab, KeyModifiers::default())),
            '<' => {
                let mut tag = String::new();
                while let Some(&ch) = chars.peek() { if ch == '>' { chars.next(); break; } tag.push(ch); chars.next(); }
                match tag.as_str() {
                    "Esc" => keys.push((Key::Escape, KeyModifiers::default())),
                    "BS" => keys.push((Key::Backspace, KeyModifiers::default())),
                    _ => {}
                }
            }
            _ => keys.push((Key::Char(c), KeyModifiers::default())),
        }
    }
    keys
}
