//! Editor state: aggregates buffers, windows, mode,
//! and snapshot production.

use std::collections::HashMap;

use kjxlkj_core_edit::{CursorPosition, RegisterFile};
use kjxlkj_core_mode::{
    CommandModeState, InsertModeState,
    NormalModeState, VisualModeState,
};
use kjxlkj_core_types::{
    Action, BufferId, ForceMotionType, Key, Mode, Operator,
    TabId, TextObjectScope, WindowId,
};

use crate::after_dir::AfterDirConfig;
use crate::audio::AudioConfig;
use crate::auto_session::{AutoSessionConfig, InitFileState};
use crate::autocmd::AutoCmdRegistry;
use crate::buffer_options::{ArgList, BufferGroupRegistry};
use crate::completion::CompletionPopup;
use crate::dap::DapState;
use crate::digraphs::DigraphTable;
use crate::editor_tabs::TabPage;
use crate::flash_jump::FlashState;
use crate::floating::FloatRegistry;
use crate::folds_advanced::FoldState;
use crate::git_features::GitState;
use crate::ime::ImeState;
use crate::keybinding_dsl::{
    CommandPalette, LeaderConfig, WhichKeyState,
};
use crate::live_grep::LiveGrepState;
use crate::lsp_features::LspState;
use crate::mappings::MappingRegistry;
use crate::mouse::MouseConfig;
use crate::multicursor::MultiCursorState;
use crate::notifications::NotificationManager;
use crate::persistence::PersistenceConfig;
use crate::popup::PopupMenu;
use crate::regex_engine::RegexConfig;
use crate::remote::RemoteState;
use crate::search::SearchState;
use crate::session_features::{
    ExCommandBatch, ExpressionEval, MacroPersistence,
    RegisterPersistence,
};
use crate::snippets::{SnippetRegistry, SnippetState};
use crate::spell::SpellChecker;
use crate::statusline_dsl::StatuslineConfig;
use crate::tags::TagStack;
use crate::theming::ThemeRegistry;
use crate::tmux::TmuxState;
use crate::treesitter_objects::TsTextObjects;
use crate::unicode_input::UnicodeInputState;
use crate::user_commands::UserCommandRegistry;
use crate::user_functions::UserFunctionRegistry;
use crate::view_management::ViewRegistry;
use crate::wm_integration::WmState;
use crate::{BufferState, WindowState};

/// A stored mark: buffer id + cursor position.
#[derive(Debug, Clone, Copy)]
pub struct MarkEntry {
    pub buffer: BufferId,
    pub cursor: CursorPosition,
}

/// A quickfix list entry.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    /// File path.
    pub file: String,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub col: usize,
    /// Error type (E, W, I).
    pub kind: char,
    /// Error text.
    pub text: String,
}

/// Top-level editor state.
pub struct EditorState {
    /// All open buffers.
    pub buffers: HashMap<BufferId, BufferState>,
    /// All open windows.
    pub windows: HashMap<WindowId, WindowState>,
    /// Currently focused window.
    pub focused_window: WindowId,
    /// Current editing mode.
    pub mode: Mode,
    /// Normal mode state.
    pub normal_state: NormalModeState,
    /// Insert mode state.
    pub insert_state: InsertModeState,
    /// Visual mode state.
    pub visual_state: Option<VisualModeState>,
    /// Command mode state.
    pub command_state: Option<CommandModeState>,
    /// Snapshot sequence counter.
    pub sequence: u64,
    /// Terminal dimensions.
    pub terminal_size: (u16, u16),
    /// Next buffer ID.
    next_buffer_id: u64,
    /// Next window ID.
    next_window_id: u64,
    /// Should quit flag.
    pub should_quit: bool,
    /// Search state.
    pub search_state: SearchState,
    /// Marks storage: char → (buffer, pos).
    pub marks: HashMap<char, MarkEntry>,
    /// Alternate buffer for `Ctrl-^`.
    pub alternate_buffer: Option<BufferId>,
    /// Last repeatable action for dot-repeat.
    pub last_repeatable: Option<Action>,
    /// Register file for yank/delete/macro storage.
    pub register_file: RegisterFile,
    /// Macro recording: register being recorded into.
    pub macro_recording: Option<char>,
    /// Macro recording: key buffer.
    pub macro_keys: Vec<Key>,
    /// Jump list: (buffer, cursor) entries.
    pub jump_list: Vec<(BufferId, CursorPosition)>,
    /// Jump list cursor (for Ctrl-O / Ctrl-I).
    pub jump_list_pos: usize,
    /// Change list: (buffer, cursor) entries.
    pub change_list: Vec<(BufferId, CursorPosition)>,
    /// Change list cursor (for g; / g,).
    pub change_list_pos: usize,
    /// Pending text object scope in operator-pending mode.
    pub op_text_obj_pending: Option<(Operator, TextObjectScope)>,
    /// Forced motion type in operator-pending mode.
    pub op_force_motion: Option<ForceMotionType>,
    /// Terminal escape pending: after Ctrl-\ waiting for Ctrl-n.
    pub terminal_escape_pending: bool,
    /// Macro playback depth for recursion limit.
    pub macro_depth: u32,
    /// Autocommand registry.
    pub autocmds: AutoCmdRegistry,
    /// Quickfix list entries.
    pub quickfix: Vec<QuickfixEntry>,
    /// Quickfix list cursor position.
    pub quickfix_pos: usize,
    /// Previous window for `Ctrl-w p`.
    pub prev_window: Option<WindowId>,
    /// Key mapping registry.
    pub mappings: MappingRegistry,
    /// Tab pages.
    pub tabs: Vec<TabPage>,
    /// Active tab index.
    pub active_tab: usize,
    /// Next tab ID counter.
    pub next_tab_id: u64,
    /// Floating window registry.
    pub float_registry: FloatRegistry,
    /// User-defined commands.
    pub user_commands: UserCommandRegistry,
    /// Tag stack for navigation.
    pub tag_stack: TagStack,
    /// Completion popup state.
    pub completion: CompletionPopup,
    /// LSP feature state.
    pub lsp_state: LspState,
    /// Git feature state.
    pub git_state: GitState,
    /// Flash/EasyMotion jump state.
    pub flash_state: FlashState,
    /// Multi-cursor state.
    pub multi_cursor: MultiCursorState,
    /// Snippet engine state.
    pub snippet_state: SnippetState,
    /// Snippet registry.
    pub snippet_registry: SnippetRegistry,
    /// Spell checker.
    pub spell_checker: SpellChecker,
    /// Notification manager.
    pub notifications: NotificationManager,
    /// Popup menu state.
    pub popup_menu: PopupMenu,
    /// Persistence configuration.
    pub persistence: PersistenceConfig,
    /// Theme registry.
    pub theme_registry: ThemeRegistry,
    /// Buffer group registry.
    pub buffer_groups: BufferGroupRegistry,
    /// Argument list.
    pub arglist: ArgList,
    /// Leader key configuration.
    pub leader_config: LeaderConfig,
    /// Which-key state.
    pub which_key: WhichKeyState,
    /// Command palette state.
    pub command_palette: CommandPalette,
    /// Digraph table.
    pub digraph_table: DigraphTable,
    /// Mouse configuration.
    pub mouse_config: MouseConfig,
    /// Statusline configuration.
    pub statusline_config: StatuslineConfig,
    /// User function registry.
    pub user_functions: UserFunctionRegistry,
    /// Tmux integration state.
    pub tmux_state: TmuxState,
    /// DAP debugging state.
    pub dap_state: DapState,
    /// Remote editing state.
    pub remote_state: RemoteState,
    /// Window manager integration.
    pub wm_state: WmState,
    /// View management registry.
    pub view_registry: ViewRegistry,
    /// Macro persistence.
    pub macro_persistence: MacroPersistence,
    /// Register persistence.
    pub register_persistence: RegisterPersistence,
    /// Expression evaluator.
    pub expr_eval: ExpressionEval,
    /// Ex command batch processor.
    pub ex_batch: ExCommandBatch,
    /// IME composition state.
    pub ime_state: ImeState,
    /// Unicode input state.
    pub unicode_input: UnicodeInputState,
    /// Live grep state.
    pub live_grep: LiveGrepState,
    /// Regex configuration.
    pub regex_config: RegexConfig,
    /// After-directory config.
    pub after_dir: AfterDirConfig,
    /// Audio/bell config.
    pub audio_config: AudioConfig,
    /// Advanced fold state.
    pub fold_state: FoldState,
    /// Auto-session configuration.
    pub auto_session: AutoSessionConfig,
    /// Init file sourcing state.
    pub init_files: InitFileState,
    /// Treesitter text objects.
    pub ts_text_objects: TsTextObjects,
}

impl EditorState {
    /// Create editor state with a single empty buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(1);
        let win_id = WindowId(1);

        let buf = BufferState::new(buf_id);
        let mut win =
            WindowState::new_buffer(win_id, buf_id);
        win.viewport
            .set_size(cols, rows.saturating_sub(2));

        let mut buffers = HashMap::new();
        buffers.insert(buf_id, buf);

        let mut windows = HashMap::new();
        windows.insert(win_id, win);

        Self {
            buffers,
            windows,
            focused_window: win_id,
            mode: Mode::Normal,
            normal_state: NormalModeState::new(),
            insert_state: InsertModeState::new(),
            visual_state: None,
            command_state: None,
            sequence: 0,
            terminal_size: (cols, rows),
            next_buffer_id: 2,
            next_window_id: 2,
            should_quit: false,
            search_state: SearchState::new(),
            marks: HashMap::new(),
            alternate_buffer: None,
            last_repeatable: None,
            register_file: RegisterFile::new(),
            macro_recording: None,
            macro_keys: Vec::new(),
            jump_list: Vec::new(),
            jump_list_pos: 0,
            change_list: Vec::new(),
            change_list_pos: 0,
            op_text_obj_pending: None,
            op_force_motion: None,
            terminal_escape_pending: false,
            macro_depth: 0,
            autocmds: AutoCmdRegistry::new(),
            quickfix: Vec::new(),
            quickfix_pos: 0,
            prev_window: None,
            mappings: MappingRegistry::new(),
            tabs: vec![TabPage::new(TabId(1), win_id)],
            active_tab: 0,
            next_tab_id: 2,
            float_registry: FloatRegistry::new(),
            user_commands: UserCommandRegistry::new(),
            tag_stack: TagStack::new(),
            completion: CompletionPopup::new(),
            lsp_state: LspState::default(),
            git_state: GitState::default(),
            flash_state: FlashState::new(),
            multi_cursor: MultiCursorState::new(),
            snippet_state: SnippetState::new(),
            snippet_registry: SnippetRegistry::new(),
            spell_checker: SpellChecker::new(),
            notifications: NotificationManager::new(),
            popup_menu: PopupMenu::new(),
            persistence: PersistenceConfig::new(),
            theme_registry: ThemeRegistry::new(),
            buffer_groups: BufferGroupRegistry::new(),
            arglist: ArgList::new(),
            leader_config: LeaderConfig::default(),
            which_key: WhichKeyState::new(),
            command_palette: CommandPalette::new(),
            digraph_table: DigraphTable::new(),
            mouse_config: MouseConfig::new(),
            statusline_config: StatuslineConfig::new(),
            user_functions: UserFunctionRegistry::new(),
            tmux_state: TmuxState::detect(),
            dap_state: DapState::new(),
            remote_state: RemoteState::new(),
            wm_state: WmState::detect(),
            view_registry: ViewRegistry::new(),
            macro_persistence: MacroPersistence::new(),
            register_persistence: RegisterPersistence::new(),
            expr_eval: ExpressionEval::new(),
            ex_batch: ExCommandBatch::new(),
            ime_state: ImeState::new(),
            unicode_input: UnicodeInputState::new(),
            live_grep: LiveGrepState::new(),
            regex_config: RegexConfig::default(),
            after_dir: AfterDirConfig::new(),
            audio_config: AudioConfig::new(),
            fold_state: FoldState::new(),
            auto_session: AutoSessionConfig::new(),
            init_files: InitFileState::new(),
            ts_text_objects: TsTextObjects::new(),
        }
    }

    /// Allocate a new buffer ID.
    pub fn alloc_buffer_id(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        id
    }

    /// Allocate a new window ID.
    pub fn alloc_window_id(&mut self) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Get the focused window.
    pub fn focused_window(&self) -> Option<&WindowState> {
        self.windows.get(&self.focused_window)
    }

    /// Get the focused window mutably.
    pub fn focused_window_mut(&mut self) -> Option<&mut WindowState> {
        self.windows.get_mut(&self.focused_window)
    }

    /// Get the active buffer ID.
    pub fn active_buffer_id(&self) -> Option<BufferId> {
        self.focused_window().and_then(|w| w.buffer_id())
    }

    /// Get the active buffer.
    pub fn active_buffer(&self) -> Option<&BufferState> {
        self.active_buffer_id().and_then(|id| self.buffers.get(&id))
    }

    /// Get the active buffer mutably.
    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let id = self.active_buffer_id()?;
        self.buffers.get_mut(&id)
    }

    /// Handle a resize event.
    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.terminal_size = (cols, rows);
        for win in self.windows.values_mut() {
            win.viewport.set_size(cols, rows.saturating_sub(2));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_editor() {
        let state = EditorState::new(80, 24);
        assert_eq!(state.mode, Mode::Normal);
        assert_eq!(state.buffers.len(), 1);
        assert_eq!(state.windows.len(), 1);
        assert!(!state.should_quit);
    }

    #[test]
    fn snapshot_production() {
        let mut state = EditorState::new(80, 24);
        let snap = state.snapshot();
        assert_eq!(snap.sequence, 1);
        assert_eq!(snap.mode, Mode::Normal);
        assert_eq!(snap.buffers.len(), 1);
    }
}
