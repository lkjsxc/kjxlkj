use kjxlkj_core_text::BufferText;
use kjxlkj_core_types::CursorPos;

use crate::{CoreEvent, Effect, ServiceResult};

use super::{EditorState, PendingIo};

impl EditorState {
    pub fn handle_event(&mut self, event: CoreEvent) -> Vec<Effect> {
        match event {
            CoreEvent::Resize { cols, rows } => {
                self.set_term_size(cols, rows);
                Vec::new()
            }
            CoreEvent::Service(r) => self.handle_service_result(r),
            CoreEvent::Key(k) => self.handle_key(k),
        }
    }

    fn handle_service_result(&mut self, result: ServiceResult) -> Vec<Effect> {
        match result {
            ServiceResult::FsReadOk {
                request_id,
                path: result_path,
                contents,
            } => match self.pending_io.remove(&request_id) {
                Some(PendingIo::OpenFile {
                    buffer_id,
                    path: expected_path,
                }) => {
                    if expected_path != result_path {
                        self.set_status(format!("opened {result_path} (unexpected path)"));
                    }
                    if let Some(buf) = self.buffers.iter_mut().find(|b| b.id == buffer_id) {
                        buf.text = BufferText::from_text(&contents);
                        buf.path = Some(result_path.clone());
                        buf.name = Self::buffer_name_from_path(&result_path);
                        buf.modified = false;
                        buf.saved_text = contents;
                        buf.version = buf.version.next();
                    }
                    self.reset_cursor();
                    self.set_status(format!("opened {result_path}"));
                    Vec::new()
                }
                _ => Vec::new(),
            },
            ServiceResult::FsWriteOk { request_id, path } => match self.pending_io.remove(&request_id) {
                Some(PendingIo::WriteFile {
                    buffer_id,
                    path: expected_path,
                    expected_version,
                    quit_after,
                }) => {
                    let mut effects = Vec::new();
                    let mut status = None;
                    if let Some(buf) = self.buffers.iter_mut().find(|b| b.id == buffer_id) {
                        if buf.version == expected_version {
                            buf.path = Some(path.clone());
                            buf.name = Self::buffer_name_from_path(&path);
                            buf.modified = false;
                            buf.saved_text = buf.text.to_string();
                            status = Some(format!("written {path}"));
                        } else {
                            status = Some(format!("written {path} (stale)"));
                        }
                        if expected_path != path {
                            status = Some(format!("written {path} (unexpected path)"));
                        }
                    }
                    if let Some(status) = status {
                        self.set_status(status);
                    }
                    if quit_after {
                        effects.push(Effect::Quit { force: false });
                    }
                    effects
                }
                _ => Vec::new(),
            },
            ServiceResult::FsError { request_id, message } => {
                self.pending_io.remove(&request_id);
                self.set_status(format!("fs error: {message}"));
                Vec::new()
            }
            ServiceResult::TerminalOk { request_id, output } => {
                if let Some(PendingIo::TerminalRun { command }) = self.pending_io.remove(&request_id) {
                    let out = output.lines().next().unwrap_or(&command).to_string();
                    self.set_status(out.trim_end().to_string());
                }
                Vec::new()
            }
            ServiceResult::TerminalError { request_id, message } => {
                self.pending_io.remove(&request_id);
                self.set_status(format!("terminal error: {message}"));
                Vec::new()
            }
        }
    }
}

impl EditorState {
    pub(super) fn set_visual_anchor(&mut self, anchor: CursorPos) {
        self.visual_anchor = Some(anchor);
    }
}
