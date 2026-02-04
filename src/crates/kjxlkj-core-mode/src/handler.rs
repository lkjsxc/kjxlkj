//! Mode handler trait and result types.

use kjxlkj_core_types::{Intent, KeyEvent, Mode};

/// Result of processing a key event in a mode.
#[derive(Debug, Clone)]
pub enum ModeResult {
    /// Key was consumed and produced intents.
    Consumed(Vec<Intent>),
    /// Key was consumed but more keys are needed.
    Pending,
    /// Key was not handled.
    Ignored,
}

impl ModeResult {
    /// Create a consumed result with a single intent.
    pub fn intent(intent: Intent) -> Self {
        ModeResult::Consumed(vec![intent])
    }

    /// Create a consumed result with multiple intents.
    pub fn intents(intents: Vec<Intent>) -> Self {
        ModeResult::Consumed(intents)
    }

    /// Create an empty consumed result (key was handled but no action).
    pub fn nop() -> Self {
        ModeResult::Consumed(vec![Intent::Nop])
    }
}

/// Trait for mode-specific key handling.
pub trait ModeHandler {
    /// Process a key event.
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult;

    /// Get the current mode.
    fn mode(&self) -> Mode;

    /// Reset any pending state.
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_result() {
        let result = ModeResult::intent(Intent::Nop);
        assert!(matches!(result, ModeResult::Consumed(_)));
    }
}
