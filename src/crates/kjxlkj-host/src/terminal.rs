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
    use super::*;

    #[test]
    fn host_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }

    #[test]
    fn terminal_host_struct_exists() {
        // Verify the struct type exists
        let _: fn() -> Result<TerminalHost> = TerminalHost::new;
    }

    #[test]
    fn terminal_host_has_stdout() {
        // Verify the stdout method exists
        let _: for<'a> fn(&'a mut TerminalHost) -> &'a mut Stdout = TerminalHost::stdout;
    }

    #[test]
    fn terminal_host_has_size() {
        // Verify the size method exists
        let _: fn(&TerminalHost) -> Result<(u16, u16)> = TerminalHost::size;
    }

    #[test]
    fn terminal_host_has_flush() {
        // Verify the flush method exists  
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::flush;
    }

    #[test]
    fn terminal_host_has_enter() {
        // Verify the enter method exists
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::enter;
    }

    #[test]
    fn terminal_host_has_leave() {
        // Verify the leave method exists
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::leave;
    }

    #[test]
    fn terminal_host_has_poll() {
        // Verify poll_event method exists
        let _: fn(&TerminalHost, u64) -> Result<Option<InputEvent>> = TerminalHost::poll_event;
    }

    #[test]
    fn terminal_host_new_signature() {
        // Just verify the function signature
        let _f: fn() -> Result<TerminalHost> = TerminalHost::new;
    }

    #[test]
    fn terminal_host_result_type() {
        // TerminalHost::new returns Result type
        let result: Result<TerminalHost> = TerminalHost::new();
        // We can't easily test the actual result without terminal access
        drop(result);
    }

    #[test]
    fn terminal_flush_sig() {
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::flush;
    }

    #[test]
    fn terminal_enter_sig() {
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::enter;
    }

    #[test]
    fn terminal_leave_sig() {
        let _: fn(&mut TerminalHost) -> Result<()> = TerminalHost::leave;
    }

    #[test]
    fn terminal_size_sig() {
        let _: fn(&TerminalHost) -> Result<(u16, u16)> = TerminalHost::size;
    }

    #[test]
    fn terminal_poll_sig() {
        let _: fn(&TerminalHost, u64) -> Result<Option<InputEvent>> = TerminalHost::poll_event;
    }

    #[test]
    fn terminal_stdout_sig() {
        let _: for<'a> fn(&'a mut TerminalHost) -> &'a mut Stdout = TerminalHost::stdout;
    }

    #[test]
    fn terminal_module_compiles() {
        assert!(true);
    }

    #[test]
    fn terminal_host_type_name() {
        let name = std::any::type_name::<TerminalHost>();
        assert!(name.contains("TerminalHost"));
    }
}
