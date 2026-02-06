//! UI model types — layout, status line, command line.

use kjxlkj_core_types::{BufferId, Mode, Size, WindowId};

/// Represents the visible viewport of a window.
#[derive(Debug, Clone)]
pub struct Viewport {
    pub window_id: WindowId,
    pub buffer_id: BufferId,
    /// Top-left line visible in this viewport.
    pub top_line: usize,
    /// Number of visible lines.
    pub height: u16,
    /// Cursor line (relative to buffer).
    pub cursor_line: usize,
    /// Cursor column.
    pub cursor_col: usize,
}

/// Status-line data for a single window.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    pub mode: String,
    pub file_name: String,
    pub modified: bool,
    pub line: usize,
    pub col: usize,
    pub total_lines: usize,
}

/// Command-line state (the `:` prompt at the bottom).
#[derive(Debug, Clone, Default)]
pub struct CommandLine {
    pub content: String,
    pub cursor_pos: usize,
    pub visible: bool,
}

/// A message to display in the message area.
#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
    pub kind: MessageKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Warning,
    Error,
}

/// The complete UI model snapshot used by the renderer.
#[derive(Debug, Clone)]
pub struct UiModel {
    pub size: Size,
    pub viewports: Vec<Viewport>,
    pub status_lines: Vec<StatusLine>,
    pub command_line: CommandLine,
    pub message: Option<Message>,
    pub current_mode: Mode,
}

impl UiModel {
    pub fn empty(size: Size) -> Self {
        Self {
            size,
            viewports: Vec::new(),
            status_lines: Vec::new(),
            command_line: CommandLine::default(),
            message: None,
            current_mode: Mode::Normal,
        }
    }
}
