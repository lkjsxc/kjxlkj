//! Actions that can be sent to the core.

use kjxlkj_core_types::KeyEvent;
use std::path::PathBuf;

/// Action that can be sent to the core task.
#[derive(Debug, Clone)]
pub enum Action {
    /// Key event.
    Key(KeyEvent),
    /// Resize terminal.
    Resize(u16, u16),
    /// Paste text.
    Paste(String),
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
    /// Quit.
    Quit,
    /// Open a buffer.
    OpenBuffer {
        /// File path.
        path: Option<PathBuf>,
        /// Initial content.
        content: Option<String>,
    },
}
