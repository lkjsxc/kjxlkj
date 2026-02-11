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
        &mut self, key: &Key, _mods: &KeyModifiers, kind: CommandKind,
    ) {
        match key {
            Key::Escape => {
                self.cmdline = CmdlineState::default();
                self.mode = Mode::Normal;
            }
            Key::Enter => {
                let content = self.cmdline.content.clone();
                self.cmdline = CmdlineState::default();
                self.mode = Mode::Normal;
                self.dispatch_cmdline(kind, &content);
            }
            Key::Backspace => {
                if self.cmdline.content.is_empty() {
                    self.cmdline = CmdlineState::default();
                    self.mode = Mode::Normal;
                } else {
                    self.cmdline.content.pop();
                    self.cmdline.cursor_pos =
                        self.cmdline.cursor_pos.saturating_sub(1);
                }
            }
            Key::Char(c) => {
                self.cmdline.content.push(*c);
                self.cmdline.cursor_pos += 1;
            }
            _ => {}
        }
        self.sequence += 1;
    }

    /// Activate the command line with appropriate prefix.
    pub(crate) fn activate_cmdline(&mut self, kind: CommandKind) {
        let prefix = match kind {
            CommandKind::Ex => ":",
            CommandKind::SearchForward => "/",
            CommandKind::SearchBackward => "?",
        };
        self.cmdline = CmdlineState {
            prefix: prefix.to_string(),
            content: String::new(),
            cursor_pos: 0,
            active: true,
        };
    }

    /// Dispatch a completed command-line entry.
    fn dispatch_cmdline(
        &mut self,
        kind: CommandKind,
        content: &str,
    ) {
        match kind {
            CommandKind::Ex => {
                let action = parse_ex_command(content);
                self.apply_action(action);
            }
            CommandKind::SearchForward => {
                self.execute_search(
                    content,
                    SearchDirection::Forward,
                );
            }
            CommandKind::SearchBackward => {
                self.execute_search(
                    content,
                    SearchDirection::Backward,
                );
            }
        }
    }

    /// Execute a search and move cursor to first match.
    fn execute_search(
        &mut self,
        pattern: &str,
        dir: SearchDirection,
    ) {
        if pattern.is_empty() {
            // Repeat last search.
            self.jump_to_match(dir);
            return;
        }
        if self.search.set_pattern(pattern, dir).is_ok() {
            self.jump_to_match(dir);
        }
    }

    /// Jump cursor to next/prev search match.
    pub(crate) fn jump_to_match(
        &mut self,
        dir: SearchDirection,
    ) {
        let wid = self.focus.focused;
        let win = match self.windows.get(&wid) {
            Some(w) => w,
            None => return,
        };
        let buf_id = match win.content {
            ContentKind::Buffer(id) => id,
            _ => return,
        };
        let buf = match self.buffers.get(&buf_id) {
            Some(b) => b,
            None => return,
        };
        let (row, col) =
            (win.cursor.line, win.cursor.col);
        let found = match dir {
            SearchDirection::Forward => {
                self.search.find_next(buf, row, col)
            }
            SearchDirection::Backward => {
                self.search.find_prev(buf, row, col)
            }
        };
        if let Some((r, c)) = found {
            let win = self.windows.get_mut(&wid).unwrap();
            win.cursor.line = r;
            win.cursor.col = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn ex_quit_from_cmdline() {
        let mut s = EditorState::new(80, 24);
        s.mode = Mode::Command(CommandKind::Ex);
        s.activate_cmdline(CommandKind::Ex);
        let m = KeyModifiers::default();
        s.handle_command_input(&Key::Char('q'), &m, CommandKind::Ex);
        assert_eq!(s.cmdline.content, "q");
        s.handle_command_input(&Key::Enter, &m, CommandKind::Ex);
        assert!(s.quit_requested);
        assert_eq!(s.mode, Mode::Normal);
    }

    #[test]
    fn search_forward_moves_cursor() {
        let mut s = EditorState::new(80, 24);
        s.buffers.get_mut(&BufferId(0)).unwrap()
            .insert(0, 0, "hello world").unwrap();
        s.mode = Mode::Command(CommandKind::SearchForward);
        s.activate_cmdline(CommandKind::SearchForward);
        let m = KeyModifiers::default();
        for c in "world".chars() {
            s.handle_command_input(&Key::Char(c), &m, CommandKind::SearchForward);
        }
        s.handle_command_input(&Key::Enter, &m, CommandKind::SearchForward);
        assert_eq!(s.windows.get(&s.focus.focused).unwrap().cursor.col, 6);
    }

    #[test]
    fn escape_cancels_cmdline() {
        let mut s = EditorState::new(80, 24);
        s.mode = Mode::Command(CommandKind::Ex);
        s.activate_cmdline(CommandKind::Ex);
        let m = KeyModifiers::default();
        s.handle_command_input(&Key::Char('w'), &m, CommandKind::Ex);
        s.handle_command_input(&Key::Escape, &m, CommandKind::Ex);
        assert_eq!(s.mode, Mode::Normal);
        assert!(!s.cmdline.active);
    }

    #[test]
    fn backspace_on_empty_exits() {
        let mut s = EditorState::new(80, 24);
        s.mode = Mode::Command(CommandKind::Ex);
        s.activate_cmdline(CommandKind::Ex);
        let m = KeyModifiers::default();
        s.handle_command_input(&Key::Backspace, &m, CommandKind::Ex);
        assert_eq!(s.mode, Mode::Normal);
    }
}
