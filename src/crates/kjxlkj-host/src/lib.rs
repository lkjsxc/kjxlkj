//! Terminal host integration.
//!
//! This crate provides:
//! - Terminal setup and teardown
//! - Raw mode management
//! - Alternate screen handling

use anyhow::Result;
use crossterm::{
    cursor,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Stdout, Write};

/// Terminal host state.
pub struct TerminalHost {
    stdout: Stdout,
    is_raw: bool,
    is_alternate: bool,
}

impl TerminalHost {
    /// Create a new terminal host (does not initialize yet).
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            is_raw: false,
            is_alternate: false,
        }
    }

    /// Initialize the terminal for TUI mode.
    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.is_raw = true;

        execute!(
            self.stdout,
            EnterAlternateScreen,
            cursor::Hide,
        )?;
        self.is_alternate = true;

        Ok(())
    }

    /// Restore the terminal to normal mode.
    pub fn restore(&mut self) -> Result<()> {
        if self.is_alternate {
            execute!(
                self.stdout,
                cursor::Show,
                LeaveAlternateScreen,
            )?;
            self.is_alternate = false;
        }

        if self.is_raw {
            disable_raw_mode()?;
            self.is_raw = false;
        }

        Ok(())
    }

    /// Get the terminal size.
    pub fn size(&self) -> Result<(u16, u16)> {
        Ok(crossterm::terminal::size()?)
    }

    /// Get mutable access to stdout for rendering.
    pub fn stdout_mut(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    /// Flush stdout.
    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
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
        // Best-effort restoration on drop
        let _ = self.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_creation() {
        let host = TerminalHost::new();
        assert!(!host.is_raw);
        assert!(!host.is_alternate);
    }

    // Note: Full terminal tests require a real terminal
    // and are covered in E2E tests
}
