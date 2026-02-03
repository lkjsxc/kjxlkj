use kjxlkj_core_edit::Edit;
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::BufferText;
use kjxlkj_core_types::{BufferId, BufferVersion, CursorPos, Mode, WindowId};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, WindowSnapshot};
use kjxlkj_core_undo::UndoStack;

#[derive(Clone, Debug)]
pub enum CoreAction {
    SetMode(Mode),
    InsertText(String),
}

#[derive(Clone, Debug)]
struct BufferState {
    id: BufferId,
    version: BufferVersion,
    name: String,
    path: Option<String>,
    modified: bool,
    text: BufferText,
}

#[derive(Clone, Debug)]
struct WindowState {
    id: WindowId,
    buffer_id: BufferId,
    cursor: CursorPos,
    viewport_top: usize,
    viewport_left: usize,
}

#[derive(Clone, Debug)]
pub struct EditorState {
    mode: ModeState,
    active_window: WindowId,
    buffers: Vec<BufferState>,
    windows: Vec<WindowState>,
    undo: UndoStack,
    status: String,
    cmdline: Option<String>,
}

impl Default for EditorState {
    fn default() -> Self {
        let buf_id = BufferId::new(1);
        let win_id = WindowId::new(1);
        Self {
            mode: ModeState::default(),
            active_window: win_id,
            buffers: vec![BufferState {
                id: buf_id,
                version: BufferVersion::new(0),
                name: "[No Name]".to_string(),
                path: None,
                modified: false,
                text: BufferText::default(),
            }],
            windows: vec![WindowState {
                id: win_id,
                buffer_id: buf_id,
                cursor: CursorPos::default(),
                viewport_top: 0,
                viewport_left: 0,
            }],
            undo: UndoStack::default(),
            status: String::new(),
            cmdline: None,
        }
    }
}

impl EditorState {
    pub fn mode(&self) -> Mode {
        self.mode.mode()
    }

    pub fn apply(&mut self, action: CoreAction) {
        match action {
            CoreAction::SetMode(m) => {
                self.mode.set_mode(m);
                self.cmdline = match m {
                    Mode::Command => Some(String::new()),
                    _ => None,
                };
            }
            CoreAction::InsertText(text) => {
                if self.mode() != Mode::Insert && self.mode() != Mode::Replace {
                    return;
                }
                let (buf, win) = match self.active_buf_and_win_mut() {
                    Some(v) => v,
                    None => return,
                };
                let at = win.cursor.col;
                if buf.text.insert(at, &text).is_ok() {
                    buf.modified = true;
                    buf.version = buf.version.next();
                    let edit = Edit::Insert { at, text };
                    self.undo.push_transaction(vec![edit]);
                }
            }
        }
    }

    pub fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            mode: self.mode(),
            active_window: self.active_window,
            windows: self.windows.iter().map(|w| WindowSnapshot {
                id: w.id,
                buffer_id: w.buffer_id,
                cursor: w.cursor,
                viewport_top: w.viewport_top,
                viewport_left: w.viewport_left,
            }).collect(),
            buffers: self.buffers.iter().map(|b| BufferSnapshot {
                id: b.id,
                version: b.version,
                name: b.name.clone(),
                path: b.path.clone(),
                modified: b.modified,
                lines: (0..b.text.line_count()).filter_map(|i| b.text.line(i)).collect(),
            }).collect(),
            status: self.status.clone(),
            cmdline: self.cmdline.clone(),
        }
    }

    fn active_buf_and_win_mut(&mut self) -> Option<(&mut BufferState, &mut WindowState)> {
        let win_idx = self.windows.iter().position(|w| w.id == self.active_window)?;
        let buf_id = self.windows.get(win_idx)?.buffer_id;
        let buf_idx = self.buffers.iter().position(|b| b.id == buf_id)?;
        let (left, right) = self.windows.split_at_mut(win_idx);
        let win = if win_idx < left.len() { return None } else { &mut right[0] };
        let buf = &mut self.buffers[buf_idx];
        Some((buf, win))
    }
}
