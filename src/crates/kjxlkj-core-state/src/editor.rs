//! Core editor state: the single mutable owner of all state.

use std::collections::HashMap;
use std::path::PathBuf;

use kjxlkj_core_edit::RegisterSet;
use kjxlkj_core_mode::ModeDispatcher;
use kjxlkj_core_types::{BufferId, Mode, TerminalId};
use kjxlkj_core_ui::{
    BufferSnapshot, CmdlineState, EditorSnapshot, Notification, SearchState, TabSnapshot,
    WindowContent, WindowSnapshot,
};
use kjxlkj_core_undo::UndoTree;
use kjxlkj_input::ime::ImeComposition;

use crate::buffer_list::BufferList;
use crate::explorer::ExplorerState;
use crate::window_tree::{TabPage, Window, WindowTree};

/// The editor's complete mutable state.
pub struct EditorState {
    pub buffers: BufferList,
    pub windows: WindowTree,
    pub mode: Mode,
    pub dispatcher: ModeDispatcher,
    pub cmdline: CmdlineState,
    pub notifications: Vec<Notification>,
    pub search: SearchState,
    pub terminal_size: (u16, u16),
    pub quit_requested: bool,
    pub sequence: u64,
    pub registers: RegisterSet,
    pub pending_register: Option<char>,
    pub undo_trees: HashMap<BufferId, UndoTree>,
    pub explorer: ExplorerState,
    pub visual_anchor: Option<(usize, usize)>,
    pub replace_stack: Vec<Option<char>>,
    pub ime: ImeComposition,
    next_terminal_id: u64,
}

impl EditorState {
    /// Create initial editor state.
    pub fn new(cols: u16, rows: u16) -> Self {
        let mut state = Self {
            buffers: BufferList::new(),
            windows: WindowTree::new(),
            mode: Mode::Normal,
            dispatcher: ModeDispatcher::new(),
            cmdline: CmdlineState::default(),
            notifications: Vec::new(),
            search: SearchState::default(),
            terminal_size: (cols, rows),
            quit_requested: false,
            sequence: 0,
            registers: RegisterSet::new(),
            pending_register: None,
            undo_trees: HashMap::new(),
            explorer: ExplorerState::default(),
            visual_anchor: None,
            replace_stack: Vec::new(),
            ime: ImeComposition::new(),
            next_terminal_id: 1,
        };
        state.init_scratch();
        state
    }

    /// Initialize with a scratch buffer.
    fn init_scratch(&mut self) {
        let buf_id = self.buffers.create_scratch();
        let win_id = self.windows.next_window_id();
        let window = Window::new_buffer(win_id, buf_id);
        let tab = TabPage::new(window);
        self.windows.add_tab(tab);
        self.undo_trees.insert(buf_id, UndoTree::default());
    }

    /// Open a file.
    pub fn open_file(&mut self, path: PathBuf, content: String) {
        let buf_id = self.buffers.open_file(path, content);
        let win = self.windows.active_tab_mut().active_mut();
        win.content = WindowContent::Buffer(buf_id);
        win.cursor_line = 0;
        win.cursor_offset = 0;
        win.top_line = 0;
        self.undo_trees.entry(buf_id).or_default();
    }

    /// Get the active buffer id.
    pub fn active_buffer_id(&self) -> Option<BufferId> {
        let win = self.windows.active_tab().active();
        match &win.content {
            WindowContent::Buffer(id) => Some(*id),
            _ => None,
        }
    }

    /// Get a snapshot for rendering.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.sequence += 1;
        let mut buf_snapshots = HashMap::new();
        for (id, buf) in self.buffers.iter() {
            buf_snapshots.insert(
                *id,
                BufferSnapshot {
                    id: *id,
                    version: buf.version.0,
                    content: buf.snapshot_rope(),
                    line_count: buf.line_count(),
                    path: buf.path.clone(),
                    modified: buf.modified,
                    name: buf.name.to_string(),
                },
            );
        }

        let tabs: Vec<TabSnapshot> = self
            .windows
            .tabs
            .iter()
            .map(|tab| {
                let win_snaps: Vec<WindowSnapshot> = tab
                    .windows
                    .iter()
                    .map(|w| WindowSnapshot {
                        id: w.id,
                        content: w.content.clone(),
                        cursor_line: w.cursor_line,
                        cursor_col: w.cursor_offset,
                        top_line: w.top_line,
                        left_col: w.left_col,
                        width: self.terminal_size.0,
                        height: self.terminal_size.1.saturating_sub(2),
                        wrap: w.wrap,
                        line_numbers: w.line_numbers,
                    })
                    .collect();
                TabSnapshot {
                    windows: win_snaps,
                    active_window: tab.active_window,
                    layout: tab.layout.clone(),
                }
            })
            .collect();

        EditorSnapshot {
            sequence: self.sequence,
            tabs,
            active_tab: self.windows.active_tab,
            buffers: buf_snapshots,
            terminals: HashMap::new(),
            mode: self.mode.clone(),
            cmdline: self.cmdline.clone(),
            notifications: self.notifications.clone(),
            search: self.search.clone(),
            terminal_size: self.terminal_size,
        }
    }

    /// Allocate a new terminal id.
    pub fn next_terminal_id(&mut self) -> TerminalId {
        let id = TerminalId(self.next_terminal_id);
        self.next_terminal_id += 1;
        id
    }
}
