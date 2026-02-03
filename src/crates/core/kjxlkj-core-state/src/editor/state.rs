use kjxlkj_core_types::{BufferId, BufferVersion, CursorPos, Mode, WindowId};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, WindowSnapshot};

use super::{BufferState, EditorState, PendingNormal, WindowState};

impl Default for EditorState {
    fn default() -> Self {
        let buf_id = BufferId::new(1);
        let win_id = WindowId::new(1);
        let version = BufferVersion::new(0);
        Self {
            mode: Default::default(),
            active_window: win_id,
            next_request_id: 1,
            buffers: vec![BufferState {
                id: buf_id,
                version,
                name: "[No Name]".to_string(),
                path: None,
                modified: false,
                saved_text: String::new(),
                text: Default::default(),
            }],
            windows: vec![WindowState {
                id: win_id,
                buffer_id: buf_id,
                cursor: Default::default(),
                viewport_top: 0,
                viewport_left: 0,
            }],
            undo: Default::default(),
            yank: String::new(),
            status: String::new(),
            cmdline: None,
            pending_normal: None,
            pending_io: Default::default(),
            visual_anchor: None,
            term_size: None,
        }
    }
}

impl EditorState {
    pub fn mode(&self) -> Mode {
        self.mode.mode()
    }

    pub fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            mode: self.mode(),
            active_window: self.active_window,
            windows: self
                .windows
                .iter()
                .map(|w| WindowSnapshot {
                    id: w.id,
                    buffer_id: w.buffer_id,
                    cursor: w.cursor,
                    viewport_top: w.viewport_top,
                    viewport_left: w.viewport_left,
                })
                .collect(),
            buffers: self
                .buffers
                .iter()
                .map(|b| BufferSnapshot {
                    id: b.id,
                    version: b.version,
                    name: b.name.clone(),
                    path: b.path.clone(),
                    modified: b.modified,
                    lines: (0..b.text.line_count()).filter_map(|i| b.text.line(i)).collect(),
                })
                .collect(),
            status: self.status.clone(),
            cmdline: self.cmdline.as_ref().map(|s| format!(":{s}")),
        }
    }

    pub(super) fn set_mode(&mut self, mode: Mode) {
        self.mode.set_mode(mode);
        self.pending_normal = None;
        self.visual_anchor = None;
        self.cmdline = match mode {
            Mode::Command => Some(String::new()),
            _ => None,
        };
    }

    pub(super) fn set_status(&mut self, msg: impl Into<String>) {
        self.status = msg.into();
    }

    pub(super) fn alloc_request_id(&mut self) -> u64 {
        let id = self.next_request_id;
        self.next_request_id = self.next_request_id.saturating_add(1);
        id
    }

    pub(super) fn set_pending_normal(&mut self, p: PendingNormal) {
        self.pending_normal = Some(p);
    }

    pub(super) fn take_pending_normal(&mut self) -> Option<PendingNormal> {
        self.pending_normal.take()
    }

    pub(super) fn active_indices(&self) -> Option<(usize, usize)> {
        let win_idx = self.windows.iter().position(|w| w.id == self.active_window)?;
        let buf_id = self.windows.get(win_idx)?.buffer_id;
        let buf_idx = self.buffers.iter().position(|b| b.id == buf_id)?;
        Some((buf_idx, win_idx))
    }

    pub(super) fn active_buf_idx(&self) -> Option<usize> {
        self.active_indices().map(|(b, _)| b)
    }

    pub(super) fn active_win_idx(&self) -> Option<usize> {
        self.active_indices().map(|(_, w)| w)
    }

    pub(super) fn active_window_cursor(&self) -> Option<CursorPos> {
        self.active_win_idx().map(|i| self.windows[i].cursor)
    }

    pub(super) fn set_term_size(&mut self, cols: u16, rows: u16) {
        self.term_size = Some((cols, rows));
    }

    pub(super) fn buffer_name_from_path(path: &str) -> String {
        std::path::Path::new(path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string())
    }

    pub(super) fn reset_cursor(&mut self) {
        let Some(win_idx) = self.active_win_idx() else {
            return;
        };
        let win = &mut self.windows[win_idx];
        win.cursor = CursorPos::default();
        win.viewport_top = 0;
        win.viewport_left = 0;
    }
}
