//! Event loop implementation.

use anyhow::Result;
use crossterm::event::{poll, read};
use kjxlkj_core_state::EditorState;
use kjxlkj_input::decode_event;
use kjxlkj_render::TerminalRenderer;
use std::time::Duration;
use tracing::{debug, error, info};

/// Main event loop.
pub struct EventLoop {
    state: EditorState,
    renderer: TerminalRenderer,
}

impl EventLoop {
    /// Create a new event loop.
    pub fn new(state: EditorState) -> Result<Self> {
        let renderer = TerminalRenderer::new()?;
        Ok(Self { state, renderer })
    }

    /// Run the event loop.
    pub fn run(&mut self) -> Result<()> {
        info!("Starting event loop");

        // Initial render
        let snapshot = self.state.snapshot();
        self.renderer.render(&snapshot)?;

        loop {
            // Poll for events
            if poll(Duration::from_millis(100))? {
                let event = read()?;
                let editor_event = decode_event(event);

                debug!(?editor_event, "Received event");

                self.state.handle_event(editor_event);

                if self.state.should_quit() {
                    info!("Quit requested");
                    break;
                }

                // Render
                let snapshot = self.state.snapshot();
                if let Err(e) = self.renderer.render(&snapshot) {
                    error!(?e, "Render error");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_event_loop_creation() {
        // Can't easily test event loop without terminal
        // This is a placeholder for integration tests
    }
}
