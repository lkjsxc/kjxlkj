//! Input decoding: terminal events → normalized keys.
//!
//! See /docs/spec/architecture/input-decoding.md for pipeline stages.

mod decode;
mod normalize;

pub use decode::decode_crossterm_event;
pub use normalize::normalize_key;

use kjxlkj_core_types::{Action, Key, KeyModifiers};

/// Decoded input event from the terminal.
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(Key, KeyModifiers),
    Resize(u16, u16),
    Paste(String),
    FocusGained,
    FocusLost,
    Ignored,
}

impl InputEvent {
    /// Convert to an Action for resize/paste/focus events.
    pub fn to_action(&self) -> Option<Action> {
        match self {
            Self::Resize(cols, rows) => {
                Some(Action::Resize(*cols, *rows))
            }
            Self::Paste(text) => {
                Some(Action::Paste(text.clone()))
            }
            Self::FocusGained => Some(Action::FocusGained),
            Self::FocusLost => Some(Action::FocusLost),
            _ => None,
        }
    }
}
