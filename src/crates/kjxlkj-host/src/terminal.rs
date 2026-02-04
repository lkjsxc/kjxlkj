//! Terminal host for interactive editing.

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use kjxlkj_input::{InputEvent, Key};
use std::io::{self, Stdout, Write};

/// Terminal host manages terminal lifecycle.
pub struct TerminalHost {
    stdout: Stdout,
    in_raw_mode: bool,
}

impl TerminalHost {
    /// Create a new terminal host.
    pub fn new() -> Result<Self> {
        let stdout = io::stdout();
        Ok(Self {
            stdout,
            in_raw_mode: false,
        })
    }

    /// Enter raw mode and alternate screen.
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.in_raw_mode = true;
        self.stdout.execute(EnterAlternateScreen)?;
        Ok(())
    }

    /// Leave raw mode and alternate screen.
    pub fn leave(&mut self) -> Result<()> {
        self.stdout.execute(LeaveAlternateScreen)?;
        if self.in_raw_mode {
            terminal::disable_raw_mode()?;
            self.in_raw_mode = false;
        }
        Ok(())
    }

    /// Get terminal size.
    pub fn size(&self) -> Result<(u16, u16)> {
        let (w, h) = terminal::size()?;
        Ok((w, h))
    }

    /// Poll for input event with timeout.
    pub fn poll_event(&self, timeout_ms: u64) -> Result<Option<InputEvent>> {
        if event::poll(std::time::Duration::from_millis(timeout_ms))? {
            match event::read()? {
                Event::Key(key) => Ok(Some(InputEvent::Key(Key::from_crossterm(key)))),
                Event::Resize(w, h) => Ok(Some(InputEvent::Resize(w, h))),
                Event::FocusGained => Ok(Some(InputEvent::FocusGained)),
                Event::FocusLost => Ok(Some(InputEvent::FocusLost)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// Get stdout for rendering.
    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    /// Flush stdout.
    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        let _ = self.leave();
    }
}

impl Default for TerminalHost {
    fn default() -> Self {
        Self::new().expect("Failed to create terminal host")
    }
}

#[cfg(test)]
mod tests {
    // Terminal tests are hard to run in CI, so we keep them minimal

    #[test]
    fn host_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }
}
