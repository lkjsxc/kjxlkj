use crate::Effect;

use super::{EditorState, PendingIo};

impl EditorState {
    pub(super) fn cmdline_push(&mut self, c: char) {
        let Some(s) = self.cmdline.as_mut() else {
            return;
        };
        s.push(c);
    }

    pub(super) fn cmdline_backspace(&mut self) {
        let Some(s) = self.cmdline.as_mut() else {
            return;
        };
        s.pop();
    }

    pub(super) fn cmdline_submit(&mut self) -> Vec<Effect> {
        let cmdline = self.cmdline.clone().unwrap_or_default();
        self.exec_ex_command(cmdline.trim())
    }

    fn exec_ex_command(&mut self, cmd: &str) -> Vec<Effect> {
        if cmd.is_empty() {
            return Vec::new();
        }
        if let Some(rest) = cmd.strip_prefix('!') {
            return self.exec_shell(rest.trim());
        }

        let mut parts = cmd.split_whitespace();
        let Some(head) = parts.next() else {
            return Vec::new();
        };

        match head {
            "q" | "quit" => self.exec_quit(false),
            "q!" | "quit!" => self.exec_quit(true),
            "qa" | "qall" => self.exec_quit(false),
            "qa!" | "qall!" => self.exec_quit(true),
            "w" | "write" => self.exec_write(parts.next(), false),
            "wa" | "wall" => self.exec_write(parts.next(), false),
            "wq" | "x" => self.exec_write(parts.next(), true),
            "e" | "edit" => self.exec_edit(parts.next(), false),
            "e!" | "edit!" => self.exec_edit(parts.next(), true),
            _ => {
                self.set_status(format!("not an editor command: {head}"));
                Vec::new()
            }
        }
    }

    fn exec_shell(&mut self, command: &str) -> Vec<Effect> {
        if command.is_empty() {
            self.set_status("empty shell command");
            return Vec::new();
        }
        let request_id = self.alloc_request_id();
        self.pending_io
            .insert(request_id, PendingIo::TerminalRun { command: command.to_string() });
        vec![Effect::TerminalRun { request_id, command: command.to_string() }]
    }

    fn exec_quit(&mut self, force: bool) -> Vec<Effect> {
        let modified = self
            .active_buf_idx()
            .and_then(|i| self.buffers.get(i).map(|b| b.modified))
            .unwrap_or(false);
        if modified && !force {
            self.set_status("No write since last change (add ! to override)");
            return Vec::new();
        }
        vec![Effect::Quit { force }]
    }

    fn exec_edit(&mut self, path: Option<&str>, force: bool) -> Vec<Effect> {
        let Some(path) = path else {
            self.set_status("edit: missing path");
            return Vec::new();
        };
        let (buf_id, modified) = match self.active_buf_idx().and_then(|i| self.buffers.get(i).map(|b| (b.id, b.modified))) {
            Some(v) => v,
            None => {
                self.set_status("no active buffer");
                return Vec::new();
            }
        };
        if modified && !force {
            self.set_status("No write since last change (use :e! to override)");
            return Vec::new();
        }
        let request_id = self.alloc_request_id();
        self.pending_io.insert(
            request_id,
            PendingIo::OpenFile {
                buffer_id: buf_id,
                path: path.to_string(),
            },
        );
        vec![Effect::FsRead { request_id, path: path.to_string() }]
    }

    fn exec_write(&mut self, path: Option<&str>, quit_after: bool) -> Vec<Effect> {
        let Some(buf_idx) = self.active_buf_idx() else {
            self.set_status("no active buffer");
            return Vec::new();
        };
        let buf = &self.buffers[buf_idx];

        let write_path = match (path, buf.path.as_deref()) {
            (Some(p), _) => p.to_string(),
            (None, Some(p)) => p.to_string(),
            (None, None) => {
                self.set_status("write: no file name");
                return Vec::new();
            }
        };

        let contents = self.buffers[buf_idx].text.to_string();
        let expected_version = self.buffers[buf_idx].version;
        let buf_id = self.buffers[buf_idx].id;
        let request_id = self.alloc_request_id();

        self.pending_io.insert(
            request_id,
            PendingIo::WriteFile {
                buffer_id: buf_id,
                path: write_path.clone(),
                expected_version,
                quit_after,
            },
        );

        vec![Effect::FsWrite {
            request_id,
            path: write_path,
            contents,
        }]
    }
}
