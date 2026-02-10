//! Editor snapshot types for the render pipeline.

use kjxlkj_core_types::{BufferId, Mode, TerminalId, WindowId};
use ropey::Rope;
use std::collections::HashMap;
use std::path::PathBuf;

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    pub sequence: u64,
    pub tabs: Vec<TabSnapshot>,
    pub active_tab: usize,
    pub buffers: HashMap<BufferId, BufferSnapshot>,
    pub terminals: HashMap<TerminalId, TerminalSnapshot>,
    pub mode: Mode,
    pub cmdline: CmdlineState,
    pub notifications: Vec<Notification>,
    pub search: SearchState,
    pub terminal_size: (u16, u16),
}

/// A tab page snapshot.
#[derive(Debug, Clone)]
pub struct TabSnapshot {
    pub windows: Vec<WindowSnapshot>,
    pub active_window: usize,
    pub layout: LayoutNode,
}

/// Layout tree node.
#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf(WindowId),
    Horizontal(Vec<LayoutChild>),
    Vertical(Vec<LayoutChild>),
}

/// A child in a layout split with weight.
#[derive(Debug, Clone)]
pub struct LayoutChild {
    pub node: LayoutNode,
    pub weight: f32,
}

/// Window snapshot.
#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    pub id: WindowId,
    pub content: WindowContent,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub top_line: usize,
    pub left_col: usize,
    pub width: u16,
    pub height: u16,
    pub wrap: bool,
    pub line_numbers: bool,
}

/// What a window displays.
#[derive(Debug, Clone)]
pub enum WindowContent {
    Buffer(BufferId),
    Terminal(TerminalId),
    Explorer,
}

/// Buffer snapshot for rendering.
#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub id: BufferId,
    pub version: u64,
    pub content: Rope,
    pub line_count: usize,
    pub path: Option<PathBuf>,
    pub modified: bool,
    pub name: String,
}

/// Terminal screen snapshot.
#[derive(Debug, Clone)]
pub struct TerminalSnapshot {
    pub id: TerminalId,
    pub cells: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub rows: u16,
    pub cols: u16,
}

/// A single terminal cell.
#[derive(Debug, Clone, Default)]
pub struct TerminalCell {
    pub grapheme: String,
    pub width: u8,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// Color representation.
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub const fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
}

/// Command-line state.
#[derive(Debug, Clone, Default)]
pub struct CmdlineState {
    pub visible: bool,
    pub prefix: String,
    pub content: String,
    pub cursor_pos: usize,
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

/// Search state.
#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub pattern: String,
    pub active: bool,
    pub forward: bool,
}
