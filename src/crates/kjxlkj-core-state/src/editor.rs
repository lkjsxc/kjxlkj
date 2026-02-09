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
        }
    }

    /// Open file content into a buffer and display it.
    pub fn open_file(&mut self, path: &str, content: &str) {
        let buf_id = self.buffers.open(content, std::path::PathBuf::from(path));
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }
}
