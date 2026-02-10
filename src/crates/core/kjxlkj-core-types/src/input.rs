//! Input action types.

use crate::KeyEvent;

/// Action from the input task.
#[derive(Debug, Clone)]
pub enum InputAction {
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
}
