//! Terminal service.

use crate::Pty;
use std::collections::HashMap;
use std::path::PathBuf;

/// Terminal service managing PTYs.
pub struct TerminalService {
    /// PTYs by ID.
    ptys: HashMap<u64, Pty>,
    /// Next ID.
    next_id: u64,
    /// Default shell.
    shell: PathBuf,
}

impl TerminalService {
    /// Creates a new terminal service.
    pub fn new() -> Self {
        let shell = std::env::var("SHELL")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/bin/sh"));

        Self {
            ptys: HashMap::new(),
            next_id: 0,
            shell,
        }
    }

    /// Creates a new PTY.
    pub fn create(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let pty = Pty::new(id, self.shell.clone());
        self.ptys.insert(id, pty);
        id
    }

    /// Gets a PTY.
    pub fn get(&self, id: u64) -> Option<&Pty> {
        self.ptys.get(&id)
    }

    /// Gets a mutable PTY.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Pty> {
        self.ptys.get_mut(&id)
    }

    /// Removes a PTY.
    pub fn remove(&mut self, id: u64) {
        self.ptys.remove(&id);
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}
