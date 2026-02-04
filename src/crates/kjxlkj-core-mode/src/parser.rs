//! Input parsing for key sequences.

/// Result of parsing input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult {
    /// Need more input.
    Pending,
    /// Complete command parsed.
    Complete,
    /// Invalid input sequence.
    Invalid,
}

/// Pending input state for multi-key sequences.
#[derive(Debug, Clone, Default)]
pub struct PendingInput {
    /// Accumulated count prefix.
    count: Option<usize>,
    /// Operator waiting for motion (d, c, y, etc.).
    operator: Option<char>,
    /// Register selection.
    register: Option<char>,
    /// Partial key sequence.
    keys: Vec<char>,
}

impl PendingInput {
    /// Create empty pending input.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a key to the pending input.
    pub fn push(&mut self, c: char) {
        self.keys.push(c);
    }

    /// Clear all pending input.
    pub fn clear(&mut self) {
        self.count = None;
        self.operator = None;
        self.register = None;
        self.keys.clear();
    }

    /// Get the accumulated count.
    pub fn count(&self) -> Option<usize> {
        self.count
    }

    /// Set the count.
    pub fn set_count(&mut self, n: usize) {
        self.count = Some(n);
    }

    /// Get the pending operator.
    pub fn operator(&self) -> Option<char> {
        self.operator
    }

    /// Set the pending operator.
    pub fn set_operator(&mut self, op: char) {
        self.operator = Some(op);
    }

    /// Get the selected register.
    pub fn register(&self) -> Option<char> {
        self.register
    }

    /// Set the register.
    pub fn set_register(&mut self, reg: char) {
        self.register = Some(reg);
    }

    /// Get the key sequence.
    pub fn keys(&self) -> &[char] {
        &self.keys
    }

    /// Check if there is pending input.
    pub fn is_empty(&self) -> bool {
        self.count.is_none()
            && self.operator.is_none()
            && self.register.is_none()
            && self.keys.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_input() {
        let mut pending = PendingInput::new();
        assert!(pending.is_empty());

        pending.set_count(3);
        pending.set_operator('d');
        assert!(!pending.is_empty());
        assert_eq!(pending.count(), Some(3));
        assert_eq!(pending.operator(), Some('d'));

        pending.clear();
        assert!(pending.is_empty());
    }
}
