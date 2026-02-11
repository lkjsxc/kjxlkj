//! Ctrl-w (wincmd) key dispatch.
//!
//! See /docs/spec/features/window/wincmd.md for the
//! normative command catalog.

use kjxlkj_core_types::{Action, Direction, Key, Mode, ResizeEdge};

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
        // Navigation: cycle / previous / boundary
        Key::Char('w') => (Action::FocusCycle, None),
        Key::Char('p') => (Action::FocusPrevious, None),
        Key::Char('t') => (Action::FocusTopLeft, None),
        Key::Char('b') => (Action::FocusBottomRight, None),
        // Split / close
        Key::Char('s') => (Action::SplitHorizontal, None),
        Key::Char('v') => (Action::SplitVertical, None),
        Key::Char('n') => (Action::SplitHorizontal, None),
        Key::Char('c') | Key::Char('q') => (Action::CloseWindow, None),
        Key::Char('o') => (Action::WindowOnly, None),
        // Resize
        Key::Char('+') => (Action::WindowResize(ResizeEdge::Height, 1), None),
        Key::Char('-') => (Action::WindowResize(ResizeEdge::Height, -1), None),
        Key::Char('>') => (Action::WindowResize(ResizeEdge::Width, 1), None),
        Key::Char('<') => (Action::WindowResize(ResizeEdge::Width, -1), None),
        Key::Char('=') => (Action::WindowEqualize, None),
        Key::Char('_') => (Action::WindowMaxHeight, None),
        Key::Char('|') => (Action::WindowMaxWidth, None),
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
    fn wincmd_c_closes() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('c'), &mut ps).0, Action::CloseWindow);
    }

    #[test]
    fn wincmd_o_window_only() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('o'), &mut ps).0, Action::WindowOnly);
    }

    #[test]
    fn wincmd_w_cycles() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('w'), &mut ps).0, Action::FocusCycle);
    }

    #[test]
    fn wincmd_p_previous() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('p'), &mut ps).0, Action::FocusPrevious);
    }

    #[test]
    fn wincmd_t_top_left() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('t'), &mut ps).0, Action::FocusTopLeft);
    }

    #[test]
    fn wincmd_b_bottom_right() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('b'), &mut ps).0, Action::FocusBottomRight);
    }

    #[test]
    fn wincmd_plus_resize_height() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('+'), &mut ps).0,
                   Action::WindowResize(ResizeEdge::Height, 1));
    }

    #[test]
    fn wincmd_minus_resize_height() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('-'), &mut ps).0,
                   Action::WindowResize(ResizeEdge::Height, -1));
    }

    #[test]
    fn wincmd_eq_equalize() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('='), &mut ps).0, Action::WindowEqualize);
    }

    #[test]
    fn wincmd_underscore_max_height() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('_'), &mut ps).0, Action::WindowMaxHeight);
    }

    #[test]
    fn wincmd_pipe_max_width() {
        let mut ps = PendingState::default();
        assert_eq!(handle_wincmd_key(&Key::Char('|'), &mut ps).0, Action::WindowMaxWidth);
    }
}
