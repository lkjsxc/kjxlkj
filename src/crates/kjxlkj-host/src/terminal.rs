use anyhow::Result;
use crossterm::{
    cursor,
    event::{DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste, EnableFocusChange},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io;

/// Host terminal state manager.
pub struct HostTerminal;

impl HostTerminal {
    /// Detect terminal size.
    pub fn size() -> Result<(u16, u16)> {
        let (cols, rows) = terminal::size()?;
        Ok((cols, rows))
    }
}

/// RAII guard for terminal raw mode and alternate screen.
///
/// On drop, restores terminal state.
pub struct TerminalGuard {
    _private: (),
}

impl TerminalGuard {
    /// Enter raw mode, alternate screen, enable bracketed paste
    /// and focus reporting.
    pub fn enter() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableBracketedPaste,
            EnableFocusChange,
            cursor::Hide,
        )?;
        Ok(Self { _private: () })
    }

    /// Explicitly restore terminal (also called on drop).
    pub fn restore(&self) {
        let mut stdout = io::stdout();
        let _ = execute!(
            stdout,
            cursor::Show,
            DisableFocusChange,
            DisableBracketedPaste,
            LeaveAlternateScreen,
        );
        let _ = disable_raw_mode();
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        self.restore();
    }
}
