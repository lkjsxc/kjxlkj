//! Terminal abstraction.

use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use std::io::{self, Stdout, Write};

/// Terminal wrapper.
pub struct Terminal {
    stdout: Stdout,
    size: (u16, u16),
}

impl Terminal {
    /// Creates a new terminal.
    pub fn new() -> io::Result<Self> {
        let stdout = io::stdout();
        let size = terminal::size()?;
        Ok(Self { stdout, size })
    }

    /// Enters raw mode.
    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            event::EnableMouseCapture
        )
    }

    /// Exits raw mode.
    pub fn exit_raw_mode(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            event::DisableMouseCapture,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;
        terminal::disable_raw_mode()
    }

    /// Returns the terminal size.
    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    /// Updates the terminal size.
    pub fn update_size(&mut self) -> io::Result<()> {
        self.size = terminal::size()?;
        Ok(())
    }

    /// Clears the screen.
    pub fn clear(&mut self) -> io::Result<()> {
        execute!(self.stdout, terminal::Clear(ClearType::All))
    }

    /// Moves the cursor.
    pub fn move_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveTo(x, y))
    }

    /// Shows the cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)
    }

    /// Hides the cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)
    }

    /// Prints text at current position.
    pub fn print(&mut self, text: &str) -> io::Result<()> {
        execute!(self.stdout, Print(text))
    }

    /// Writes text at a specific position.
    pub fn write_at(&mut self, x: u16, y: u16, text: &str) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveTo(x, y), Print(text))
    }

    /// Flushes output.
    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    /// Polls for an event.
    pub fn poll_event(&self, timeout: std::time::Duration) -> io::Result<bool> {
        event::poll(timeout)
    }

    /// Reads an event.
    pub fn read_event(&self) -> io::Result<Event> {
        event::read()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.exit_raw_mode();
    }
}
