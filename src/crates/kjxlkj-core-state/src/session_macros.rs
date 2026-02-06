//! Session persistence for macros and key repeats.

use kjxlkj_core_types::Intent;
use std::collections::HashMap;

/// A key stroke for macro serialization.
#[derive(Debug, Clone)]
pub struct KeyStroke {
    pub code: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

/// Key modifiers for serialized macro playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers { pub ctrl: bool, pub alt: bool, pub shift: bool }

/// A recorded macro containing a sequence of intents.
#[derive(Debug, Clone)]
pub struct Macro {
    pub register: char,
    pub intents: Vec<Intent>,
}

/// A recorded macro stored as serializable key strokes.
#[derive(Debug, Clone)]
pub struct MacroRecord {
    pub register: char,
    pub key_strokes: Vec<KeyStroke>,
}

/// Persistent macro storage for session save/restore.
#[derive(Debug, Clone, Default)]
pub struct MacroStore {
    pub macros: HashMap<char, Vec<KeyStroke>>,
}

impl MacroStore {
    pub fn new() -> Self { Self { macros: HashMap::new() } }

    /// Store a macro's key strokes.
    pub fn store(&mut self, reg: char, strokes: Vec<KeyStroke>) {
        self.macros.insert(reg, strokes);
    }

    /// Retrieve macro key strokes.
    pub fn get(&self, reg: char) -> Option<&[KeyStroke]> {
        self.macros.get(&reg).map(|v| v.as_slice())
    }

    /// Remove a macro.
    pub fn remove(&mut self, reg: char) -> Option<Vec<KeyStroke>> {
        self.macros.remove(&reg)
    }

    /// List all registered macro names.
    pub fn registers(&self) -> Vec<char> {
        let mut regs: Vec<char> = self.macros.keys().copied().collect();
        regs.sort();
        regs
    }

    pub fn len(&self) -> usize { self.macros.len() }
    pub fn is_empty(&self) -> bool { self.macros.is_empty() }

    /// Clear all macros.
    pub fn clear(&mut self) { self.macros.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_store_basic_ops() {
        let mut store = MacroStore::new();
        assert!(store.is_empty());
        store.store('a', vec![
            KeyStroke { code: "j".into(), ctrl: false, alt: false, shift: false },
            KeyStroke { code: "d".into(), ctrl: false, alt: false, shift: false },
            KeyStroke { code: "d".into(), ctrl: false, alt: false, shift: false },
        ]);
        assert_eq!(store.len(), 1);
        assert_eq!(store.get('a').unwrap().len(), 3);
        assert!(store.get('b').is_none());
    }

    #[test]
    fn macro_store_overwrite() {
        let mut store = MacroStore::new();
        store.store('a', vec![KeyStroke { code: "x".into(), ctrl: false, alt: false, shift: false }]);
        store.store('a', vec![
            KeyStroke { code: "y".into(), ctrl: false, alt: false, shift: false },
            KeyStroke { code: "y".into(), ctrl: false, alt: false, shift: false },
        ]);
        assert_eq!(store.get('a').unwrap().len(), 2);
        assert_eq!(store.get('a').unwrap()[0].code, "y");
    }

    #[test]
    fn macro_store_list_and_remove() {
        let mut store = MacroStore::new();
        store.store('b', vec![]);
        store.store('a', vec![]);
        let regs = store.registers();
        assert_eq!(regs, vec!['a', 'b']);
        store.remove('a');
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn macro_store_clear() {
        let mut store = MacroStore::new();
        store.store('x', vec![]);
        store.store('y', vec![]);
        store.clear();
        assert!(store.is_empty());
    }

    #[test]
    fn key_modifiers_default() {
        let km = KeyModifiers::default();
        assert!(!km.ctrl && !km.alt && !km.shift);
    }

    #[test]
    fn macro_record_type() {
        let mr = MacroRecord {
            register: 'q',
            key_strokes: vec![KeyStroke { code: "i".into(), ctrl: false, alt: false, shift: false }],
        };
        assert_eq!(mr.register, 'q');
    }
}
