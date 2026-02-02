//! Mode state management.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

use crate::{CommandState, InsertState, NormalState, VisualState};

/// Aggregate mode state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Normal mode state.
    pub normal: NormalState,
    /// Insert mode state.
    pub insert: InsertState,
    /// Visual mode state.
    pub visual: VisualState,
    /// Command mode state.
    pub command: CommandState,
}

impl ModeState {
    /// Creates a new mode state in normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            normal: NormalState::new(),
            insert: InsertState::new(),
            visual: VisualState::new(),
            command: CommandState::new(),
        }
    }

    /// Transitions to a new mode.
    pub fn transition(&mut self, mode: Mode) {
        self.mode = mode;
        match mode {
            Mode::Normal => self.normal.reset(),
            Mode::Insert => self.insert.reset(),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {}
            Mode::Command => self.command.reset(),
            Mode::Replace => {}
            Mode::OperatorPending => {}
        }
    }

    /// Returns the mode name for display.
    pub fn mode_name(&self) -> &'static str {
        self.mode.name()
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}
