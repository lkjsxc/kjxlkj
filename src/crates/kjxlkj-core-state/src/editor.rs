//! Editor state.

use std::collections::HashMap;

use kjxlkj_core_edit::{CursorPosition, RegisterFile};
use kjxlkj_core_mode::{CommandModeState, InsertModeState, NormalModeState, VisualModeState};
use kjxlkj_core_types::{
    Action, BufferId, ForceMotionType, Key, Mode, Operator, TextObjectScope, WindowId,
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
use crate::editor_types::{MarkEntry, QuickfixEntry};
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
#[rustfmt::skip]
pub struct EditorState {
    pub buffers: HashMap<BufferId, BufferState>, pub windows: HashMap<WindowId, WindowState>, pub focused_window: WindowId,
    pub mode: Mode, pub normal_state: NormalModeState, pub insert_state: InsertModeState,
    pub visual_state: Option<VisualModeState>, pub command_state: Option<CommandModeState>,
    pub sequence: u64, pub terminal_size: (u16, u16), pub(crate) next_buffer_id: u64, pub(crate) next_window_id: u64,
    pub should_quit: bool, pub search_state: SearchState, pub marks: HashMap<char, MarkEntry>,
    pub alternate_buffer: Option<BufferId>, pub last_repeatable: Option<Action>, pub register_file: RegisterFile,
    pub macro_recording: Option<char>, pub macro_keys: Vec<Key>, pub jump_list: Vec<(BufferId, CursorPosition)>,
    pub jump_list_pos: usize, pub change_list: Vec<(BufferId, CursorPosition)>, pub change_list_pos: usize,
    pub op_text_obj_pending: Option<(Operator, TextObjectScope)>, pub op_force_motion: Option<ForceMotionType>,
    pub terminal_escape_pending: bool, pub macro_depth: u32,
    pub autocmds: AutoCmdRegistry, pub quickfix: Vec<QuickfixEntry>, pub quickfix_pos: usize,
    pub prev_window: Option<WindowId>, pub mappings: MappingRegistry,
    pub tabs: Vec<TabPage>, pub active_tab: usize, pub next_tab_id: u64,
    pub float_registry: FloatRegistry, pub user_commands: UserCommandRegistry, pub tag_stack: TagStack,
    pub completion: CompletionPopup, pub lsp_state: LspState, pub git_state: GitState, pub flash_state: FlashState,
    pub multi_cursor: MultiCursorState, pub snippet_state: SnippetState, pub snippet_registry: SnippetRegistry,
    pub spell_checker: SpellChecker, pub notifications: NotificationManager, pub popup_menu: PopupMenu,
    pub persistence: PersistenceConfig, pub theme_registry: ThemeRegistry,
    pub buffer_groups: BufferGroupRegistry, pub arglist: ArgList,
    pub leader_config: LeaderConfig, pub which_key: WhichKeyState, pub command_palette: CommandPalette,
    pub digraph_table: DigraphTable, pub mouse_config: MouseConfig, pub statusline_config: StatuslineConfig,
    pub user_functions: UserFunctionRegistry, pub tmux_state: TmuxState, pub dap_state: DapState, pub remote_state: RemoteState,
    pub wm_state: WmState, pub view_registry: ViewRegistry,
    pub macro_persistence: MacroPersistence, pub register_persistence: RegisterPersistence,
    pub expr_eval: ExpressionEval, pub ex_batch: ExCommandBatch,
    pub ime_state: ImeState, pub unicode_input: UnicodeInputState,
    pub live_grep: LiveGrepState, pub regex_config: RegexConfig,
    pub after_dir: AfterDirConfig, pub audio_config: AudioConfig, pub fold_state: FoldState,
    pub auto_session: AutoSessionConfig, pub init_files: InitFileState, pub ts_text_objects: TsTextObjects,
}

impl EditorState {
    pub fn alloc_buffer_id(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer_id);
        self.next_buffer_id += 1;
        id
    }

    pub fn alloc_window_id(&mut self) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    pub fn focused_window(&self) -> Option<&WindowState> {
        self.windows.get(&self.focused_window)
    }

    pub fn focused_window_mut(&mut self) -> Option<&mut WindowState> {
        self.windows.get_mut(&self.focused_window)
    }

    pub fn active_buffer_id(&self) -> Option<BufferId> {
        self.focused_window().and_then(|w| w.buffer_id())
    }

    pub fn active_buffer(&self) -> Option<&BufferState> {
        self.active_buffer_id().and_then(|id| self.buffers.get(&id))
    }

    pub fn active_buffer_mut(&mut self) -> Option<&mut BufferState> {
        let id = self.active_buffer_id()?;
        self.buffers.get_mut(&id)
    }

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
