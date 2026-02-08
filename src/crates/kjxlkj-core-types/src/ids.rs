//! Stable unique identifiers for editor entities.

use serde::{Deserialize, Serialize};

/// Stable unique identifier for a buffer. Monotonic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Stable unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

/// Stable unique identifier for a terminal instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalId(pub u64);

/// Stable unique identifier for a tab page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

/// Counter for generating monotonic IDs.
#[derive(Debug)]
pub struct IdGenerator {
    next_buffer: u64,
    next_window: u64,
    next_terminal: u64,
    next_tab: u64,
}

impl IdGenerator {
    /// Create a new generator starting from 1.
    pub fn new() -> Self {
        Self {
            next_buffer: 1,
            next_window: 1,
            next_terminal: 1,
            next_tab: 1,
        }
    }

    /// Generate the next buffer ID.
    pub fn next_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next_buffer);
        self.next_buffer += 1;
        id
    }

    /// Generate the next window ID.
    pub fn next_window(&mut self) -> WindowId {
        let id = WindowId(self.next_window);
        self.next_window += 1;
        id
    }

    /// Generate the next terminal ID.
    pub fn next_terminal(&mut self) -> TerminalId {
        let id = TerminalId(self.next_terminal);
        self.next_terminal += 1;
        id
    }

    /// Generate the next tab ID.
    pub fn next_tab(&mut self) -> TabId {
        let id = TabId(self.next_tab);
        self.next_tab += 1;
        id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_monotonic() {
        let mut gen = IdGenerator::new();
        let b1 = gen.next_buffer();
        let b2 = gen.next_buffer();
        assert_eq!(b1, BufferId(1));
        assert_eq!(b2, BufferId(2));
    }

    #[test]
    fn ids_are_independent() {
        let mut gen = IdGenerator::new();
        let b = gen.next_buffer();
        let w = gen.next_window();
        assert_eq!(b, BufferId(1));
        assert_eq!(w, WindowId(1));
    }
}
