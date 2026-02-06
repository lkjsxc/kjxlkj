//! Editor state aggregation — ties together all core sub-crates.

mod autocommands;
mod buffer_state;
mod commands;
mod commands_config;
mod commands_config_map;
mod commands_display;
mod commands_file;
mod commands_nav;
mod commands_range;
mod commands_range_ops;
mod commands_line_ops;
mod commands_substitute;
mod config;
mod dispatch;
mod dispatch_cmdline;
mod dispatch_editing;
mod dispatch_editing_extra;
mod dispatch_jumps;
mod dispatch_macros;
mod dispatch_misc;
mod dispatch_navigation;
mod dispatch_operators;
mod dispatch_search;
mod dispatch_windows;
mod mappings;
mod registers;
mod window_state;

pub use autocommands::{AutoCmdTable, AutoEvent};
pub use buffer_state::BufferState;
pub use config::{execute_script, load_default_config};
pub use dispatch::dispatch_intent;
pub use dispatch_cmdline::handle_cmdline_key;
pub use mappings::{MappingMode, MappingTable};
pub use registers::RegisterFile;
pub use window_state::WindowState;

use std::collections::HashMap;

use kjxlkj_core_mode::{KeyParser, ModeState};
use kjxlkj_core_types::{
    BufferId, FindCharKind, Mode, Position, Range, Size, WindowId,
};

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
    /// Search state.
    pub search_pattern: Option<String>,
    pub search_forward: bool,
    /// Marks: char -> (buffer_id, position).
    pub marks: HashMap<char, (BufferId, Position)>,
    /// Last find-char for ;/, repeat.
    pub last_find_char: Option<(char, FindCharKind)>,
    /// Last repeatable intent for dot repeat.
    pub last_change: Option<kjxlkj_core_types::Intent>,
    /// Macro recording: which register and accumulated intents.
    pub macro_recording: Option<(char, Vec<kjxlkj_core_types::Intent>)>,
    /// Macro storage: register char -> list of intents.
    pub macros: HashMap<char, Vec<kjxlkj_core_types::Intent>>,
    /// Last played macro register for @@ repeat.
    pub last_macro: Option<char>,
    /// Jump list: stack of (buffer_id, position).
    pub jump_list: Vec<(BufferId, Position)>,
    pub jump_list_idx: usize,
    /// Change list: positions where changes occurred.
    pub change_list: Vec<(BufferId, Position)>,
    pub change_list_idx: usize,
    /// Command line state.
    pub cmdline: CommandLine,
    /// Editor options/configuration.
    pub options: EditorOptions,
    /// User-defined key mappings.
    pub mappings: mappings::MappingTable,
    /// Autocommand table.
    pub autocmds: autocommands::AutoCmdTable,
    next_buffer_id: u64,
    next_window_id: u64,
}

/// Command-line editing state.
pub struct CommandLine {
    /// Current command text being typed.
    pub text: String,
    /// Cursor position within the command text.
    pub cursor: usize,
    /// Command prefix (: / ?).
    pub prefix: char,
    /// History of previously executed commands.
    pub history: Vec<String>,
    /// Current position in history navigation.
    pub history_idx: Option<usize>,
    /// Saved input text when navigating history.
    pub saved_text: Option<String>,
}

/// Editor configuration options.
pub struct EditorOptions {
    pub number: bool,
    pub relative_number: bool,
    pub wrap: bool,
    pub ignorecase: bool,
    pub smartcase: bool,
    pub hlsearch: bool,
    pub incsearch: bool,
    pub expandtab: bool,
    pub tabstop: usize,
    pub shiftwidth: usize,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
    pub autoindent: bool,
    pub smartindent: bool,
}

impl Default for CommandLine {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            prefix: ':',
            history: Vec::new(),
            history_idx: None,
            saved_text: None,
        }
    }
}

impl Default for EditorOptions {
    fn default() -> Self {
        Self {
            number: false,
            relative_number: false,
            wrap: true,
            ignorecase: false,
            smartcase: true,
            hlsearch: true,
            incsearch: true,
            expandtab: true,
            tabstop: 4,
            shiftwidth: 4,
            scrolloff: 3,
            sidescrolloff: 0,
            autoindent: true,
            smartindent: false,
        }
    }
}

impl EditorState {
    pub fn new(size: Size) -> Self {
        Self {
            buffers: HashMap::new(),
            windows: HashMap::new(),
            mode: ModeState::new(),
            parser: KeyParser::new(),
            registers: RegisterFile::new(),
            size,
            active_window: None,
            message: None,
            should_quit: false,
            search_pattern: None,
            search_forward: true,
            marks: HashMap::new(),
            last_find_char: None,
            last_change: None,
            macro_recording: None,
            macros: HashMap::new(),
            last_macro: None,
            jump_list: Vec::new(),
            jump_list_idx: 0,
            change_list: Vec::new(),
            change_list_idx: 0,
            cmdline: CommandLine::default(),
            options: EditorOptions::default(),
            mappings: mappings::MappingTable::new(),
            autocmds: autocommands::AutoCmdTable::new(),
            next_buffer_id: 1,
            next_window_id: 1,
        }
    }

    /// Create a new empty buffer, returning its ID.
    pub fn create_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::new(id));
        tracing::debug!(?id, "created buffer");
        id
    }

    /// Create a buffer from text content.
    pub fn create_buffer_from_text(&mut self, text: &str) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        self.buffers.insert(id, BufferState::from_text(id, text));
        tracing::debug!(?id, "created buffer from text");
        id
    }

    /// Create a new window for a buffer, returning the window ID.
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
        if self.active_window.is_none() {
            self.active_window = Some(id);
        }
        tracing::debug!(?id, ?buffer_id, "created window");
        id
    }

    /// Current mode.
    pub fn current_mode(&self) -> Mode {
        self.mode.current()
    }

    /// Get cursor position in the active window.
    pub fn cursor(&self) -> Position {
        self.active_window
            .and_then(|wid| self.windows.get(&wid))
            .map(|w| Position::new(w.cursor_line, w.cursor_col))
            .unwrap_or_default()
    }

    /// Get the active buffer, if any.
    pub fn active_buffer(&self) -> Option<&BufferState> {
        let wid = self.active_window?;
        let win = self.windows.get(&wid)?;
        self.buffers.get(&win.buffer_id)
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let wid = self.active_window?;
        let win = self.windows.get(&wid)?;
        let bid = win.buffer_id;
        self.buffers.get_mut(&bid)
    }

    /// Get the active window, if any.
    pub fn active_window_state(&self) -> Option<&WindowState> {
        self.active_window.and_then(|id| self.windows.get(&id))
    }

    /// Get the active window mutably.
    pub fn active_window_mut(&mut self) -> Option<&mut WindowState> {
        let wid = self.active_window?;
        self.windows.get_mut(&wid)
    }

    /// Check if any buffer has unsaved changes.
    pub fn has_unsaved_changes(&self) -> bool {
        self.buffers.values().any(|b| b.modified)
    }

    /// Get the visual selection range, if in visual mode.
    pub fn visual_range(&self) -> Option<Range> {
        let wid = self.active_window?;
        let win = self.windows.get(&wid)?;
        let anchor = win.visual_anchor?;
        let cursor = Position::new(win.cursor_line, win.cursor_col);
        if anchor <= cursor {
            Some(Range::new(anchor, cursor))
        } else {
            Some(Range::new(cursor, anchor))
        }
    }
}
