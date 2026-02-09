/// Central editor state: single mutable owner in core task.
use kjxlkj_core_edit::RegisterFile;
use kjxlkj_core_mode::NormalDispatch;
use kjxlkj_core_types::{ContentSource, Mode};
use kjxlkj_core_ui::{Notification, SearchState, Theme};

use crate::buffer_list::BufferList;
use crate::cmdline::CmdlineHandler;
use crate::events::EventRegistry;
use crate::mappings::MappingTable;
use crate::marks::MarkFile;
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
        }
    }

    /// Open file content into a buffer and display it.
    pub fn open_file(&mut self, path: &str, content: &str) {
        let buf_id =
            self.buffers
                .open(content, std::path::PathBuf::from(path));
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }
}
