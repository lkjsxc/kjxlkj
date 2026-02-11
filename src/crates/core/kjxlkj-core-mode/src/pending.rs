//! Normal-mode pending state for multi-key sequences.
//!
//! Tracks count prefix, register prefix, force modifier,
//! and partial keys (g-prefix, f/t/r char, z-commands, etc).
//! See /docs/spec/modes/normal.md for the key dispatch model.
//! See /docs/spec/editing/operators/operator-modifiers.md
//! for force modifier handling.

use kjxlkj_core_types::ForceModifier;

/// Pending normal-mode input accumulator.
#[derive(Debug, Clone, Default)]
pub struct PendingState {
    /// Accumulated count prefix (None = no count entered yet).
    pub count: Option<usize>,
    /// Partial key state for multi-key sequences.
    pub partial: PartialKey,
    /// Pre-operator count for count multiplication.
    /// Stored when entering OperatorPending mode.
    /// `2d3w` → pre_op_count=2, count=3 → effective=6.
    pub pre_op_count: Option<usize>,
    /// Force modifier (v/V/Ctrl-v) pressed between
    /// operator and motion in OperatorPending mode.
    pub force: Option<ForceModifier>,
    /// Selected register name from `"x` prefix.
    pub register: Option<char>,
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
    /// After `i` in operator-pending, awaiting text object char.
    TextObjectInner,
    /// After `a` in operator-pending, awaiting text object char.
    TextObjectAround,
    /// After `Ctrl-w`, awaiting wincmd char.
    WinCmd,
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
        self.pre_op_count = None;
        self.force = None;
        self.register = None;
    }

    /// Save current count as pre-operator count and
    /// reset count for post-operator accumulation.
    /// Called when entering OperatorPending mode.
    pub fn save_pre_op_count(&mut self) {
        self.pre_op_count = self.count;
        self.count = None;
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

    /// Compute multiplied count: pre_op × post_op.
    /// Both default to 1 if not set.
    pub fn multiplied_count(&self) -> usize {
        let pre = self.pre_op_count.unwrap_or(1);
        let post = self.count.unwrap_or(1);
        pre * post
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
        ps.register = Some('a');
        ps.clear();
        assert_eq!(ps.count, None);
        assert_eq!(ps.partial, PartialKey::None);
        assert!(ps.register.is_none());
        assert!(ps.pre_op_count.is_none());
        assert!(ps.force.is_none());
    }

    #[test]
    fn pre_op_count_multiplication() {
        let mut ps = PendingState::default();
        ps.push_digit(2);
        ps.save_pre_op_count();
        assert_eq!(ps.pre_op_count, Some(2));
        assert_eq!(ps.count, None);
        ps.push_digit(3);
        assert_eq!(ps.multiplied_count(), 6);
    }

    #[test]
    fn multiplied_count_defaults() {
        let ps = PendingState::default();
        assert_eq!(ps.multiplied_count(), 1);
    }
}
