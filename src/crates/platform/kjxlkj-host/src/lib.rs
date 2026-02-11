//! Host terminal setup and teardown.
//!
//! See /docs/spec/architecture/startup.md for enter/leave sequence.

use anyhow::Result;
use crossterm::{
    event::{
        DisableBracketedPaste, DisableFocusChange,
        EnableBracketedPaste, EnableFocusChange,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

/// Enter raw mode, alternate screen, bracketed paste, focus reporting.
pub fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableBracketedPaste,
        EnableFocusChange
    )?;
    Ok(())
}

/// Restore terminal: leave alternate screen, disable raw mode, etc.
pub fn restore_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        crossterm::cursor::Show,
        LeaveAlternateScreen,
        DisableBracketedPaste,
        DisableFocusChange
    )?;
    disable_raw_mode()?;
    Ok(())
}

/// Get terminal size.
pub fn terminal_size() -> Result<(u16, u16)> {
    let (cols, rows) = crossterm::terminal::size()?;
    Ok((cols, rows))
}
