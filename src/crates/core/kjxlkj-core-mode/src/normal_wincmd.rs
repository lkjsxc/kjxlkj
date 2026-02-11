//! Ctrl-w (wincmd) key dispatch.
//!
//! See /docs/spec/features/window/wincmd.md for the
//! normative command catalog.

use kjxlkj_core_types::{Action, Direction, Key, Mode};

use crate::pending::PendingState;

/// Resolve the second key after Ctrl-w.
pub(crate) fn handle_wincmd_key(
    key: &Key,
    pending: &mut PendingState,
) -> (Action, Option<Mode>) {
    pending.clear();
    match key {
        // Navigation: directional focus
        Key::Char('h') => (Action::FocusDirection(Direction::Left), None),
        Key::Char('j') => (Action::FocusDirection(Direction::Down), None),
        Key::Char('k') => (Action::FocusDirection(Direction::Up), None),
        Key::Char('l') => (Action::FocusDirection(Direction::Right), None),
        // Navigation: cycle / previous
        Key::Char('w') => (Action::FocusCycle, None),
        Key::Char('p') => (Action::FocusPrevious, None),
        // Split / close
        Key::Char('s') => (Action::SplitHorizontal, None),
        Key::Char('v') => (Action::SplitVertical, None),
        Key::Char('n') => (Action::SplitHorizontal, None),
        Key::Char('c') | Key::Char('q') => (Action::CloseWindow, None),
        Key::Char('o') => (Action::WindowOnly, None),
        _ => (Action::Noop, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wincmd_h_focuses_left() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('h'), &mut ps);
        assert_eq!(a, Action::FocusDirection(Direction::Left));
    }

    #[test]
    fn wincmd_s_splits_horizontal() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('s'), &mut ps);
        assert_eq!(a, Action::SplitHorizontal);
    }

    #[test]
    fn wincmd_v_splits_vertical() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('v'), &mut ps);
        assert_eq!(a, Action::SplitVertical);
    }

    #[test]
    fn wincmd_c_closes_window() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('c'), &mut ps);
        assert_eq!(a, Action::CloseWindow);
    }

    #[test]
    fn wincmd_o_window_only() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('o'), &mut ps);
        assert_eq!(a, Action::WindowOnly);
    }

    #[test]
    fn wincmd_w_cycles_focus() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('w'), &mut ps);
        assert_eq!(a, Action::FocusCycle);
    }

    #[test]
    fn wincmd_p_focuses_previous() {
        let mut ps = PendingState::default();
        let (a, _) = handle_wincmd_key(&Key::Char('p'), &mut ps);
        assert_eq!(a, Action::FocusPrevious);
    }
}
