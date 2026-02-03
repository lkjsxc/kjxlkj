use kjxlkj_core_types::{BufferId, BufferVersion, CursorPos, Mode, WindowId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub version: BufferVersion,
    pub name: String,
    pub path: Option<String>,
    pub modified: bool,
    pub lines: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowSnapshot {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor: CursorPos,
    pub viewport_top: usize,
    pub viewport_left: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorSnapshot {
    pub mode: Mode,
    pub active_window: WindowId,
    pub windows: Vec<WindowSnapshot>,
    pub buffers: Vec<BufferSnapshot>,
    pub status: String,
    pub cmdline: Option<String>,
}

