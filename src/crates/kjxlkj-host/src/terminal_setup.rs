//! Terminal raw-mode setup and cleanup utilities.

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

/// Enter raw mode and alternate screen.
pub fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    tracing::debug!("terminal: raw mode + alternate screen enabled");
    Ok(())
}

/// Leave raw mode and restore the original screen.
pub fn restore_terminal() -> Result<()> {
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    tracing::debug!("terminal: restored");
    Ok(())
}

/// RAII guard that restores the terminal on drop.
pub struct TerminalGuard {
    active: bool,
}

impl TerminalGuard {
    /// Activate the guard (calls `setup_terminal`).
    pub fn new() -> Result<Self> {
        setup_terminal()?;
        Ok(Self { active: true })
    }

    /// Manually release the guard early.
    pub fn release(&mut self) -> Result<()> {
        if self.active {
            self.active = false;
            restore_terminal()?;
        }
        Ok(())
    }

    /// Whether the guard is still active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = restore_terminal();
            self.active = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Terminal functions can't be tested in CI without a tty,
    // so we test the guard logic in a headless-safe way.

    #[test]
    fn guard_double_release() {
        // Simulate guard without calling setup (headless).
        let mut g = TerminalGuard { active: false };
        assert!(!g.is_active());
        assert!(g.release().is_ok());
    }

    #[test]
    fn guard_inactive_drop() {
        let g = TerminalGuard { active: false };
        drop(g); // should not panic
    }
}
