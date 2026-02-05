//! Terminal host integration.

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

/// Terminal host for managing terminal state.
pub struct TerminalHost {
    /// Whether raw mode is enabled.
    raw_mode: bool,
    /// Whether alternate screen is active.
    alternate_screen: bool,
}

impl Default for TerminalHost {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalHost {
    /// Create new terminal host.
    pub fn new() -> Self {
        Self {
            raw_mode: false,
            alternate_screen: false,
        }
    }

    /// Enter terminal UI mode.
    pub fn enter(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        self.raw_mode = true;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        self.alternate_screen = true;

        Ok(())
    }

    /// Leave terminal UI mode.
    pub fn leave(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        if self.alternate_screen {
            execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
            self.alternate_screen = false;
        }

        if self.raw_mode {
            disable_raw_mode()?;
            self.raw_mode = false;
        }

        Ok(())
    }

    /// Get terminal size.
    pub fn size() -> io::Result<(u16, u16)> {
        crossterm::terminal::size()
    }

    /// Flush stdout.
    pub fn flush() -> io::Result<()> {
        io::stdout().flush()
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_creation() {
        let host = TerminalHost::new();
        assert!(!host.raw_mode);
        assert!(!host.alternate_screen);
    }
}
