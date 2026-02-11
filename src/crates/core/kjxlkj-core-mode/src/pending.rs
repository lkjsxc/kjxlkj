//! Normal-mode pending state for multi-key sequences.
//!
//! Tracks count prefix, register prefix, and partial keys
//! (g-prefix, f/t/r char, z-commands, bracket-commands).
//! See /docs/spec/modes/normal.md for the key dispatch model.

/// Pending normal-mode input accumulator.
#[derive(Debug, Clone, Default)]
pub struct PendingState {
    /// Accumulated count prefix (None = no count entered yet).
    pub count: Option<usize>,
    /// Partial key state for multi-key sequences.
    pub partial: PartialKey,
}

/// What kind of partial key sequence we are waiting for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartialKey {
    /// No pending partial sequence.
    None,
    /// After pressing `g`, awaiting second key.
    G,
    /// After pressing `z`, awaiting second key.
    Z,
    /// After pressing `[`, awaiting second key.
    BracketOpen,
    /// After pressing `]`, awaiting second key.
    BracketClose,
    /// After pressing `f`, awaiting target char.
    FindForward,
    /// After pressing `F`, awaiting target char.
    FindBackward,
    /// After pressing `t`, awaiting target char.
    TillForward,
    /// After pressing `T`, awaiting target char.
    TillBackward,
    /// After pressing `r`, awaiting replacement char.
    ReplaceChar,
    /// After pressing `m`, awaiting mark char.
    SetMark,
    /// After pressing `'`, awaiting mark char.
    GotoMarkLine,
    /// After pressing `` ` ``, awaiting mark char.
    GotoMarkExact,
    /// After pressing `"`, awaiting register char.
    Register,
    /// After pressing `q`, awaiting register for macro.
    MacroRecord,
    /// After pressing `@`, awaiting register for macro play.
    MacroPlay,
}

impl Default for PartialKey {
    fn default() -> Self {
        Self::None
    }
}

impl PendingState {
    /// Reset all pending state.
    pub fn clear(&mut self) {
        self.count = None;
        self.partial = PartialKey::None;
    }

    /// Accumulate a digit into the count prefix.
    /// Returns true if the digit was consumed as count.
    pub fn push_digit(&mut self, d: u8) -> bool {
        // '0' is a motion (line start) unless preceded by
        // other digits already.
        if d == 0 && self.count.is_none() {
            return false;
        }
        let current = self.count.unwrap_or(0);
        self.count = Some(current * 10 + d as usize);
        true
    }

    /// Get effective count (1 if none entered).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_not_count_prefix() {
        let mut ps = PendingState::default();
        assert!(!ps.push_digit(0));
        assert_eq!(ps.count, None);
    }

    #[test]
    fn digit_accumulation() {
        let mut ps = PendingState::default();
        assert!(ps.push_digit(3));
        assert!(ps.push_digit(5));
        assert_eq!(ps.count, Some(35));
        assert_eq!(ps.effective_count(), 35);
    }

    #[test]
    fn zero_after_digit_works() {
        let mut ps = PendingState::default();
        assert!(ps.push_digit(1));
        assert!(ps.push_digit(0));
        assert_eq!(ps.count, Some(10));
    }

    #[test]
    fn clear_resets() {
        let mut ps = PendingState::default();
        ps.push_digit(5);
        ps.partial = PartialKey::G;
        ps.clear();
        assert_eq!(ps.count, None);
        assert_eq!(ps.partial, PartialKey::None);
    }
}
