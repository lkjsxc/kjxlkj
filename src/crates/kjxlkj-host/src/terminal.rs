//! Terminal host.

use anyhow::Result;
use crossterm::{
    cursor,
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use kjxlkj_core_ui::Dimensions;
use std::io::{self, Write};

/// Terminal host managing terminal state.
pub struct TerminalHost {
    /// Original terminal state preserved.
    _original_mode: bool,
}

impl TerminalHost {
    /// Initializes the terminal host.
    pub fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            cursor::Hide,
        )?;
        Ok(Self {
            _original_mode: true,
        })
    }

    /// Returns the terminal dimensions.
    pub fn dimensions(&self) -> Result<Dimensions> {
        let (width, height) = terminal::size()?;
        Ok(Dimensions::new(width, height))
    }

    /// Shows the cursor.
    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(io::stdout(), cursor::Show)?;
        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(io::stdout(), cursor::Hide)?;
        Ok(())
    }

    /// Moves the cursor.
    pub fn move_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        execute!(io::stdout(), cursor::MoveTo(x, y))?;
        Ok(())
    }

    /// Flushes stdout.
    pub fn flush(&mut self) -> Result<()> {
        io::stdout().flush()?;
        Ok(())
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        let _ = execute!(
            io::stdout(),
            cursor::Show,
            LeaveAlternateScreen,
        );
        let _ = terminal::disable_raw_mode();
    }
}
