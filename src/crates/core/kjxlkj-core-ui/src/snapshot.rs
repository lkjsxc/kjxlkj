//! Editor snapshot for rendering.

use kjxlkj_core_types::{BufferId, WindowId, TerminalId, Mode, Rect};
use std::collections::HashMap;

/// Editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Monotonic sequence number.
    pub sequence: u64,
    /// Tabs with layout info.
    pub tabs: Vec<TabSnapshot>,
    /// Active tab index.
    pub active_tab: usize,
    /// Buffer snapshots.
    pub buffers: HashMap<BufferId, BufferSnapshot>,
    /// Terminal snapshots.
    pub terminals: HashMap<TerminalId, TerminalSnapshot>,
    /// Current mode.
    pub mode: Mode,
    /// Command line state.
    pub cmdline: CmdlineState,
    /// Notifications.
    pub notifications: Vec<Notification>,
    /// Terminal dimensions.
    pub terminal_size: (u16, u16),
}

impl Default for EditorSnapshot {
    fn default() -> Self {
        Self {
            sequence: 0,
            tabs: vec![TabSnapshot::default()],
            active_tab: 0,
            buffers: HashMap::new(),
            terminals: HashMap::new(),
            mode: Mode::Normal,
            cmdline: CmdlineState::default(),
            notifications: Vec::new(),
            terminal_size: (80, 24),
        }
    }
}

/// Tab snapshot.
#[derive(Debug, Clone, Default)]
pub struct TabSnapshot {
    /// Layout tree root.
    pub layout: LayoutNode,
    /// Focused window ID.
    pub focused_window: Option<WindowId>,
}

/// Layout tree node.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// Leaf window.
    Leaf(WindowSnapshot),
    /// Horizontal container.
    Horizontal(Vec<LayoutNode>),
    /// Vertical container.
    Vertical(Vec<LayoutNode>),
}

impl Default for LayoutNode {
    fn default() -> Self {
        LayoutNode::Leaf(WindowSnapshot::default())
    }
}

/// Window snapshot.
#[derive(Debug, Clone, Default)]
pub struct WindowSnapshot {
    /// Window ID.
    pub id: WindowId,
    /// Window rectangle.
    pub rect: Rect,
    /// Window content.
    pub content: WindowContentSnapshot,
}

/// Window content snapshot.
#[derive(Debug, Clone)]
pub enum WindowContentSnapshot {
    /// Buffer content.
    Buffer {
        buffer_id: BufferId,
        top_line: usize,
        cursor_line: usize,
        cursor_grapheme: usize,
    },
    /// Explorer content.
    Explorer {
        selected_index: usize,
    },
    /// Terminal content.
    Terminal {
        terminal_id: TerminalId,
    },
}

impl Default for WindowContentSnapshot {
    fn default() -> Self {
        WindowContentSnapshot::Buffer {
            buffer_id: BufferId(0),
            top_line: 0,
            cursor_line: 0,
            cursor_grapheme: 0,
        }
    }
}

/// Buffer snapshot.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    /// Buffer ID.
    pub id: BufferId,
    /// Buffer version.
    pub version: u64,
    /// Line count.
    pub line_count: usize,
    /// Path if file-backed.
    pub path: Option<String>,
    /// Modified flag.
    pub modified: bool,
    /// Lines (for rendering).
    pub lines: Vec<String>,
}

/// Terminal snapshot.
#[derive(Debug, Clone)]
pub struct TerminalSnapshot {
    /// Terminal ID.
    pub id: TerminalId,
    /// Screen lines.
    pub lines: Vec<String>,
    /// Cursor position.
    pub cursor: (u16, u16),
}

/// Command line state.
#[derive(Debug, Clone, Default)]
pub struct CmdlineState {
    /// Prefix character.
    pub prefix: Option<char>,
    /// Content.
    pub content: String,
    /// Cursor position.
    pub cursor: usize,
}

/// Notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Message text.
    pub message: String,
    /// Severity level.
    pub level: NotificationLevel,
}

/// Notification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}
