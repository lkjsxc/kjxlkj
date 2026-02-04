//! Terminal host and lifecycle management for kjxlkj editor.
//!
//! This crate manages the terminal lifecycle and event loop.

mod event_loop;

pub use event_loop::EventLoop;

use anyhow::Result;
use crossterm::terminal;
use kjxlkj_core_state::EditorState;
use kjxlkj_input::TerminalInput;
use kjxlkj_render::TerminalRenderer;
use std::path::PathBuf;

/// Host configuration.
#[derive(Debug, Clone)]
pub struct HostConfig {
    /// Initial file to open.
    pub file: Option<PathBuf>,
    /// Initial content (if no file).
    pub content: Option<String>,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            file: None,
            content: None,
        }
    }
}

/// Terminal host.
pub struct Host {
    config: HostConfig,
}

impl Host {
    /// Create a new host.
    pub fn new(config: HostConfig) -> Self {
        Self { config }
    }

    /// Run the editor.
    pub fn run(self) -> Result<()> {
        let mut state = EditorState::new();

        // Load initial content
        if let Some(path) = &self.config.file {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                state.load_file(path.clone(), &content);
            } else {
                state.load_file(path.clone(), "");
            }
        } else if let Some(content) = &self.config.content {
            state.load_content(content);
        }

        // Get initial terminal size
        let (width, height) = terminal::size()?;
        state.resize(width, height);

        // Create event loop
        let mut event_loop = EventLoop::new(state)?;
        event_loop.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_config_default() {
        let config = HostConfig::default();
        assert!(config.file.is_none());
        assert!(config.content.is_none());
    }
}
