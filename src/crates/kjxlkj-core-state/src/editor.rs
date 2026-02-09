/// Central editor state: single mutable owner in core task.
use kjxlkj_core_edit::RegisterFile;
use kjxlkj_core_mode::NormalDispatch;
use kjxlkj_core_types::{ContentSource, CursorPosition, Key, Mode};
use kjxlkj_core_ui::{Notification, SearchState, Theme};

use crate::buffer_list::BufferList;
use crate::cmdline::CmdlineHandler;
use crate::events::EventRegistry;
use crate::mappings::MappingTable;
use crate::marks::MarkFile;
use crate::options::OptionStore;
use crate::user_commands::UserCommandRegistry;
use crate::window_tree::WindowTree;

/// Central editor state: single mutable owner in core task.
#[derive(Debug)]
pub struct EditorState {
    pub buffers: BufferList,
    pub windows: WindowTree,
    pub mode: Mode,
    pub cmdline: CmdlineHandler,
    pub registers: RegisterFile,
    pub search: SearchState,
    pub notifications: Vec<Notification>,
    pub terminal_size: (u16, u16),
    pub theme: Theme,
    pub quit_requested: bool,
    pub sequence: u64,
    pub mappings: MappingTable,
    pub marks: MarkFile,
    pub events: EventRegistry,
    pub user_commands: UserCommandRegistry,
    pub(crate) dispatch: NormalDispatch,
    /// Anchor position for visual mode selection.
    pub visual_anchor: Option<CursorPosition>,
    /// Count saved from before operator key.
    pub(crate) op_count: usize,
    /// Motion count accumulating in operator-pending mode.
    pub(crate) motion_count: Option<usize>,
    /// g-prefix flag for operator-pending mode.
    pub(crate) g_prefix: bool,
    /// Pending register for next yank/delete/put.
    pub(crate) pending_register: Option<char>,
    /// Editor options (:set).
    pub options: OptionStore,
    /// Macro recording target register (None = not recording).
    pub(crate) recording_macro: Option<char>,
    /// Keys recorded during current macro recording.
    pub(crate) macro_buffer: Vec<Key>,
    /// Per-register macro key storage.
    pub(crate) macro_store: std::collections::HashMap<char, Vec<Key>>,
    /// Last executed macro register for @@ replay.
    pub(crate) last_macro: Option<char>,
    /// Last f/t/F/T motion for ; and , repeat: (kind, char).
    /// kind: 'f'=forward, 'F'=backward, 't'=till fwd, 'T'=till bck.
    pub(crate) last_ft: Option<(char, char)>,
    /// Text-object prefix: 'i' (inner) or 'a' (around) in op-pending.
    pub(crate) text_obj_prefix: Option<char>,
    /// Macro recursion depth counter.
    pub(crate) macro_depth: usize,
    /// Last inserted text for . register.
    pub(crate) last_inserted_text: String,
    /// Last executed ex command for : register.
    pub(crate) last_ex_command: String,
    /// Changelist: positions of recent changes for g;/g, navigation.
    pub(crate) changelist: Vec<(usize, usize, usize)>,
    /// Current index in changelist (for g;/g,).
    pub(crate) changelist_idx: usize,
    /// Set during macro playback when an error occurs, to halt.
    pub(crate) macro_error: bool,
    /// Jump list for Ctrl-O / Ctrl-I navigation.
    pub(crate) jumplist: Vec<(usize, usize, usize)>,
    /// Current index in jump list.
    pub(crate) jumplist_idx: usize,
    /// Alternate (previous) buffer id for # register.
    pub(crate) alternate_buffer: Option<kjxlkj_core_types::BufferId>,
    /// Pending block insert info: (start_line, end_line, col, at_end).
    pub(crate) block_insert_pending: Option<(usize, usize, usize, bool)>,
    /// Ctrl-R pressed in insert mode: next char selects register.
    pub(crate) insert_register_pending: bool,
    /// `r` pressed in visual mode: next char replaces selection.
    pub(crate) visual_replace_pending: bool,
    /// Last visual selection for gv: (anchor, cursor, kind).
    pub(crate) last_visual: Option<(
        CursorPosition,
        CursorPosition,
        kjxlkj_core_types::VisualKind,
    )>,
    /// Pending substitute confirmation: (pattern, replacement, global, line_indices_remaining).
    pub(crate) sub_confirm: Option<SubConfirmState>,
}

/// State for :s///c confirmation dialog.
#[derive(Debug, Clone)]
pub struct SubConfirmState {
    pub pattern: String,
    pub replacement: String,
    pub global: bool,
    pub lines: Vec<usize>,
    pub current_line_idx: usize,
}

impl EditorState {
    /// Create a new editor with an initial scratch buffer.
    pub fn new(cols: u16, rows: u16) -> Self {
        let mut buffers = BufferList::new();
        let buf_id = buffers.create_scratch();
        let windows = WindowTree::new(buf_id);

        Self {
            buffers,
            windows,
            mode: Mode::Normal,
            cmdline: CmdlineHandler::new(),
            registers: RegisterFile::new(),
            search: SearchState::default(),
            notifications: Vec::new(),
            terminal_size: (cols, rows),
            theme: Theme::default(),
            quit_requested: false,
            sequence: 0,
            mappings: MappingTable::new(),
            marks: MarkFile::new(),
            events: EventRegistry::new(),
            user_commands: UserCommandRegistry::new(),
            dispatch: NormalDispatch::new(),
            visual_anchor: None,
            op_count: 1,
            motion_count: None,
            g_prefix: false,
            pending_register: None,
            options: OptionStore::new(),
            recording_macro: None,
            macro_buffer: Vec::new(),
            macro_store: std::collections::HashMap::new(),
            last_macro: None,
            last_ft: None,
            text_obj_prefix: None,
            macro_depth: 0,
            last_inserted_text: String::new(),
            last_ex_command: String::new(),
            changelist: Vec::new(),
            changelist_idx: 0,
            macro_error: false,
            jumplist: Vec::new(),
            jumplist_idx: 0,
            alternate_buffer: None,
            block_insert_pending: None,
            insert_register_pending: false,
            visual_replace_pending: false,
            last_visual: None,
            sub_confirm: None,
        }
    }

    /// Open file content into a buffer and display it.
    pub fn open_file(&mut self, path: &str, content: &str) {
        let prev = self.current_buffer_id();
        let buf_id = self.buffers.open(content, std::path::PathBuf::from(path));
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
        self.alternate_buffer = Some(prev);
        self.parse_modeline();
        // Detect filetype from extension.
        if let Some(ft) = crate::config_loader::detect_filetype(path) {
            self.options
                .set("filetype", crate::options::OptionValue::Str(ft.to_string()));
        }
    }
}
