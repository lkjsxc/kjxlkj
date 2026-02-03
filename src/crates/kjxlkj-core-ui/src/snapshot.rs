//! Editor state snapshot for rendering.

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::{
    BufferMeta, BufferVersion, Cursor, CursorStyle, Mode, Range, Viewport,
    WindowId,
};

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Active window snapshot.
    pub active_window: WindowSnapshot,
    /// Mode information.
    pub mode: Mode,
    /// Status line content.
    pub status: StatusSnapshot,
    /// Command line content (if in command mode).
    pub command_line: Option<String>,
    /// Status message.
    pub message: Option<StatusMessage>,
}

/// Window snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnapshot {
    /// Window ID.
    pub id: WindowId,
    /// Buffer metadata.
    pub buffer_meta: BufferMeta,
    /// Visible lines (pre-rendered text).
    pub lines: Vec<LineSnapshot>,
    /// Cursor position.
    pub cursor: Cursor,
    /// Cursor style.
    pub cursor_style: CursorStyle,
    /// Viewport state.
    pub viewport: Viewport,
    /// Visual selection range (if any).
    pub selection: Option<Range>,
}

/// Line snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSnapshot {
    /// Line number (1-indexed for display).
    pub number: u32,
    /// Line text content.
    pub text: String,
    /// Whether this line is the cursor line.
    pub is_cursor_line: bool,
}

/// Status line snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSnapshot {
    /// Left-aligned content (mode, file info).
    pub left: String,
    /// Right-aligned content (position, encoding).
    pub right: String,
}

/// Status message to display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    /// Message text.
    pub text: String,
    /// Message level.
    pub level: MessageLevel,
}

/// Message severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

impl EditorSnapshot {
    /// Create a minimal snapshot for testing.
    pub fn minimal() -> Self {
        Self {
            active_window: WindowSnapshot {
                id: kjxlkj_core_types::WindowId::new(),
                buffer_meta: BufferMeta {
                    id: kjxlkj_core_types::BufferId::new(),
                    name: kjxlkj_core_types::BufferName::new("[No Name]"),
                    path: None,
                    modified: false,
                    version: BufferVersion::new(0),
                    line_ending: kjxlkj_core_types::LineEnding::Lf,
                },
                lines: vec![],
                cursor: Cursor::default(),
                cursor_style: CursorStyle::Block,
                viewport: Viewport::default(),
                selection: None,
            },
            mode: Mode::Normal,
            status: StatusSnapshot {
                left: "NORMAL".to_string(),
                right: "1:1".to_string(),
            },
            command_line: None,
            message: None,
        }
    }
}
