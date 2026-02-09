//! Terminal raw mode and alternate screen management.

use std::io::Write;

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

/// Terminal host: manages raw mode, alternate screen, mouse capture.
pub struct TerminalHost {
    /// Whether raw mode is active.
    raw_mode: bool,
    /// Whether alternate screen is active.
    alt_screen: bool,
}

impl TerminalHost {
    /// Create and initialize the terminal host.
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            raw_mode: false,
            alt_screen: false,
        })
    }

    /// Enter raw mode and alternate screen.
    pub fn enter(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        self.raw_mode = true;

        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            crossterm::event::EnableBracketedPaste,
            crossterm::event::EnableFocusChange,
            crossterm::event::EnableMouseCapture,
            cursor::Hide
        )?;
        self.alt_screen = true;

        // Clear the screen.
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        Ok(())
    }

    /// Restore terminal to normal state.
    pub fn leave(&mut self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();

        if self.alt_screen {
            execute!(
                stdout,
                cursor::Show,
                crossterm::event::DisableMouseCapture,
                crossterm::event::DisableFocusChange,
                crossterm::event::DisableBracketedPaste,
                terminal::LeaveAlternateScreen,
            )?;
            self.alt_screen = false;
        }

        if self.raw_mode {
            terminal::disable_raw_mode()?;
            self.raw_mode = false;
        }

        stdout.flush()?;
        Ok(())
    }

    /// Get terminal dimensions (cols, rows).
    pub fn size() -> std::io::Result<(u16, u16)> {
        terminal::size()
    }

    /// Whether raw mode is active.
    pub fn is_raw_mode(&self) -> bool {
        self.raw_mode
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
    fn terminal_host_creation() {
        let host = TerminalHost::new();
        assert!(host.is_ok());
        let host = host.unwrap();
        assert!(!host.is_raw_mode());
    }
}
