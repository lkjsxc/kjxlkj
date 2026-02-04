//! Register storage.

use kjxlkj_core_types::{Register, RegisterName};
use std::collections::HashMap;

/// Storage for registers.
#[derive(Debug, Default)]
pub struct RegisterStore {
    /// Named and numbered registers.
    registers: HashMap<RegisterName, Register>,
    /// Recording macros.
    macro_recording: Option<(char, Vec<kjxlkj_core_types::Key>)>,
    /// Recorded macros.
    macros: HashMap<char, Vec<kjxlkj_core_types::Key>>,
    /// Last played macro register.
    last_macro: Option<char>,
}

impl RegisterStore {
    /// Create a new register store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a register.
    pub fn set(&mut self, name: RegisterName, content: Register) {
        if matches!(name, RegisterName::BlackHole) {
            return; // Black hole discards
        }
        // Also update unnamed register for most operations.
        if !matches!(name, RegisterName::Unnamed | RegisterName::BlackHole) {
            self.registers
                .insert(RegisterName::Unnamed, content.clone());
        }
        self.registers.insert(name, content);
    }

    /// Get a register.
    pub fn get(&self, name: &RegisterName) -> Option<&Register> {
        self.registers.get(name)
    }

    /// Get the unnamed register.
    pub fn unnamed(&self) -> Option<&Register> {
        self.registers.get(&RegisterName::Unnamed)
    }

    /// Yank text to a register.
    pub fn yank(&mut self, name: Option<RegisterName>, text: String, linewise: bool) {
        let name = name.unwrap_or(RegisterName::Unnamed);
        let reg = if linewise {
            Register::linewise(text)
        } else {
            Register::charwise(text)
        };
        if !matches!(name, RegisterName::BlackHole) {
            self.registers
                .insert(RegisterName::Numbered(0), reg.clone());
        }
        self.set(name, reg);
    }

    /// Start recording a macro.
    pub fn start_macro(&mut self, name: char) {
        self.macro_recording = Some((name, Vec::new()));
    }

    /// Stop recording and save the macro.
    pub fn stop_macro(&mut self) -> Option<char> {
        if let Some((name, keys)) = self.macro_recording.take() {
            self.macros.insert(name, keys);
            self.last_macro = Some(name);
            return Some(name);
        }
        None
    }

    /// Check if recording.
    pub fn is_recording(&self) -> bool {
        self.macro_recording.is_some()
    }

    /// Record a key to the current macro.
    pub fn record_key(&mut self, key: kjxlkj_core_types::Key) {
        if let Some((_, ref mut keys)) = self.macro_recording {
            keys.push(key);
        }
    }

    /// Get a recorded macro.
    pub fn get_macro(&self, name: char) -> Option<&[kjxlkj_core_types::Key]> {
        self.macros.get(&name).map(|v| v.as_slice())
    }

    /// Get the last played macro register.
    pub fn last_macro(&self) -> Option<char> {
        self.last_macro
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yank() {
        let mut store = RegisterStore::new();
        store.yank(None, "hello".to_string(), false);
        let reg = store.unnamed().unwrap();
        assert_eq!(reg.content, "hello");
        assert!(!reg.linewise);
        let reg0 = store.get(&RegisterName::Numbered(0)).unwrap();
        assert_eq!(reg0.content, "hello");
    }

    #[test]
    fn test_macro_recording() {
        let mut store = RegisterStore::new();
        store.start_macro('a');
        assert!(store.is_recording());
        store.stop_macro();
        assert!(!store.is_recording());
    }
}
