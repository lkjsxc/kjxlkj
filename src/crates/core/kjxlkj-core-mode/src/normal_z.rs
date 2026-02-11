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
        // Scroll commands
        Key::Char('z') => (Action::ScrollCenter, None),
        Key::Char('t') => (Action::ScrollTop, None),
        Key::Char('b') => (Action::ScrollBottom, None),
        // Fold commands
        Key::Char('o') => (Action::FoldOpen, None),
        Key::Char('c') => (Action::FoldClose, None),
        Key::Char('a') => (Action::FoldToggle, None),
        Key::Char('R') => (Action::FoldOpenAll, None),
        Key::Char('M') => (Action::FoldCloseAll, None),
        Key::Char('r') => (Action::FoldReduce, None),
        Key::Char('m') => (Action::FoldMore, None),
        Key::Char('j') => (Action::FoldNext, None),
        Key::Char('k') => (Action::FoldPrev, None),
        _ => (Action::Noop, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zz_scrolls_center() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('z'), &mut ps);
        assert_eq!(action, Action::ScrollCenter);
    }

    #[test]
    fn zt_scrolls_top() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('t'), &mut ps);
        assert_eq!(action, Action::ScrollTop);
    }

    #[test]
    fn zo_fold_open() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('o'), &mut ps);
        assert_eq!(action, Action::FoldOpen);
    }

    #[test]
    fn zc_fold_close() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('c'), &mut ps);
        assert_eq!(action, Action::FoldClose);
    }

    #[test]
    fn za_fold_toggle() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('a'), &mut ps);
        assert_eq!(action, Action::FoldToggle);
    }

    #[test]
    fn z_big_r_fold_open_all() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('R'), &mut ps);
        assert_eq!(action, Action::FoldOpenAll);
    }

    #[test]
    fn z_big_m_fold_close_all() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('M'), &mut ps);
        assert_eq!(action, Action::FoldCloseAll);
    }

    #[test]
    fn zr_fold_reduce() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('r'), &mut ps);
        assert_eq!(action, Action::FoldReduce);
    }

    #[test]
    fn zm_fold_more() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('m'), &mut ps);
        assert_eq!(action, Action::FoldMore);
    }

    #[test]
    fn zj_fold_next() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('j'), &mut ps);
        assert_eq!(action, Action::FoldNext);
    }

    #[test]
    fn zk_fold_prev() {
        let mut ps = PendingState::default();
        let (action, _) = handle_z_key(&Key::Char('k'), &mut ps);
        assert_eq!(action, Action::FoldPrev);
    }
}
