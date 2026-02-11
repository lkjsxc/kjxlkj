//! Macro recording and playback.
//! See /docs/spec/features/session/macros.md.
//!
//! `q{a-z}` starts recording, subsequent `q` stops.
//! `@{a-z}` plays the macro stored in that register.

use kjxlkj_core_types::{Key, KeyModifiers};

/// Captured key event for macro replay.
#[derive(Debug, Clone)]
pub struct MacroKey {
    pub key: Key,
    pub mods: KeyModifiers,
}

/// Macro recording state.
#[derive(Debug, Clone, Default)]
pub struct MacroState {
    /// If Some, we are recording into the named register.
    pub recording: Option<char>,
    /// Keys captured during the current recording.
    pub buffer: Vec<MacroKey>,
}

impl MacroState {
    pub fn new() -> Self { Self::default() }

    /// Start recording into register `c` (must be a-z).
    pub fn start(&mut self, c: char) -> bool {
        if !c.is_ascii_lowercase() { return false; }
        self.recording = Some(c);
        self.buffer.clear();
        true
    }

    /// Stop recording and return the register + captured keys.
    pub fn stop(&mut self) -> Option<(char, Vec<MacroKey>)> {
        let reg = self.recording.take()?;
        let keys = std::mem::take(&mut self.buffer);
        Some((reg, keys))
    }

    /// Record a key if currently recording.
    pub fn capture(&mut self, key: &Key, mods: &KeyModifiers) {
        if self.recording.is_some() {
            self.buffer.push(MacroKey {
                key: key.clone(),
                mods: mods.clone(),
            });
        }
    }

    /// Whether currently recording.
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }
}

/// Serialize macro keys to a string representation for register storage.
pub fn keys_to_string(keys: &[MacroKey]) -> String {
    let mut out = String::new();
    for mk in keys {
        if mk.mods.ctrl {
            if let Key::Char(c) = mk.key {
                out.push('^');
                out.push(c);
                continue;
            }
        }
        match &mk.key {
            Key::Char(c) => out.push(*c),
            Key::Enter => out.push('\n'),
            Key::Escape => { out.push_str("<Esc>"); }
            Key::Backspace => { out.push_str("<BS>"); }
            Key::Tab => out.push('\t'),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk(c: char) -> (Key, KeyModifiers) {
        (Key::Char(c), KeyModifiers::default())
    }

    #[test]
    fn start_lowercase_only() {
        let mut m = MacroState::new();
        assert!(m.start('a'));
        assert!(!m.start('A')); // uppercase rejected
    }

    #[test]
    fn capture_and_stop() {
        let mut m = MacroState::new();
        m.start('a');
        let (k, mo) = mk('x');
        m.capture(&k, &mo);
        let (k2, mo2) = mk('y');
        m.capture(&k2, &mo2);
        let (reg, keys) = m.stop().unwrap();
        assert_eq!(reg, 'a');
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn stop_without_start_is_none() {
        let mut m = MacroState::new();
        assert!(m.stop().is_none());
    }

    #[test]
    fn capture_when_not_recording_is_noop() {
        let mut m = MacroState::new();
        let (k, mo) = mk('z');
        m.capture(&k, &mo);
        assert!(m.buffer.is_empty());
    }

    #[test]
    fn keys_to_string_basic() {
        let keys = vec![
            MacroKey { key: Key::Char('i'), mods: KeyModifiers::default() },
            MacroKey { key: Key::Char('h'), mods: KeyModifiers::default() },
            MacroKey { key: Key::Escape, mods: KeyModifiers::default() },
        ];
        assert_eq!(keys_to_string(&keys), "ih<Esc>");
    }
}
