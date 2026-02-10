//! Main editor state.

use crate::{BufferList, WindowTree};
use kjxlkj_core_mode::ModeState;
use kjxlkj_core_types::{Mode, Rect, WindowContent};
use kjxlkj_core_ui::{
    BufferSnapshot, CmdlineState, EditorSnapshot, LayoutNode as SnapLayout, TabSnapshot,
    WindowContentSnapshot, WindowSnapshot,
};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main editor state.
#[derive(Debug)]
pub struct EditorState {
    /// Buffer list.
    pub buffers: BufferList,
    /// Window tree.
    pub windows: WindowTree,
    /// Mode state.
    pub mode: ModeState,
    /// Snapshot sequence counter.
    snapshot_seq: u64,
    /// Terminal dimensions.
    pub terminal_size: (u16, u16),
    /// Quit flag.
    pub should_quit: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    /// Create a new editor state.
    pub fn new() -> Self {
        let mut state = Self {
            buffers: BufferList::new(),
            windows: WindowTree::new(),
            mode: ModeState::new(),
            snapshot_seq: 0,
            terminal_size: (80, 24),
            should_quit: false,
        };

        // Create initial scratch buffer and window.
        let buffer_id = state.buffers.add_scratch();
        state.windows.add_buffer_window(buffer_id);

        state
    }

    /// Create a new editor state with terminal size.
    pub fn with_size(cols: u16, rows: u16) -> Self {
        let mut state = Self::new();
        state.terminal_size = (cols, rows);
        state
    }

    /// Set terminal dimensions.
    pub fn set_terminal_size(&mut self, cols: u16, rows: u16) {
        self.terminal_size = (cols, rows);
    }

    /// Open a buffer with optional path and content.
    pub fn open_buffer(&mut self, path: Option<PathBuf>, content: Option<String>) {
        let buffer_id = if let Some(content) = content {
            self.buffers.add_with_content(&content)
        } else {
            self.buffers.add_scratch()
        };

        if let Some(path) = path {
            if let Some(buffer) = self.buffers.get_mut(buffer_id) {
                buffer.meta.path = Some(path);
            }
        }

        // Focus the new buffer in current window or create a new window.
        if let Some(window) = self.windows.focused_mut() {
            window.content = kjxlkj_core_types::WindowContent::Buffer(buffer_id);
        } else {
            self.windows.add_buffer_window(buffer_id);
        }
    }

    /// Request quit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Request quit without saving (force).
    pub fn quit_force(&mut self) {
        self.should_quit = true;
        // Mark all buffers as not modified to skip save prompts.
    }

    /// Create a snapshot for rendering.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.snapshot_seq += 1;

        let mut buffers = HashMap::new();
        for id in self.buffers.ids() {
            if let Some(buffer) = self.buffers.get(id) {
                let lines: Vec<String> =
                    (0..buffer.line_count()).map(|i| buffer.line(i)).collect();
                buffers.insert(
                    id,
                    BufferSnapshot {
                        id,
                        version: buffer.version().0,
                        line_count: buffer.line_count(),
                        path: buffer
                            .meta
                            .path
                            .as_ref()
                            .map(|p| p.to_string_lossy().into_owned()),
                        modified: buffer.meta.modified,
                        lines,
                    },
                );
            }
        }

        // Build layout with window info.
        let layout = if let Some(window) = self.windows.focused() {
            let content = match window.content {
                WindowContent::Buffer(buffer_id) => WindowContentSnapshot::Buffer {
                    buffer_id,
                    top_line: window.viewport.top_line,
                    cursor_line: window.cursor.line,
                    cursor_grapheme: window.cursor.grapheme,
                },
                WindowContent::Terminal(terminal_id) => {
                    WindowContentSnapshot::Terminal { terminal_id }
                }
                WindowContent::Explorer(_) => WindowContentSnapshot::Explorer { selected_index: 0 },
            };
            SnapLayout::Leaf(WindowSnapshot {
                id: window.id,
                rect: Rect {
                    x: 0,
                    y: 0,
                    width: self.terminal_size.0,
                    height: self.terminal_size.1.saturating_sub(2),
                },
                content,
            })
        } else {
            SnapLayout::default()
        };

        let tab = TabSnapshot {
            layout,
            focused_window: self.windows.focused().map(|w| w.id),
        };

        EditorSnapshot {
            sequence: self.snapshot_seq,
            tabs: vec![tab],
            active_tab: 0,
            buffers,
            terminals: HashMap::new(),
            mode: self.mode.mode.clone(),
            cmdline: CmdlineState {
                prefix: match &self.mode.mode {
                    Mode::Command(kjxlkj_core_types::CommandKind::Ex) => Some(':'),
                    Mode::Command(kjxlkj_core_types::CommandKind::SearchForward) => Some('/'),
                    Mode::Command(kjxlkj_core_types::CommandKind::SearchBackward) => Some('?'),
                    _ => None,
                },
                content: self.mode.cmdline.clone(),
                cursor: self.mode.cmdline_cursor,
            },
            notifications: Vec::new(),
            terminal_size: self.terminal_size,
        }
    }
}
