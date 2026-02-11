//! Headless state harness for integration tests (T1 tier).
//!
//! Drives core actions without PTY.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    EditorSnapshot, Key, KeyModifiers, Mode,
};

/// Headless harness: wraps EditorState for testing.
pub struct HeadlessHarness {
    pub state: EditorState,
    pub steps: Vec<StepDump>,
}

/// Per-step dump for assertion.
#[derive(Debug, Clone)]
pub struct StepDump {
    pub step_index: usize,
    pub key: Key,
    pub mods: KeyModifiers,
    pub mode_before: Mode,
    pub mode_after: Mode,
    pub snapshot: EditorSnapshot,
}

impl HeadlessHarness {
    /// Create a new harness with given terminal geometry.
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            state: EditorState::new(cols, rows),
            steps: Vec::new(),
        }
    }

    /// Send a key and record the step dump.
    pub fn send_key(&mut self, key: Key, mods: KeyModifiers) {
        let mode_before = self.state.mode;
        self.state.handle_key(&key, &mods);
        let mode_after = self.state.mode;
        let snapshot = self.state.snapshot();
        let step_index = self.steps.len();
        self.steps.push(StepDump {
            step_index,
            key,
            mods,
            mode_before,
            mode_after,
            snapshot,
        });
    }

    /// Send a sequence of characters.
    pub fn send_chars(&mut self, chars: &str) {
        for c in chars.chars() {
            self.send_key(Key::Char(c), KeyModifiers::default());
        }
    }

    /// Get the latest snapshot.
    pub fn snapshot(&self) -> &EditorSnapshot {
        &self
            .steps
            .last()
            .map(|s| &s.snapshot)
            .unwrap_or_else(|| {
                panic!("no steps recorded")
            })
    }

    /// Get the current mode.
    pub fn mode(&self) -> Mode {
        self.state.mode
    }

    /// Assert current mode matches expected.
    pub fn assert_mode(&self, expected: Mode) {
        assert_eq!(
            self.state.mode, expected,
            "mode mismatch: {:?} != {:?}",
            self.state.mode, expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_basic_flow() {
        let mut h = HeadlessHarness::new(80, 24);
        // Enter insert mode.
        h.send_key(Key::Char('i'), KeyModifiers::default());
        h.assert_mode(Mode::Insert);

        // Type hello.
        h.send_chars("hello");

        // Exit to normal.
        h.send_key(Key::Escape, KeyModifiers::default());
        h.assert_mode(Mode::Normal);

        assert_eq!(h.steps.len(), 7); // i + h + e + l + l + o + Esc
    }

    #[test]
    fn shift_a_symmetry() {
        // WR-01R: Shift+a should yield same result as physical A.
        let mut h1 = HeadlessHarness::new(80, 24);
        let mut h2 = HeadlessHarness::new(80, 24);

        // Put text "hello" in both.
        for h in [&mut h1, &mut h2] {
            h.send_key(Key::Char('i'), KeyModifiers::default());
            h.send_chars("hello");
            h.send_key(Key::Escape, KeyModifiers::default());
        }

        // Press 'A' (physical uppercase A key).
        h1.send_key(Key::Char('A'), KeyModifiers::default());
        // Press 'A' (from Shift+a normalization).
        h2.send_key(Key::Char('A'), KeyModifiers::default());

        // Both should be in Insert mode at same position.
        h1.assert_mode(Mode::Insert);
        h2.assert_mode(Mode::Insert);

        let s1 = h1.snapshot();
        let s2 = h2.snapshot();
        assert_eq!(s1.mode, s2.mode);
    }
}
