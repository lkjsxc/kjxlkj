//! Terminal setup and event loop.

use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use kjxlkj_core::EditorState;
use kjxlkj_input::convert_event;
use kjxlkj_render::render_snapshot;

/// Terminal host for the editor.
pub struct TerminalHost {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    editor: EditorState,
}

impl TerminalHost {
    /// Create a new terminal host.
    pub fn new(editor: EditorState) -> Result<Self> {
        let terminal = setup_terminal()?;
        Ok(Self { terminal, editor })
    }

    /// Run the main event loop.
    pub fn run(&mut self) -> Result<()> {
        // Get initial terminal size
        let size = self.terminal.size()?;
        self.editor.set_terminal_size(size.width, size.height);

        loop {
            // Render
            let snapshot = self.editor.snapshot();
            self.terminal.draw(|frame| {
                render_snapshot(frame, &snapshot);
            })?;

            // Check for quit
            if self.editor.should_quit() {
                break;
            }

            // Wait for events
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;

                // Handle resize
                if let Event::Resize(width, height) = event {
                    self.editor.set_terminal_size(width, height);
                    continue;
                }

                // Handle key events (only on press)
                if let Event::Key(key_event) = &event {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                }

                // Convert and handle
                if let Some(key) = convert_event(event) {
                    if let Err(e) = self.editor.handle_key(key) {
                        self.editor.set_message(
                            &format!("Error: {}", e),
                            kjxlkj_core::MessageLevel::Error,
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

impl Drop for TerminalHost {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

/// Setup the terminal for raw mode.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to normal mode.
pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
