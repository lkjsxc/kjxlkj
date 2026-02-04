//! Normal mode state machine.

mod awaiting;
mod keys;
mod motions;
mod operators;
mod state;

pub use state::NormalModeState;

use crate::Intent;

impl NormalModeState {
    /// Process a key and return an intent.
    pub fn process_key(&mut self, key: char, ctrl: bool, _shift: bool) -> Intent {
        // Handle awaiting character first
        if let Some(awaiting) = self.awaiting_char.take() {
            return self.handle_awaiting_char(awaiting, key);
        }

        // Handle Ctrl keys
        if ctrl {
            return self.handle_ctrl_key(key);
        }

        // Handle digits for count
        if key.is_ascii_digit() && !(key == '0' && self.count.is_none()) {
            let digit = key as usize - '0' as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return Intent::None;
        }

        let count = self.get_count();

        // Handle pending operator
        if let Some(op) = self.pending_operator.take() {
            return self.handle_operator_motion(op, key, count);
        }

        // Handle regular keys
        self.handle_normal_key(key, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_mode_movement() {
        let mut state = NormalModeState::new();
        let intent = state.process_key('j', false, false);
        assert!(matches!(intent, Intent::Move(_)));
    }

    #[test]
    fn normal_mode_count() {
        let mut state = NormalModeState::new();
        state.process_key('5', false, false);
        let intent = state.process_key('j', false, false);
        if let Intent::Move(motion) = intent {
            assert_eq!(motion.count, 5);
        } else {
            panic!("Expected Move intent");
        }
    }

    #[test]
    fn normal_mode_operator() {
        let mut state = NormalModeState::new();
        let intent1 = state.process_key('d', false, false);
        assert_eq!(intent1, Intent::None);
        let intent2 = state.process_key('d', false, false);
        assert!(matches!(intent2, Intent::Execute(_)));
    }
}
