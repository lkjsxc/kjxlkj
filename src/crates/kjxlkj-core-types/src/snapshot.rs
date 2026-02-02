//! Snapshot types for kjxlkj editor state.
//!
//! Snapshots are immutable views of editor state used for rendering.

use crate::{
    buffer::BufferInfo,
    cursor::{Cursor, Selection},
    ids::{BufferId, WindowId},
    mode::Mode,
};
use serde::{Deserialize, Serialize};

/// Snapshot of the entire editor state for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Current mode.
    pub mode: Mode,
    /// Active window.
    pub active_window: WindowId,
    /// Window snapshots.
    pub windows: Vec<WindowSnapshot>,
    /// Command line content (if in command mode).
    pub command_line: Option<CommandLineSnapshot>,
    /// Status line info.
    pub status: StatusSnapshot,
    /// Optional message to display.
    pub message: Option<MessageSnapshot>,
}

/// Snapshot of a window's state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnapshot {
    /// Window identifier.
    pub id: WindowId,
    /// Buffer displayed in this window.
    pub buffer_id: BufferId,
    /// Buffer info for display.
    pub buffer_info: BufferInfo,
    /// Cursor position.
    pub cursor: Cursor,
    /// Visual selection (if in visual mode).
    pub selection: Option<Selection>,
    /// First visible line (scroll position).
    pub top_line: usize,
    /// Window dimensions.
    pub dimensions: WindowDimensions,
    /// Visible line range references (start, end).
    pub visible_range: (usize, usize),
}

/// Window dimensions and position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct WindowDimensions {
    /// X position (column).
    pub x: u16,
    /// Y position (row).
    pub y: u16,
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
}

impl WindowDimensions {
    /// Creates new window dimensions.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the number of visible text lines.
    pub fn text_height(&self) -> u16 {
        self.height.saturating_sub(1) // Reserve 1 for status line
    }
}

/// Command line state snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLineSnapshot {
    /// The prompt character (: / ? etc.).
    pub prompt: char,
    /// Current command text.
    pub content: String,
    /// Cursor position within command.
    pub cursor_pos: usize,
    /// Completion candidates if any.
    pub completions: Vec<String>,
    /// Selected completion index.
    pub selected_completion: Option<usize>,
}

/// Status line snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusSnapshot {
    /// Mode indicator text.
    pub mode_text: String,
    /// File name or buffer name.
    pub file_name: String,
    /// File flags (modified, readonly, etc.).
    pub file_flags: String,
    /// Cursor position text (line:col).
    pub position: String,
    /// File type or language.
    pub file_type: String,
    /// Encoding.
    pub encoding: String,
    /// Percentage through file.
    pub percentage: String,
}

/// Message to display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSnapshot {
    /// The message text.
    pub text: String,
    /// Message severity level.
    pub level: MessageLevel,
}

/// Message severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum MessageLevel {
    /// Normal informational message.
    #[default]
    Info,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}

impl MessageSnapshot {
    /// Creates an info message.
    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: MessageLevel::Info,
        }
    }

    /// Creates a warning message.
    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: MessageLevel::Warning,
        }
    }

    /// Creates an error message.
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: MessageLevel::Error,
        }
    }
}
