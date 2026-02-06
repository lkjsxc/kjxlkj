//! Editor state aggregation — ties together all core sub-crates.

mod autocommands;   mod buffer_state;       mod commands;
mod commands_buffer; mod commands_config;   mod commands_config_map;
mod commands_display; mod commands_file;    mod commands_line_ops;
mod commands_nav;   mod commands_range;     mod commands_range_ops;
mod commands_substitute; mod config;        mod dispatch;
mod dispatch_case;  mod dispatch_cmdline;   mod dispatch_editing;
mod dispatch_editing_extra; mod dispatch_insert; mod dispatch_jumps;
mod dispatch_macros; mod dispatch_marks;    mod dispatch_misc;
mod dispatch_navigation; mod dispatch_operators; mod dispatch_search;
mod dispatch_windows; mod dispatch_yank_paste;
mod mappings;       mod quickfix;           mod registers;
mod session;        mod session_macros;     pub mod scripting;
mod window_state;

pub use autocommands::{AutoCmdTable, AutoEvent};
pub use buffer_state::BufferState;
pub use config::{execute_script, load_default_config};
pub use dispatch::dispatch_intent;
pub use dispatch_cmdline::handle_cmdline_key;
pub use mappings::{MappingMode, MappingTable};
pub use registers::RegisterFile;
pub use quickfix::{QuickfixEntry, QuickfixKind, QuickfixList};
pub use session::{AutoSaveConfig, RecentFiles, Session, SessionLayout, SplitDirection, SwapFile, UndoFile, Workspace};
pub use session_macros::{KeyModifiers, KeyStroke, Macro, MacroRecord, MacroStore};
pub use window_state::WindowState;

use std::collections::HashMap;
use kjxlkj_core_mode::{KeyParser, ModeState};
use kjxlkj_core_types::{BufferId, FindCharKind, Mode, Position, Range, Size, WindowId};

/// Top-level editor state that owns all buffers, windows, modes, registers.
pub struct EditorState {
    pub buffers: HashMap<BufferId, BufferState>,
    pub windows: HashMap<WindowId, WindowState>,
    pub mode: ModeState,
    pub parser: KeyParser,
    pub registers: RegisterFile,
    pub size: Size,
    pub active_window: Option<WindowId>,
    pub message: Option<String>,
    pub should_quit: bool,
    pub search_pattern: Option<String>,
    pub search_forward: bool,
    pub marks: HashMap<char, (BufferId, Position)>,
    pub last_find_char: Option<(char, FindCharKind)>,
    pub last_change: Option<kjxlkj_core_types::Intent>,
    pub macro_recording: Option<(char, Vec<kjxlkj_core_types::Intent>)>,
    pub macros: HashMap<char, Vec<kjxlkj_core_types::Intent>>,
    pub last_macro: Option<char>,
    pub jump_list: Vec<(BufferId, Position)>,
    pub jump_list_idx: usize,
    pub change_list: Vec<(BufferId, Position)>,
    pub change_list_idx: usize,
    pub cmdline: CommandLine,
    pub options: EditorOptions,
    pub mappings: mappings::MappingTable,
    pub autocmds: autocommands::AutoCmdTable,
    pub last_visual: Option<(Position, Position, Mode)>,
    pub syntax_enabled: bool,
    pub highlight_overrides: HashMap<kjxlkj_core_types::HighlightGroup, kjxlkj_core_types::Style>,
    pub alternate_file: Option<BufferId>,
    pub recent_files: session::RecentFiles,
    pub autosave: session::AutoSaveConfig,
    pub quickfix: quickfix::QuickfixList,
    pub loclist: quickfix::QuickfixList,
    next_buffer_id: u64,
    next_window_id: u64,
}

/// Command-line editing state.
pub struct CommandLine {
    pub text: String,
    pub cursor: usize,
    pub prefix: char,
    pub history: Vec<String>,
    pub history_idx: Option<usize>,
    pub saved_text: Option<String>,
}

/// Editor configuration options.
pub struct EditorOptions {
    pub number: bool, pub relative_number: bool, pub wrap: bool,
    pub ignorecase: bool, pub smartcase: bool,
    pub hlsearch: bool, pub incsearch: bool,
    pub expandtab: bool, pub tabstop: usize, pub shiftwidth: usize,
    pub scrolloff: usize, pub sidescrolloff: usize,
    pub autoindent: bool, pub smartindent: bool, pub autopairs: bool,
    pub list: bool, pub cursorline: bool, pub cursorcolumn: bool,
    pub showmode: bool, pub showcmd: bool, pub hidden: bool,
}

impl Default for CommandLine {
    fn default() -> Self {
        Self { text: String::new(), cursor: 0, prefix: ':', history: Vec::new(), history_idx: None, saved_text: None }
    }
}

impl Default for EditorOptions {
    fn default() -> Self {
        Self {
            number: false, relative_number: false, wrap: true, ignorecase: false,
            smartcase: true, hlsearch: true, incsearch: true, expandtab: true,
            tabstop: 4, shiftwidth: 4, scrolloff: 3, sidescrolloff: 0,
            autoindent: true, smartindent: false, autopairs: false,
            list: false, cursorline: false, cursorcolumn: false,
            showmode: true, showcmd: true, hidden: false,
        }
    }
}

impl EditorState {
    pub fn new(size: Size) -> Self {
        Self {
            buffers: HashMap::new(), windows: HashMap::new(),
            mode: ModeState::new(), parser: KeyParser::new(),
            registers: RegisterFile::new(), size,
            active_window: None, message: None, should_quit: false,
            search_pattern: None, search_forward: true,
            marks: HashMap::new(), last_find_char: None, last_change: None,
            macro_recording: None, macros: HashMap::new(), last_macro: None,
            jump_list: Vec::new(), jump_list_idx: 0,
            change_list: Vec::new(), change_list_idx: 0,
            cmdline: CommandLine::default(), options: EditorOptions::default(),
            mappings: mappings::MappingTable::new(), autocmds: autocommands::AutoCmdTable::new(),
            last_visual: None,
            syntax_enabled: true,
            highlight_overrides: HashMap::new(),
            alternate_file: None,
            recent_files: session::RecentFiles::new(100),
            autosave: session::AutoSaveConfig::default(),
            quickfix: quickfix::QuickfixList::new(),
            loclist: quickfix::QuickfixList::new(),
            next_buffer_id: 1, next_window_id: 1,
        }
    }

    pub fn create_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::new(id));
        id
    }

    pub fn create_buffer_from_text(&mut self, text: &str) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::from_text(id, text));
        id
    }

    pub fn create_window(&mut self, buffer_id: BufferId) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        let mut win = WindowState::new(id, buffer_id);
        win.scrolloff = self.options.scrolloff;
        win.sidescrolloff = self.options.sidescrolloff;
        win.wrap = self.options.wrap;
        win.width = self.size.width as usize;
        win.height = self.size.height.saturating_sub(2) as usize;
        self.windows.insert(id, win);
        if self.active_window.is_none() { self.active_window = Some(id); }
        id
    }

    pub fn current_mode(&self) -> Mode { self.mode.current() }

    pub fn cursor(&self) -> Position {
        self.active_window.and_then(|wid| self.windows.get(&wid))
            .map(|w| Position::new(w.cursor_line, w.cursor_col)).unwrap_or_default()
    }

    pub fn active_buffer(&self) -> Option<&BufferState> {
        let win = self.windows.get(&self.active_window?)?;
        self.buffers.get(&win.buffer_id)
    }

    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let bid = self.windows.get(&self.active_window?)?.buffer_id;
        self.buffers.get_mut(&bid)
    }

    pub fn active_window_state(&self) -> Option<&WindowState> {
        self.active_window.and_then(|id| self.windows.get(&id))
    }

    pub fn active_window_mut(&mut self) -> Option<&mut WindowState> {
        self.windows.get_mut(&self.active_window?)
    }

    pub fn has_unsaved_changes(&self) -> bool { self.buffers.values().any(|b| b.modified) }

    pub fn visual_range(&self) -> Option<Range> {
        let win = self.windows.get(&self.active_window?)?;
        let anchor = win.visual_anchor?;
        let cursor = Position::new(win.cursor_line, win.cursor_col);
        Some(if anchor <= cursor { Range::new(anchor, cursor) } else { Range::new(cursor, anchor) })
    }
}
