//! Action types for core communication.

use kjxlkj_core_mode::Intent;
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::Dimensions;
use serde::{Deserialize, Serialize};

/// Actions that can be sent to the core task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Process an intent.
    Intent(Intent),

    /// Resize terminal.
    Resize(Dimensions),

    /// Open a file.
    OpenFile { path: String },

    /// Save current buffer.
    Save,

    /// Save buffer to path.
    SaveAs { path: String },

    /// Quit editor.
    Quit,

    /// Force quit.
    ForceQuit,

    /// Request snapshot.
    RequestSnapshot,
}

/// Result of an action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
    /// Action succeeded.
    Ok,

    /// Mode changed.
    ModeChanged(Mode),

    /// Editor should quit.
    Quit,

    /// Editor should save.
    Save,

    /// Editor should save and quit.
    SaveQuit,

    /// Open a file.
    OpenFile(String),

    /// Error occurred.
    Error(String),

    /// Snapshot ready.
    Snapshot,
}

impl Action {
    /// Creates a resize action.
    pub fn resize(width: u16, height: u16) -> Self {
        Self::Resize(Dimensions::new(width, height))
    }

    /// Creates an open file action.
    pub fn open(path: impl Into<String>) -> Self {
        Self::OpenFile { path: path.into() }
    }
}
