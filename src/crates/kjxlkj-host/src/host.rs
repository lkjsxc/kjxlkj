//! Terminal host implementation.

use crossterm::{
    cursor,
    event,
    execute, terminal,
};
use kjxlkj_core_state::Editor;
use kjxlkj_input::decode_event;
use kjxlkj_render::Renderer;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;

/// The terminal host.
pub struct Host {
    editor: Editor,
    renderer: Renderer<Stdout>,
}

impl Host {
    /// Create a new host.
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;

        Ok(Self {
            editor: Editor::new(width, height),
            renderer: Renderer::new(io::stdout()),
        })
    }

    /// Open a file in the editor.
    pub fn open_file(&mut self, path: &PathBuf) -> io::Result<()> {
        self.editor.open_file(path)
    }

    /// Run the main event loop.
    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        let result = self.event_loop();

        execute!(
            stdout,
            terminal::LeaveAlternateScreen,
            cursor::Show
        )?;
        terminal::disable_raw_mode()?;

        result
    }

    fn event_loop(&mut self) -> io::Result<()> {
        // Initial render
        self.render()?;

        loop {
            // Poll for events
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;

                // Decode crossterm event to EditorEvent
                if let Some(editor_event) = decode_event(event) {
                    self.editor.process_event(editor_event);
                }

                // Render
                self.render()?;

                // Check for quit
                if self.editor.quit_requested() {
                    break;
                }
            }
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let snapshot = self.editor.snapshot();
        self.renderer.render(&snapshot)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Most host tests require a TTY, so we keep them minimal
    #[test]
    fn test_host_module_loads() {
        assert!(true);
    }
}
