//! Stable identity types.

use serde::{Deserialize, Serialize};

/// Stable buffer identity (monotonic counter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Stable window identity for layout tree leaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

/// Explorer state identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExplorerStateId(pub u64);

/// Terminal instance identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalId(pub u64);

/// Monotonically increasing buffer edit counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Increment the version counter.
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Counter for generating unique IDs.
#[derive(Debug, Default)]
pub struct IdGenerator {
    next: u64,
}

impl IdGenerator {
    pub fn next_buffer(&mut self) -> BufferId {
        let id = BufferId(self.next);
        self.next += 1;
        id
    }

    pub fn next_window(&mut self) -> WindowId {
        let id = WindowId(self.next);
        self.next += 1;
        id
    }

    pub fn next_explorer(&mut self) -> ExplorerStateId {
        let id = ExplorerStateId(self.next);
        self.next += 1;
        id
    }

    pub fn next_terminal(&mut self) -> TerminalId {
        let id = TerminalId(self.next);
        self.next += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generator_monotonic() {
        let mut gen = IdGenerator::default();
        let b1 = gen.next_buffer();
        let b2 = gen.next_buffer();
        assert_ne!(b1, b2);
        assert!(b1.0 < b2.0);
    }

    #[test]
    fn buffer_version_increment() {
        let v = BufferVersion(0);
        assert_eq!(v.next(), BufferVersion(1));
    }
}
