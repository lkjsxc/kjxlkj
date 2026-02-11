//! Terminal emulator service.
//!
//! See /docs/spec/features/terminal/terminal.md.

pub mod csi;
pub mod escape_parser;
pub mod screen;

use kjxlkj_core_types::TerminalId;

/// Terminal instance state.
#[derive(Debug)]
pub struct TerminalState {
    pub id: TerminalId,
    /// Shell command used to spawn.
    pub shell: String,
    /// Terminal title (from OSC escape).
    pub title: String,
    /// Whether the child process has exited.
    pub exited: bool,
    /// Exit code if child has exited.
    pub exit_code: Option<i32>,
    /// Number of terminal columns.
    pub cols: u16,
    /// Number of terminal rows.
    pub rows: u16,
}

impl TerminalState {
    /// Create a new terminal state.
    pub fn new(id: TerminalId, shell: String, cols: u16, rows: u16) -> Self {
        Self {
            id, shell, title: String::new(), exited: false,
            exit_code: None, cols, rows,
        }
    }

    /// Mark the terminal as exited.
    pub fn set_exited(&mut self, code: i32) {
        self.exited = true;
        self.exit_code = Some(code);
    }

    /// Resize the terminal dimensions.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }
}

/// Service managing all terminal instances.
pub struct TerminalService;

impl TerminalService {
    pub fn new() -> Self { Self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_state_lifecycle() {
        let mut ts = TerminalState::new(TerminalId(0), "/bin/sh".into(), 80, 24);
        assert!(!ts.exited);
        assert_eq!(ts.cols, 80);
        ts.resize(120, 40);
        assert_eq!(ts.cols, 120);
        ts.set_exited(0);
        assert!(ts.exited);
        assert_eq!(ts.exit_code, Some(0));
    }

    #[test]
    fn terminal_title() {
        let mut ts = TerminalState::new(TerminalId(1), "bash".into(), 80, 24);
        ts.title = "my terminal".into();
        assert_eq!(ts.title, "my terminal");
    }
}

