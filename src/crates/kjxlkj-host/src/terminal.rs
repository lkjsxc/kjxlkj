//! Terminal host.

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

/// Terminal host for managing terminal state.
pub struct TerminalHost {
    in_raw_mode: bool,
}

impl TerminalHost {
    /// Create a new terminal host.
    pub fn new() -> Self {
        Self { in_raw_mode: false }
    }

    /// Enter raw mode and alternate screen.
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        self.in_raw_mode = true;
        Ok(())
    }

    /// Leave raw mode and alternate screen.
    pub fn leave(&mut self) -> Result<()> {
        if self.in_raw_mode {
            execute!(io::stdout(), LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;
            self.in_raw_mode = false;
        }
        Ok(())
    }

    /// Get terminal size.
    pub fn size(&self) -> Result<(u16, u16)> {
        let (cols, rows) = terminal::size()?;
        Ok((cols, rows))
    }

    /// Poll for an event with timeout.
    pub fn poll_event(&self, timeout: std::time::Duration) -> Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    /// Read an event (blocking).
    pub fn read_event(&self) -> Result<Event> {
        Ok(event::read()?)
    }
}

impl Default for TerminalHost {
    fn default() -> Self {
        Self::new()
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
    fn test_terminal_host_creation() {
        let host = TerminalHost::new();
        assert!(!host.in_raw_mode);
    }
}
