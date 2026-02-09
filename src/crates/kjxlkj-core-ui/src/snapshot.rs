use kjxlkj_core_types::{BufferId, ContentSource, CursorPosition, LayoutNode, Mode, WindowId};
use ropey::Rope;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::theme::Theme;

/// Immutable snapshot of entire editor state for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Monotonic sequence number for stale detection.
    pub sequence: u64,
    /// Tab pages with layout trees.
    pub tabs: Vec<TabSnapshot>,
    /// Currently visible tab index.
    pub active_tab: usize,
    /// Buffer snapshots by ID.
    pub buffers: HashMap<BufferId, BufferSnapshot>,
    /// Current editing mode.
    pub mode: Mode,
    /// Command-line state.
    pub cmdline: CmdlineState,
    /// Active notifications.
    pub notifications: Vec<Notification>,
    /// Active search state.
    pub search: SearchState,
    /// Active color theme.
    pub theme: Theme,
    /// Total terminal dimensions.
    pub terminal_size: (u16, u16),
    /// Focused window ID.
    pub focused_window: WindowId,
    /// Popup completion menu (if active).
    pub popup_menu: Option<PopupMenu>,
}

/// Snapshot of a single tab page.
#[derive(Debug, Clone)]
pub struct TabSnapshot {
    /// Layout tree for this tab.
    pub layout: LayoutNode,
    /// Window snapshots by ID.
    pub windows: HashMap<WindowId, WindowSnapshot>,
}

/// Snapshot of a single window.
#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    pub window_id: WindowId,
    pub content: ContentSource,
    pub cursor: CursorPosition,
    /// Top visible line.
    pub top_line: usize,
    /// Left column offset (horizontal scroll).
    pub left_col: usize,
    /// Window area in terminal cells.
    pub area: WindowArea,
    /// Whether line numbers are shown.
    pub show_line_numbers: bool,
    /// Whether text wraps.
    pub wrap: bool,
    /// Visual selection range (anchor, cursor, kind) for rendering.
    pub visual_selection: Option<VisualSelection>,
}

/// Visual selection info for rendering.
#[derive(Debug, Clone)]
pub struct VisualSelection {
    pub anchor: CursorPosition,
    pub cursor: CursorPosition,
    pub kind: kjxlkj_core_types::VisualKind,
}

/// Window area rectangle in terminal coordinates.
#[derive(Debug, Clone, Copy)]
pub struct WindowArea {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// Snapshot of buffer text for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub version: kjxlkj_core_types::BufferVersion,
    pub content: Rope,
    pub line_count: usize,
    pub path: Option<PathBuf>,
    pub modified: bool,
    pub name: String,
}

/// Command-line state.
#[derive(Debug, Clone, Default)]
pub struct CmdlineState {
    /// Prefix character: ':', '/', '?'
    pub prefix: Option<char>,
    /// Current command-line content.
    pub content: String,
    /// Cursor position within content.
    pub cursor_pos: usize,
    /// Whether command line is active.
    pub active: bool,
    /// Wildmenu completion candidates (for display).
    pub completions: Vec<String>,
    /// Selected completion index.
    pub completion_index: Option<usize>,
}

/// Search state.
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    /// Current search pattern.
    pub pattern: Option<String>,
    /// Whether search is active.
    pub active: bool,
    /// Search direction: true = forward.
    pub forward: bool,
    /// Highlighted match positions (line, start_col, end_col) for hlsearch.
    pub highlight_ranges: Vec<(usize, usize, usize)>,
    /// Search match count: (current_match, total_matches).
    pub match_count: Option<(usize, usize)>,
    /// Search offset parsed from /pattern/e+N style syntax.
    pub offset: SearchOffset,
}

/// Search offset: /pattern/e+N, /pattern/+N, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchOffset {
    #[default]
    None,
    /// Line offset from match start.
    Lines(i32),
    /// Character offset from end of match.
    End(i32),
    /// Character offset from start of match.
    Start(i32),
}

/// Notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
}

/// Notification severity.
#[derive(Debug, Clone, Copy)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

/// Popup completion menu for rendering.
#[derive(Debug, Clone)]
pub struct PopupMenu {
    /// Candidate items to display.
    pub items: Vec<String>,
    /// Index of currently selected item (if any).
    pub selected: Option<usize>,
}
