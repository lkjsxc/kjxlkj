//! Jumplist, changelist, mark, and macro navigation for EditorState.

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
        if let Some(p) = pos {
            let wid = self.focus.focused;
            if let Some(win) = self.windows.get_mut(&wid) {
                if let ContentKind::Buffer(buf_id) = win.content {
                    if let Some(buf) = self.buffers.get(&buf_id) {
                        let lines = buf.line_count();
                        let line = p.line.min(lines.saturating_sub(1));
                        let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                        win.cursor.line = line;
                        win.cursor.col = p.col.min(cols.saturating_sub(1));
                    }
                }
            }
        }
    }

    /// Navigate changelist (g; / g,).
    pub(crate) fn navigate_changelist(&mut self, action: &Action) {
        let pos = match action {
            Action::ChangeOlder => self.changelist.go_older(),
            Action::ChangeNewer => self.changelist.go_newer(),
            _ => return,
        };
        if let Some(p) = pos {
            let wid = self.focus.focused;
            if let Some(win) = self.windows.get_mut(&wid) {
                if let ContentKind::Buffer(buf_id) = win.content {
                    if let Some(buf) = self.buffers.get(&buf_id) {
                        let lines = buf.line_count();
                        let line = p.line.min(lines.saturating_sub(1));
                        let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                        win.cursor.line = line;
                        win.cursor.col = p.col.min(cols.saturating_sub(1));
                    }
                }
            }
        }
    }

    /// Record current cursor position in the jumplist.
    pub(crate) fn record_jump(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.jumplist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }

    /// Record current cursor position in the changelist.
    pub(crate) fn record_change(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.changelist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }

    /// Set mark `c` at the current cursor position (`m{a-z}`).
    pub(crate) fn set_mark_at_cursor(&mut self, c: char) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.marks.set(c, win.cursor.line, win.cursor.col);
        }
    }

    /// Go to mark line, placing cursor at first non-blank (`'{a-z}`).
    pub(crate) fn goto_mark_line(&mut self, c: char) {
        let pos = match self.marks.get(c) { Some(p) => p, None => return };
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            if let ContentKind::Buffer(buf_id) = win.content {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let lines = buf.line_count();
                    let line = pos.line.min(lines.saturating_sub(1));
                    win.cursor.line = line;
                    // First non-blank on the target line.
                    let col = buf.line(line)
                        .map(|l| l.find(|ch: char| !ch.is_ascii_whitespace()).unwrap_or(0))
                        .unwrap_or(0);
                    win.cursor.col = col;
                }
            }
        }
    }

    /// Go to exact mark position (`\`{a-z}`).
    pub(crate) fn goto_mark_exact(&mut self, c: char) {
        let pos = match self.marks.get(c) { Some(p) => p, None => return };
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get_mut(&wid) {
            if let ContentKind::Buffer(buf_id) = win.content {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let lines = buf.line_count();
                    let line = pos.line.min(lines.saturating_sub(1));
                    let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                    win.cursor.line = line;
                    win.cursor.col = pos.col.min(cols.saturating_sub(1));
                }
            }
        }
    }

    /// Start macro recording into register `c`.
    pub(crate) fn start_macro_recording(&mut self, c: char) {
        self.macro_state.start(c);
    }

    /// Stop macro recording and save captured keys to the register.
    pub(crate) fn stop_macro_recording(&mut self) {
        if let Some((reg, keys)) = self.macro_state.stop() {
            let text = macros::keys_to_string(&keys);
            self.registers.write(reg, text, RangeType::Characterwise);
        }
    }

    /// Play macro from register `c`: replay captured key sequence.
    pub(crate) fn play_macro(&mut self, c: char) {
        let entry = match self.registers.get(c) { Some(e) => e, None => return };
        let text = entry.text.clone();
        let keys = parse_macro_keys(&text);
        for mk in &keys {
            self.handle_key(&mk.0, &mk.1);
        }
    }
}

/// Parse a macro string back into key events.
fn parse_macro_keys(s: &str) -> Vec<(Key, KeyModifiers)> {
    let mut keys = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '^' => {
                if let Some(&next) = chars.peek() {
                    chars.next();
                    keys.push((Key::Char(next), KeyModifiers { ctrl: true, ..Default::default() }));
                }
            }
            '\n' => keys.push((Key::Enter, KeyModifiers::default())),
            '\t' => keys.push((Key::Tab, KeyModifiers::default())),
            '<' => {
                let mut tag = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '>' { chars.next(); break; }
                    tag.push(ch); chars.next();
                }
                match tag.as_str() {
                    "Esc" => keys.push((Key::Escape, KeyModifiers::default())),
                    "BS" => keys.push((Key::Backspace, KeyModifiers::default())),
                    _ => {} // unknown tag — skip
                }
            }
            _ => keys.push((Key::Char(c), KeyModifiers::default())),
        }
    }
    keys
}
