//! Command-line mode key handling, ex command execution, and service responses.

use kjxlkj_core_types::{Mode, ServiceResponse};

use super::ex_dispatch::{self, ExResult};
use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn process_cmdline_key(&mut self, key: &kjxlkj_core_types::Key) {
        use kjxlkj_core_types::KeyCode;
        match &key.code {
            KeyCode::Esc => {
                self.change_mode(Mode::Normal);
            }
            KeyCode::Enter => {
                let cmd = self.cmdline.content.clone();
                let is_search = matches!(
                    self.mode,
                    Mode::Command(kjxlkj_core_types::CommandKind::SearchForward)
                        | Mode::Command(kjxlkj_core_types::CommandKind::SearchBackward)
                );
                self.change_mode(Mode::Normal);
                if is_search {
                    self.search.pattern = cmd;
                    self.search.active = true;
                } else {
                    self.execute_ex(&cmd);
                }
            }
            KeyCode::Backspace => {
                if self.cmdline.cursor_pos > 0 {
                    self.cmdline.cursor_pos -= 1;
                    self.cmdline.content.remove(self.cmdline.cursor_pos);
                }
            }
            KeyCode::Char(c) => {
                self.cmdline.content.insert(self.cmdline.cursor_pos, *c);
                self.cmdline.cursor_pos += 1;
            }
            _ => {}
        }
    }

    pub(crate) fn execute_ex(&mut self, cmd: &str) {
        self.registers.set_last_command(cmd.to_string());
        match ex_dispatch::parse_ex_command(cmd) {
            ExResult::Write(path) => {
                self.do_write(path);
            }
            ExResult::Quit => self.try_quit(false),
            ExResult::ForceQuit => self.try_quit(true),
            ExResult::WriteQuit => {
                self.do_write(None);
                self.try_quit(false);
            }
            ExResult::QuitAll => self.try_quit(false),
            ExResult::ForceQuitAll => self.try_quit(true),
            ExResult::Edit(path) => {
                self.notify_info(&format!("Edit: {path} (pending FS service)"));
            }
            ExResult::Split(path) => self.do_split(false, path),
            ExResult::VSplit(path) => self.do_split(true, path),
            ExResult::BNext => self.switch_buffer_next(),
            ExResult::BPrev => self.switch_buffer_prev(),
            ExResult::BDelete => {}
            ExResult::Set(opts) => self.apply_set_option(&opts),
            ExResult::Terminal => self.do_terminal_open(),
            ExResult::Explorer => self.do_explorer_toggle(),
            ExResult::Message(msg) => self.notify_info(&msg),
            ExResult::Error(msg) => self.notify_error(&msg),
            ExResult::Noop => {}
        }
    }

    /// Write the current buffer to disk.
    pub(crate) fn do_write(&mut self, path: Option<String>) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let buf = match self.buffers.get_mut(&buf_id) {
            Some(b) => b,
            None => return,
        };
        let target = path
            .map(std::path::PathBuf::from)
            .or_else(|| buf.path.clone());
        let target = match target {
            Some(p) => p,
            None => {
                self.notify_error("No file name");
                return;
            }
        };
        let content = buf.to_string_content();
        match std::fs::write(&target, &content) {
            Ok(()) => {
                buf.modified = false;
                if buf.path.is_none() {
                    buf.path = Some(target.clone());
                }
                self.notify_info(&format!("Written: {}", target.display()));
            }
            Err(e) => {
                self.notify_error(&format!("Write failed: {e}"));
            }
        }
    }

    pub(crate) fn try_quit(&mut self, force: bool) {
        if !force && self.buffers.any_modified() {
            self.notify_error("No write since last change (add ! to override)");
            return;
        }
        self.quit_requested = true;
    }

    pub(crate) fn process_service_response(&mut self, resp: ServiceResponse) {
        match resp {
            ServiceResponse::FileRead { path, content } => {
                self.open_file(path, content);
            }
            ServiceResponse::FileWritten { path } => {
                self.notify_info(&format!("Written: {}", path.display()));
            }
            ServiceResponse::TerminalOutput { id, data } => {
                tracing::debug!("Terminal {id:?} output: {} bytes", data.len());
            }
            ServiceResponse::TerminalExited { id } => {
                tracing::info!("Terminal {id:?} exited");
            }
        }
    }
}
