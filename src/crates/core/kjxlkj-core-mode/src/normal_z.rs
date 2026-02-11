//! Normal mode `z`-prefix key dispatch.
//!
//! See /docs/spec/modes/normal.md § z-prefix commands
//! and /docs/spec/editing/motions/scroll-motions.md.

use kjxlkj_core_types::{Action, Key, Mode};

use crate::pending::PendingState;

/// Handle the second key after `z` was pressed.
pub(crate) fn handle_z_key(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    pending.clear();
    match key {
        Key::Char('z') => (Action::ScrollCenter, None),
        Key::Char('t') => (Action::ScrollTop, None),
        Key::Char('b') => (Action::ScrollBottom, None),
        _ => (Action::Noop, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zz_scrolls_center() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_z_key(&Key::Char('z'), &mut ps);
        assert_eq!(action, Action::ScrollCenter);
    }

    #[test]
    fn zt_scrolls_top() {
        let mut ps = PendingState::default();
        let (action, _) =
            handle_z_key(&Key::Char('t'), &mut ps);
        assert_eq!(action, Action::ScrollTop);
    }
}
