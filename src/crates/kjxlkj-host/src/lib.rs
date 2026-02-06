//! Terminal host integration — raw mode, resize, lifecycle.

mod supervisor;
mod session_commands;
mod file_io_commands;
mod script_files;
mod session_full;
mod feature_integration;
mod file_flows;
mod feature_reachability;

use kjxlkj_core_types::Size;

/// Manages the terminal lifecycle: raw mode, alternate screen, cleanup.
pub struct TerminalHost {
    original_size: Option<Size>,
}

impl TerminalHost {
    pub fn new() -> Self {
        Self {
            original_size: None,
        }
    }

    /// Enter raw mode and switch to the alternate screen.
    pub fn enter(&mut self) -> anyhow::Result<()> {
        let (w, h) = crossterm::terminal::size()?;
        self.original_size = Some(Size::new(w, h));
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::cursor::Hide
        )?;
        tracing::info!(width = w, height = h, "entered raw mode");
        Ok(())
    }

    /// Leave raw mode and restore the main screen.
    pub fn leave(&mut self) -> anyhow::Result<()> {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;
        tracing::info!("left raw mode");
        Ok(())
    }

    /// Query the current terminal size.
    pub fn size(&self) -> anyhow::Result<Size> {
        let (w, h) = crossterm::terminal::size()?;
        Ok(Size::new(w, h))
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
