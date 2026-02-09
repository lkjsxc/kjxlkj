//! EditorState constructor.

use std::collections::HashMap;

use kjxlkj_core_edit::RegisterFile;
use kjxlkj_core_mode::{InsertModeState, NormalModeState};
use kjxlkj_core_types::{BufferId, Mode, TabId, WindowId};

use crate::after_dir::AfterDirConfig;
use crate::audio::AudioConfig;
use crate::auto_session::{AutoSessionConfig, InitFileState};
use crate::autocmd::AutoCmdRegistry;
use crate::buffer_options::{ArgList, BufferGroupRegistry};
use crate::completion::CompletionPopup;
use crate::dap::DapState;
use crate::digraphs::DigraphTable;
use crate::editor::EditorState;
use crate::editor_tabs::TabPage;
use crate::flash_jump::FlashState;
use crate::floating::FloatRegistry;
use crate::folds_advanced::FoldState;
use crate::git_features::GitState;
use crate::ime::ImeState;
use crate::keybinding_dsl::{CommandPalette, LeaderConfig, WhichKeyState};
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
    ExCommandBatch, ExpressionEval, MacroPersistence, RegisterPersistence,
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

impl EditorState {
    /// Create editor state with a single empty buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let buf_id = BufferId(1);
        let win_id = WindowId(1);

        let buf = BufferState::new(buf_id);
        let mut win = WindowState::new_buffer(win_id, buf_id);
        win.viewport.set_size(cols, rows.saturating_sub(2));

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
}
