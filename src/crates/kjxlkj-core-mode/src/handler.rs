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

    #[test]
    fn test_mode_result_intents() {
        let result = ModeResult::intents(vec![Intent::Nop, Intent::Nop]);
        if let ModeResult::Consumed(intents) = result {
            assert_eq!(intents.len(), 2);
        } else {
            panic!("Expected Consumed");
        }
    }

    #[test]
    fn test_mode_result_nop() {
        let result = ModeResult::nop();
        if let ModeResult::Consumed(intents) = result {
            assert_eq!(intents.len(), 1);
            assert!(matches!(intents[0], Intent::Nop));
        } else {
            panic!("Expected Consumed");
        }
    }

    #[test]
    fn test_mode_result_pending() {
        let result = ModeResult::Pending;
        assert!(matches!(result, ModeResult::Pending));
    }

    #[test]
    fn test_mode_result_ignored() {
        let result = ModeResult::Ignored;
        assert!(matches!(result, ModeResult::Ignored));
    }

    #[test]
    fn test_mode_result_debug() {
        let result = ModeResult::Pending;
        let debug = format!("{:?}", result);
        assert!(debug.contains("Pending"));
    }

    #[test]
    fn test_mode_result_clone() {
        let result = ModeResult::Consumed(vec![Intent::Nop]);
        let cloned = result.clone();
        assert!(matches!(cloned, ModeResult::Consumed(_)));
    }

    #[test]
    fn test_mode_result_empty_intents() {
        let result = ModeResult::intents(vec![]);
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.is_empty());
        } else {
            panic!("Expected Consumed");
        }
    }

    #[test]
    fn test_mode_result_debug_consumed() {
        let result = ModeResult::Consumed(vec![Intent::Nop]);
        let debug = format!("{:?}", result);
        assert!(debug.contains("Consumed"));
    }

    #[test]
    fn test_mode_result_debug_ignored() {
        let result = ModeResult::Ignored;
        let debug = format!("{:?}", result);
        assert!(debug.contains("Ignored"));
    }

    #[test]
    fn test_mode_result_intent_switch_mode() {
        let result = ModeResult::intent(Intent::SwitchMode(Mode::Insert));
        if let ModeResult::Consumed(intents) = result {
            assert_eq!(intents.len(), 1);
            assert!(matches!(intents[0], Intent::SwitchMode(Mode::Insert)));
        } else {
            panic!("Expected Consumed");
        }
    }
}
