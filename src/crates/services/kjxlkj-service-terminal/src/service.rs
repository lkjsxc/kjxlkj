//! Terminal service.

use crate::{ParseAction, Parser, Pty, PtyError, Screen};
use kjxlkj_core_types::TerminalId;
use std::collections::HashMap;
use thiserror::Error;
use tracing::info;

/// Terminal service error.
#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("Terminal not found: {0:?}")]
    NotFound(TerminalId),
    #[error("PTY error: {0}")]
    Pty(#[from] PtyError),
}

/// Terminal instance.
pub struct TerminalInstance {
    /// PTY process.
    pty: Pty,
    /// Screen buffer.
    screen: Screen,
    /// Parser.
    parser: Parser,
}

/// Terminal service.
pub struct TerminalService {
    /// Terminal instances.
    terminals: HashMap<TerminalId, TerminalInstance>,
    /// Next terminal ID.
    next_id: u32,
    /// Default shell.
    shell: String,
}

impl TerminalService {
    /// Create a new terminal service.
    pub fn new() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        Self {
            terminals: HashMap::new(),
            next_id: 1,
            shell,
        }
    }

    /// Spawn a new terminal.
    pub async fn spawn(&mut self, width: u16, height: u16) -> Result<TerminalId, TerminalError> {
        let pty = Pty::spawn(&self.shell).await?;
        let screen = Screen::new(width, height);
        let parser = Parser::new();

        let id = TerminalId(self.next_id);
        self.next_id += 1;

        self.terminals.insert(
            id,
            TerminalInstance {
                pty,
                screen,
                parser,
            },
        );

        info!(?id, "Spawned terminal");
        Ok(id)
    }

    /// Write to a terminal.
    pub async fn write(&self, id: TerminalId, data: &[u8]) -> Result<(), TerminalError> {
        let term = self.terminals.get(&id).ok_or(TerminalError::NotFound(id))?;
        term.pty.write(data).await?;
        Ok(())
    }

    /// Process output from a terminal.
    pub async fn process_output(&mut self, id: TerminalId) -> Result<bool, TerminalError> {
        let term = self.terminals.get_mut(&id).ok_or(TerminalError::NotFound(id))?;

        if let Some(data) = term.pty.read().await {
            for byte in data {
                let actions = term.parser.parse(byte);
                for action in actions {
                    match action {
                        ParseAction::Print(ch) => term.screen.print(ch),
                        ParseAction::Newline => term.screen.newline(),
                        ParseAction::CarriageReturn => term.screen.carriage_return(),
                        ParseAction::Backspace => {
                            let (x, _y) = term.screen.cursor();
                            if x > 0 {
                                // Move cursor back.
                            }
                        }
                        ParseAction::Bell => {
                            // Ignore bell.
                        }
                        ParseAction::CsiDispatch(_) => {
                            // Handle CSI sequences.
                        }
                        ParseAction::OscDispatch => {
                            // Handle OSC sequences.
                        }
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get screen for a terminal.
    pub fn screen(&self, id: TerminalId) -> Option<&Screen> {
        self.terminals.get(&id).map(|t| &t.screen)
    }

    /// Close a terminal.
    pub fn close(&mut self, id: TerminalId) {
        self.terminals.remove(&id);
        info!(?id, "Closed terminal");
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}
