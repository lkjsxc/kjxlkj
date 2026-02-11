//! PTY E2E test harness for kjxlkj
//!
//! Provides WR-* tests that verify key normalization, command wiring,
//! and deterministic behavior across the editor.

use kjxlkj_core_types::{KeyEvent, Mode};
use kjxlkj_core_mode::{dispatch_key, HandleResult, ModeState};
use kjxlkj_core_state::WindowTree;

#[cfg(test)]
mod wr_tests;
#[cfg(test)]
mod ime_tests;
#[cfg(test)]
mod ime_advanced_tests;
#[cfg(test)]
mod wrap_tests;
#[cfg(test)]
mod service_tests;

/// Test harness for headless state testing
pub struct StateHarness {
    pub mode_state: ModeState,
    pub windows: WindowTree,
}

impl StateHarness {
    /// Create a new test harness
    pub fn new() -> Self {
        Self {
            mode_state: ModeState::new(),
            windows: WindowTree::new(),
        }
    }

    /// Send a key and get the result
    pub fn send_key(&mut self, key: KeyEvent) -> HandleResult {
        dispatch_key(&mut self.mode_state, &key)
    }

    /// Get current mode
    pub fn mode(&self) -> &Mode {
        &self.mode_state.mode
    }
}

impl Default for StateHarness {
    fn default() -> Self {
        Self::new()
    }
}
