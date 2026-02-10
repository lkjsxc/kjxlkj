//! Terminal service task.

use std::collections::HashMap;

use kjxlkj_core_types::TerminalId;

use crate::parser::Parser;
use crate::screen::Screen;

/// A single terminal instance.
pub struct TerminalInstance {
    pub id: TerminalId,
    pub screen: Screen,
    pub parser: Parser,
    pub exited: bool,
}

impl TerminalInstance {
    pub fn new(id: TerminalId, cols: usize, rows: usize) -> Self {
        Self {
            id,
            screen: Screen::new(cols, rows),
            parser: Parser::new(),
            exited: false,
        }
    }

    /// Feed output data from PTY to the parser.
    pub fn feed(&mut self, data: &[u8]) {
        self.parser.feed_bytes(data, &mut self.screen);
    }

    /// Resize this terminal instance.
    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.screen.resize(cols, rows);
    }
}

/// Terminal service manages PTY processes.
pub struct TerminalService {
    next_id: u64,
    pub terminals: HashMap<TerminalId, TerminalInstance>,
}

impl TerminalService {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            terminals: HashMap::new(),
        }
    }

    /// Allocate a new terminal id.
    pub fn next_id(&mut self) -> TerminalId {
        let id = TerminalId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Spawn a new terminal with the given dimensions.
    pub fn spawn(&mut self, cols: usize, rows: usize) -> TerminalId {
        let id = self.next_id();
        let instance = TerminalInstance::new(id, cols, rows);
        self.terminals.insert(id, instance);
        id
    }

    /// Feed PTY output data to a terminal.
    pub fn feed_output(&mut self, id: TerminalId, data: &[u8]) {
        if let Some(inst) = self.terminals.get_mut(&id) {
            inst.feed(data);
        }
    }

    /// Mark a terminal as exited.
    pub fn mark_exited(&mut self, id: TerminalId) {
        if let Some(inst) = self.terminals.get_mut(&id) {
            inst.exited = true;
        }
    }

    /// Remove a terminal.
    pub fn remove(&mut self, id: TerminalId) {
        self.terminals.remove(&id);
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}
