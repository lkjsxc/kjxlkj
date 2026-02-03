#![forbid(unsafe_code)]

use kjxlkj_core_types::Mode;

#[derive(Clone, Debug)]
pub struct ModeState {
    mode: Mode,
}

impl Default for ModeState {
    fn default() -> Self {
        Self { mode: Mode::Normal }
    }
}

impl ModeState {
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

