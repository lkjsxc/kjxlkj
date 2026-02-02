//! Editor snapshot for rendering.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

use crate::{BufferView, Dimensions, Layout, StatusLine};

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
