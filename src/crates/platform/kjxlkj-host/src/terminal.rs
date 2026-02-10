//! Terminal setup and cleanup.

use crossterm::{
    cursor,
    event::{DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste, EnableFocusChange},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{self, Write};
use tracing::{info, error};

/// Terminal host state.
pub struct TerminalHost {
    /// Whether we're in raw mode.
    raw_mode: bool,
    /// Whether we're in alternate screen.
    alternate_screen: bool,
}

impl TerminalHost {
    /// Create a new terminal host.
    pub fn new() -> Self {
        Self {
            raw_mode: false,
            alternate_screen: false,
        }
    }

    /// Initialize the terminal and enter raw mode.
    pub fn init<W: Write>(_writer: W) -> io::Result<Self> {
        let mut host = Self::new();
        host.init_internal()?;
        Ok(host)
    }

    /// Internal initialization.
    fn init_internal(&mut self) -> io::Result<()> {
        info!("Initializing terminal");

        enable_raw_mode()?;
        self.raw_mode = true;

        let mut stdout = io::stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableBracketedPaste,
            EnableFocusChange,
            cursor::Hide,
        )?;
        self.alternate_screen = true;

        Ok(())
    }

    /// Get terminal size.
    pub fn size(&self) -> io::Result<(u16, u16)> {
        crossterm::terminal::size()
    }

    /// Restore the terminal.
    pub fn restore(&mut self) -> io::Result<()> {
        info!("Restoring terminal");

        let mut stdout = io::stdout();

        if self.alternate_screen {
            if let Err(e) = execute!(
                stdout,
                cursor::Show,
                DisableFocusChange,
                DisableBracketedPaste,
                LeaveAlternateScreen,
            ) {
                error!(?e, "Failed to leave alternate screen");
            }
            self.alternate_screen = false;
        }

        if self.raw_mode {
            if let Err(e) = disable_raw_mode() {
                error!(?e, "Failed to disable raw mode");
            }
            self.raw_mode = false;
        }

        stdout.flush()?;
        Ok(())
    }
}

impl Default for TerminalHost {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        if self.raw_mode || self.alternate_screen {
            let _ = self.restore();
        }
    }
}
