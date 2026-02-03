#![forbid(unsafe_code)]

use std::collections::HashMap;

use kjxlkj_core_mode::ModeState;
use kjxlkj_core_text::BufferText;
use kjxlkj_core_types::{BufferId, BufferVersion, CursorPos, WindowId};
use kjxlkj_core_undo::UndoStack;

mod cmdline;
mod edit;
mod event;
mod keys;
mod movement;
mod open_line;
mod state;
mod undo;

#[derive(Clone, Debug)]
enum PendingIo {
    OpenFile { buffer_id: BufferId, path: String },
    WriteFile {
        buffer_id: BufferId,
        path: String,
        expected_version: BufferVersion,
        quit_after: bool,
    },
    TerminalRun { command: String },
}

#[derive(Clone, Copy, Debug)]
enum PendingNormal {
    Delete,
    Yank,
}

#[derive(Clone, Debug)]
struct BufferState {
    id: BufferId,
    version: BufferVersion,
    name: String,
    path: Option<String>,
    modified: bool,
    saved_text: String,
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
    next_request_id: u64,
    buffers: Vec<BufferState>,
    windows: Vec<WindowState>,
    undo: UndoStack,
    yank: String,
    status: String,
    cmdline: Option<String>,
    pending_normal: Option<PendingNormal>,
    pending_io: HashMap<u64, PendingIo>,
    visual_anchor: Option<CursorPos>,
    term_size: Option<(u16, u16)>,
}
