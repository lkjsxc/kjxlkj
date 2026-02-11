//! Command-line input handling for EditorState.
//! See /docs/spec/commands/syntax.md.

use kjxlkj_core_types::{
    CmdlineState, CommandKind, ContentKind, Key,
    KeyModifiers, Mode,
};

use crate::command_parse::parse_ex_command;
use crate::editor::EditorState;
use crate::search::SearchDirection;

impl EditorState {
    /// Handle a key while in Command mode.
    pub(crate) fn handle_command_input(
        &mut self, key: &Key, mods: &KeyModifiers, kind: CommandKind,
    ) {
        if mods.ctrl { match key {
            Key::Char('b') => { self.cmdline.cursor_pos = 0; }
            Key::Char('e') => { self.cmdline.cursor_pos = self.cmdline.content.len(); }
            Key::Char('w') => { self.cmdline_delete_word_backward(); }
            Key::Char('u') => { self.cmdline_delete_to_start(); }
            Key::Char('c') => { self.cmdline = CmdlineState::default(); self.mode = Mode::Normal; }
            _ => {}
        } self.sequence += 1; return; }
        match key {
            Key::Escape => { self.cmdline = CmdlineState::default(); self.mode = Mode::Normal; }
            Key::Enter => {
                let content = self.cmdline.content.clone();
                self.cmdline = CmdlineState::default();
                self.mode = Mode::Normal;
                self.dispatch_cmdline(kind, &content);
            }
            Key::Left => { self.cmdline.cursor_pos = self.cmdline.cursor_pos.saturating_sub(1); }
            Key::Right => { if self.cmdline.cursor_pos < self.cmdline.content.len() { self.cmdline.cursor_pos += 1; } }
            Key::Home => { self.cmdline.cursor_pos = 0; }
            Key::End => { self.cmdline.cursor_pos = self.cmdline.content.len(); }
            Key::Backspace => {
                if self.cmdline.cursor_pos == 0 && self.cmdline.content.is_empty() {
                    self.cmdline = CmdlineState::default();
                    self.mode = Mode::Normal;
                } else if self.cmdline.cursor_pos > 0 {
                    self.cmdline.content.remove(self.cmdline.cursor_pos - 1);
                    self.cmdline.cursor_pos -= 1;
                }
            }
            Key::Delete => {
                if self.cmdline.cursor_pos < self.cmdline.content.len() {
                    self.cmdline.content.remove(self.cmdline.cursor_pos);
                }
            }
            Key::Char(c) => {
                self.cmdline.content.insert(self.cmdline.cursor_pos, *c);
                self.cmdline.cursor_pos += 1;
            }
            _ => {}
        }
        self.sequence += 1;
    }

    fn cmdline_delete_word_backward(&mut self) {
        let pos = self.cmdline.cursor_pos;
        if pos == 0 { return; }
        let bytes = self.cmdline.content.as_bytes();
        let mut i = pos;
        while i > 0 && bytes[i - 1] == b' ' { i -= 1; }
        while i > 0 && bytes[i - 1] != b' ' { i -= 1; }
        self.cmdline.content.drain(i..pos);
        self.cmdline.cursor_pos = i;
    }

    fn cmdline_delete_to_start(&mut self) {
        let pos = self.cmdline.cursor_pos;
        if pos == 0 { return; }
        self.cmdline.content.drain(..pos);
        self.cmdline.cursor_pos = 0;
    }

    pub(crate) fn activate_cmdline(&mut self, kind: CommandKind) {
        let prefix = match kind {
            CommandKind::Ex => ":", CommandKind::SearchForward => "/",
            CommandKind::SearchBackward => "?",
        };
        self.cmdline = CmdlineState {
            prefix: prefix.to_string(), content: String::new(), cursor_pos: 0, active: true,
        };
    }

    fn dispatch_cmdline(&mut self, kind: CommandKind, content: &str) {
        match kind {
            CommandKind::Ex => {
                self.registers.set_readonly(':', content.to_string());
                let action = parse_ex_command(content);
                self.apply_action(action);
            }
            CommandKind::SearchForward => self.execute_search(content, SearchDirection::Forward),
            CommandKind::SearchBackward => self.execute_search(content, SearchDirection::Backward),
        }
    }

    fn execute_search(&mut self, pattern: &str, dir: SearchDirection) {
        if pattern.is_empty() { self.jump_to_match(dir); return; }
        self.registers.set_readonly('/', pattern.to_string());
        if self.search.set_pattern(pattern, dir).is_ok() { self.jump_to_match(dir); }
    }

    pub(crate) fn jump_to_match(&mut self, dir: SearchDirection) {
        let wid = self.focus.focused;
        let win = match self.windows.get(&wid) { Some(w) => w, None => return };
        let buf_id = match win.content { ContentKind::Buffer(id) => id, _ => return };
        let buf = match self.buffers.get(&buf_id) { Some(b) => b, None => return };
        let (row, col) = (win.cursor.line, win.cursor.col);
        let found = match dir {
            SearchDirection::Forward => self.search.find_next(buf, row, col),
            SearchDirection::Backward => self.search.find_prev(buf, row, col),
        };
        if let Some((r, c)) = found {
            let win = self.windows.get_mut(&wid).unwrap();
            win.cursor.line = r; win.cursor.col = c;
        }
    }
}
