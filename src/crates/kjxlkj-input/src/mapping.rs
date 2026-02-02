//! Key mapping.

use crate::{Key, KeySequence};
use kjxlkj_core_mode::Intent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Key mapping entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapEntry {
    /// Target intent.
    pub intent: Intent,
    /// Description.
    pub description: String,
}

/// Key map for a mode.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyMap {
    /// Single key mappings.
    single: HashMap<Key, KeyMapEntry>,
    /// Sequence mappings.
    sequences: Vec<(KeySequence, KeyMapEntry)>,
}

impl KeyMap {
    /// Creates a new key map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a single key mapping.
    pub fn bind(&mut self, key: Key, intent: Intent, description: impl Into<String>) {
        self.single.insert(
            key,
            KeyMapEntry {
                intent,
                description: description.into(),
            },
        );
    }

    /// Adds a sequence mapping.
    pub fn bind_seq(&mut self, seq: KeySequence, intent: Intent, description: impl Into<String>) {
        self.sequences.push((
            seq,
            KeyMapEntry {
                intent,
                description: description.into(),
            },
        ));
    }

    /// Looks up a single key.
    pub fn lookup(&self, key: &Key) -> Option<&Intent> {
        self.single.get(key).map(|e| &e.intent)
    }

    /// Looks up a sequence.
    pub fn lookup_seq(&self, seq: &KeySequence) -> Option<&Intent> {
        self.sequences
            .iter()
            .find(|(s, _)| s.keys == seq.keys)
            .map(|(_, e)| &e.intent)
    }

    /// Checks if a sequence is a prefix of any mapping.
    pub fn is_prefix(&self, seq: &KeySequence) -> bool {
        self.sequences.iter().any(|(s, _)| {
            s.keys.len() > seq.keys.len()
                && s.keys.iter().zip(seq.keys.iter()).all(|(a, b)| a == b)
        })
    }
}
