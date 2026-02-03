//! Transaction grouping for atomic edits.

/// A transaction groups multiple edits into one undo step.
#[derive(Debug)]
pub struct Transaction {
    active: bool,
}

impl Transaction {
    /// Creates a new inactive transaction.
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Begins the transaction.
    pub fn begin(&mut self) {
        self.active = true;
    }

    /// Commits the transaction.
    pub fn commit(&mut self) {
        self.active = false;
    }

    /// Returns true if a transaction is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}
