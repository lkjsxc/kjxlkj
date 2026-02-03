//! Terminal host implementation.

use std::io::{self, stdout, Stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use kjxlkj_core::EditorState;
use kjxlkj_input::InputDecoder;
use kjxlkj_render::Renderer;

/// Terminal host that manages the editor lifecycle.
pub struct TerminalHost {
    state: EditorState,
    renderer: Renderer<Stdout>,
}

impl TerminalHost {
    /// Creates a new terminal host.
    pub fn new() -> io::Result<Self> {
        let state = EditorState::new();
        let renderer = Renderer::new(stdout());
        Ok(Self { state, renderer })
    }

    /// Creates a terminal host with initial content.
    pub fn with_content(content: &str) -> io::Result<Self> {
        let state = EditorState::with_content(content);
        let renderer = Renderer::new(stdout());
        Ok(Self { state, renderer })
    }

    /// Enters the terminal and runs the editor loop.
    pub fn run(&mut self) -> io::Result<()> {
        self.enter_terminal()?;
        let result = self.event_loop();
        self.leave_terminal()?;
        result
    }

    fn enter_terminal(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let (width, height) = terminal::size()?;
        self.state.set_terminal_size(width, height);
        Ok(())
    }

    fn leave_terminal(&mut self) -> io::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            let snapshot = self.state.snapshot();
            self.renderer.render(&snapshot)?;

            if self.state.is_quit_requested() {
                break;
            }

            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;
                if let Event::Resize(w, h) = event {
                    self.state.set_terminal_size(w, h);
                    continue;
                }
                if let Some(key_input) = InputDecoder::decode(event) {
                    self.state.handle_key(key_input);
                }
            }
        }
        Ok(())
    }
}

impl Default for TerminalHost {
    fn default() -> Self {
        Self::new().expect("Failed to create terminal host")
    }
}
