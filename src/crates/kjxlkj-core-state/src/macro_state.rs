//! Macro recording and playback state.

use kjxlkj_core_types::KeyEvent;
use std::collections::HashMap;

/// Tracks macro recording and stored macros.
#[derive(Debug, Clone)]
pub struct MacroState {
    recording: Option<char>,
    buffer: Vec<KeyEvent>,
    registers: HashMap<char, Vec<KeyEvent>>,
}

impl MacroState {
    pub fn new() -> Self {
        Self {
            recording: None,
            buffer: Vec::new(),
            registers: HashMap::new(),
        }
    }

    /// Start recording a macro into the given register.
    pub fn start_recording(&mut self, register: char) {
        self.recording = Some(register);
        self.buffer.clear();
    }

    /// Stop recording and return the register and recorded keys.
    pub fn stop_recording(&mut self) -> Option<(char, Vec<KeyEvent>)> {
        let reg = self.recording.take()?;
        let keys = std::mem::take(&mut self.buffer);
        self.registers.insert(reg, keys.clone());
        Some((reg, keys))
    }

    /// Record a key event during macro recording.
    pub fn record_key(&mut self, key: KeyEvent) {
        if self.recording.is_some() {
            self.buffer.push(key);
        }
    }

    /// Check if currently recording.
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    /// Get the register being recorded into.
    pub fn recording_register(&self) -> Option<char> {
        self.recording
    }

    /// Get a stored macro by register.
    pub fn get_macro(&self, register: char) -> Option<&Vec<KeyEvent>> {
        self.registers.get(&register)
    }
}

impl Default for MacroState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_get() {
        let mut ms = MacroState::new();
        ms.start_recording('a');
        assert!(ms.is_recording());
        ms.record_key(KeyEvent::char('x'));
        ms.record_key(KeyEvent::char('d'));
        let (reg, keys) = ms.stop_recording().unwrap();
        assert_eq!(reg, 'a');
        assert_eq!(keys.len(), 2);
        assert!(!ms.is_recording());
        let stored = ms.get_macro('a').unwrap();
        assert_eq!(stored.len(), 2);
    }

    #[test]
    fn no_record_when_not_recording() {
        let mut ms = MacroState::new();
        ms.record_key(KeyEvent::char('x'));
        assert!(ms.get_macro('a').is_none());
    }

    #[test]
    fn stop_without_start() {
        let mut ms = MacroState::new();
        assert!(ms.stop_recording().is_none());
    }
}
