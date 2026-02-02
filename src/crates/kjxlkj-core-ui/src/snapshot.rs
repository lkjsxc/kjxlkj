//! Editor snapshot for rendering.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

use crate::{BufferView, Dimensions, Layout, StatusLine};

/// File explorer entry for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerEntry {
    /// Display name.
    pub name: String,
    /// Is directory.
    pub is_dir: bool,
    /// Nesting depth.
    pub depth: usize,
    /// Is expanded.
    pub expanded: bool,
}

/// File explorer snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExplorerSnapshot {
    /// Is explorer open.
    pub open: bool,
    /// Width.
    pub width: u16,
    /// Visible entries.
    pub entries: Vec<ExplorerEntry>,
    /// Selected index.
    pub selected: usize,
}

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Terminal dimensions.
    pub dimensions: Dimensions,
    /// Current mode.
    pub mode: Mode,
    /// Window layout.
    pub layout: Layout,
    /// Buffer views.
    pub views: Vec<BufferView>,
    /// Status line.
    pub status: StatusLine,
    /// Command line content.
    pub command_line: Option<String>,
    /// Message to display.
    pub message: Option<String>,
    /// File explorer.
    pub explorer: ExplorerSnapshot,
}

impl EditorSnapshot {
    /// Creates a new snapshot.
    pub fn new(dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            mode: Mode::Normal,
            layout: Layout::default(),
            views: Vec::new(),
            status: StatusLine::new(),
            command_line: None,
            message: None,
            explorer: ExplorerSnapshot::default(),
        }
    }

    /// Returns the active buffer view.
    pub fn active_view(&self) -> Option<&BufferView> {
        self.views
            .iter()
            .find(|v| v.window_id == self.layout.active)
    }
}

impl Default for EditorSnapshot {
    fn default() -> Self {
        Self::new(Dimensions::new(80, 24))
    }
}
