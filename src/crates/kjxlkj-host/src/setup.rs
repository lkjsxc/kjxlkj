//! Terminal setup and teardown.

use crossterm::{
    cursor,
    event::{DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste, EnableFocusChange},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

/// Enter raw mode, alternate screen, enable bracketed paste
/// and focus reporting.
pub fn enter_raw_mode() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableBracketedPaste,
        EnableFocusChange,
        cursor::Hide,
        terminal::Clear(ClearType::All),
    )?;
    Ok(())
}

/// Restore terminal state.
pub fn leave_raw_mode() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::Show,
        DisableFocusChange,
        DisableBracketedPaste,
        LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Get current terminal dimensions.
pub fn terminal_size() -> anyhow::Result<(u16, u16)> {
    let (cols, rows) = terminal::size()?;
    Ok((cols, rows))
}
