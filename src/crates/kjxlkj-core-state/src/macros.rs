//! Macro recording and playback.

use kjxlkj_input::Key;
use std::collections::HashMap;

pub use crate::macro_types::{Macro, RecordingState};

/// Macro storage and recording.
#[derive(Debug, Clone, Default)]
pub struct MacroStore {
    /// Stored macros by register.
    macros: HashMap<char, Macro>,
    /// Current recording state.
    recording: RecordingState,
    /// Current recording buffer.
    buffer: Macro,
}

impl MacroStore {
    /// Creates a new macro store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns if currently recording.
    pub fn is_recording(&self) -> bool {
        matches!(self.recording, RecordingState::Recording(_))
    }

    /// Returns the current recording register.
    pub fn recording_register(&self) -> Option<char> {
        match self.recording {
            RecordingState::Recording(r) => Some(r),
            RecordingState::Idle => None,
        }
    }

    /// Starts recording to a register.
    pub fn start_recording(&mut self, register: char) {
        self.recording = RecordingState::Recording(register);
        self.buffer.clear();
    }

    /// Stops recording and stores the macro.
    pub fn stop_recording(&mut self) {
        if let RecordingState::Recording(register) = self.recording {
            if register.is_ascii_uppercase() {
                let lower = register.to_ascii_lowercase();
                if let Some(existing) = self.macros.get_mut(&lower) {
                    existing.append(&self.buffer);
                } else {
                    self.macros.insert(lower, std::mem::take(&mut self.buffer));
                }
            } else {
                self.macros.insert(register, std::mem::take(&mut self.buffer));
            }
        }
        self.recording = RecordingState::Idle;
    }

    /// Records a key if recording is active.
    pub fn record_key(&mut self, key: Key) {
        if self.is_recording() {
            self.buffer.push(key);
        }
    }

    /// Returns a macro by register.
    pub fn get(&self, register: char) -> Option<&Macro> {
        let r = register.to_ascii_lowercase();
        self.macros.get(&r)
    }

    /// Sets a macro.
    pub fn set(&mut self, register: char, m: Macro) {
        self.macros.insert(register.to_ascii_lowercase(), m);
    }

    /// Returns all registered macros.
    pub fn list(&self) -> Vec<(char, &Macro)> {
        let mut result: Vec<_> = self.macros.iter().map(|(k, v)| (*k, v)).collect();
        result.sort_by_key(|(k, _)| *k);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_input::KeyCodeWrapper;

    fn key(c: char) -> Key {
        Key {
            code: KeyCodeWrapper::Char(c),
            modifiers: Default::default(),
        }
    }

    #[test]
    fn test_macro_new() {
        let m = Macro::new();
        assert!(m.is_empty());
    }

    #[test]
    fn test_macro_push() {
        let mut m = Macro::new();
        m.push(key('a'));
        m.push(key('b'));
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_macro_store_new() {
        let store = MacroStore::new();
        assert!(!store.is_recording());
    }

    #[test]
    fn test_macro_recording() {
        let mut store = MacroStore::new();
        store.start_recording('a');
        assert!(store.is_recording());
        assert_eq!(store.recording_register(), Some('a'));
        
        store.record_key(key('h'));
        store.record_key(key('j'));
        store.stop_recording();
        
        assert!(!store.is_recording());
        let m = store.get('a').unwrap();
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_macro_uppercase_append() {
        let mut store = MacroStore::new();
        
        store.start_recording('a');
        store.record_key(key('x'));
        store.stop_recording();
        
        store.start_recording('A');
        store.record_key(key('y'));
        store.stop_recording();
        
        let m = store.get('a').unwrap();
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_macro_get_set() {
        let mut store = MacroStore::new();
        let m = Macro::from_keys(vec![key('a'), key('b')]);
        store.set('q', m);
        assert_eq!(store.get('q').unwrap().len(), 2);
    }
}
