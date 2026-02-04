//! Terminal host lifecycle.

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io::{stdout, Stdout};
use std::time::Duration;

/// Terminal host for managing terminal state.
pub struct TerminalHost {
    stdout: Stdout,
    in_raw_mode: bool,
}

impl TerminalHost {
    /// Create and initialize the terminal host.
    pub fn new() -> Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(Self {
            stdout,
            in_raw_mode: true,
        })
    }

    /// Get a mutable reference to stdout.
    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    /// Get terminal size (width, height).
    pub fn size(&self) -> Result<(u16, u16)> {
        let (w, h) = terminal::size()?;
        Ok((w, h))
    }

    /// Poll for an event with timeout.
    pub fn poll_event(&self, timeout: Duration) -> Result<Option<Event>> {
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

    /// Restore terminal state.
    pub fn restore(&mut self) -> Result<()> {
        if self.in_raw_mode {
            execute!(self.stdout, LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;
            self.in_raw_mode = false;
        }
        Ok(())
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[cfg(test)]
mod tests {
    // Terminal tests require actual terminal, skip in unit tests.
}
