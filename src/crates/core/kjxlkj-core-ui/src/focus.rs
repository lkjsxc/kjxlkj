//! Focus tracking for window navigation.
//!
//! See /docs/spec/editor/windows.md Focus Resolution section.

use kjxlkj_core_types::WindowId;

/// Tracks the focused window and focus history.
#[derive(Debug, Clone)]
pub struct FocusState {
    pub focused: WindowId,
    pub previous: Option<WindowId>,
    focus_seq: u64,
}

impl FocusState {
    pub fn new(initial: WindowId) -> Self {
        Self {
            focused: initial,
            previous: None,
            focus_seq: 0,
        }
    }

    /// Move focus to a new window.
    pub fn set_focus(&mut self, target: WindowId) {
        if target != self.focused {
            self.previous = Some(self.focused);
            self.focused = target;
            self.focus_seq += 1;
        }
    }

    /// Get monotonic focus sequence number.
    pub fn sequence(&self) -> u64 {
        self.focus_seq
    }

    /// Focus previous window (Ctrl-w p).
    pub fn toggle_previous(&mut self) {
        if let Some(prev) = self.previous {
            let old = self.focused;
            self.focused = prev;
            self.previous = Some(old);
            self.focus_seq += 1;
        }
    }

    /// Reset focus if current target was removed.
    pub fn on_window_closed(
        &mut self,
        closed: WindowId,
        fallback: WindowId,
    ) {
        if self.focused == closed {
            self.focused = fallback;
            self.focus_seq += 1;
        }
        if self.previous == Some(closed) {
            self.previous = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_toggle() {
        let mut f = FocusState::new(WindowId(0));
        f.set_focus(WindowId(1));
        assert_eq!(f.focused, WindowId(1));
        assert_eq!(f.previous, Some(WindowId(0)));

        f.toggle_previous();
        assert_eq!(f.focused, WindowId(0));
        assert_eq!(f.previous, Some(WindowId(1)));
    }

    #[test]
    fn closed_window_fallback() {
        let mut f = FocusState::new(WindowId(0));
        f.set_focus(WindowId(1));
        f.on_window_closed(WindowId(1), WindowId(0));
        assert_eq!(f.focused, WindowId(0));
    }
}
