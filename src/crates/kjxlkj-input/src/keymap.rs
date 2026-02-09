//! Key mapping trie and keybinding tables.

use std::collections::HashMap;

use kjxlkj_core_types::{Action, Key};

/// Entry in the keymap trie.
#[derive(Debug, Clone)]
pub enum KeymapEntry {
    /// A leaf action.
    Action(Action),
    /// A prefix node with children.
    Prefix(KeyTrie),
}

/// Trie node for key sequence lookups.
#[derive(Debug, Clone, Default)]
pub struct KeyTrie {
    children: HashMap<Key, KeymapEntry>,
}

impl KeyTrie {
    /// Create an empty trie.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a single-key binding.
    pub fn bind(&mut self, key: Key, action: Action) {
        self.children.insert(key, KeymapEntry::Action(action));
    }

    /// Insert a multi-key binding.
    pub fn bind_seq(&mut self, keys: &[Key], action: Action) {
        if keys.is_empty() {
            return;
        }
        if keys.len() == 1 {
            self.bind(keys[0].clone(), action);
            return;
        }
        let entry = self
            .children
            .entry(keys[0].clone())
            .or_insert_with(|| {
                KeymapEntry::Prefix(KeyTrie::new())
            });
        if let KeymapEntry::Prefix(ref mut trie) = entry {
            trie.bind_seq(&keys[1..], action);
        }
    }

    /// Look up a key in the trie.
    pub fn get(&self, key: &Key) -> Option<&KeymapEntry> {
        self.children.get(key)
    }

    /// Check if the trie has a prefix for this key.
    pub fn has_prefix(&self, key: &Key) -> bool {
        matches!(
            self.children.get(key),
            Some(KeymapEntry::Prefix(_))
        )
    }

    /// Number of bindings.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Whether the trie is empty.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// Key sequence matcher state.
#[derive(Debug, Default)]
pub struct KeyMatcher {
    /// Accumulated key sequence.
    pending: Vec<Key>,
    /// Timeout for ambiguous prefixes (milliseconds).
    pub timeout_ms: u64,
}

impl KeyMatcher {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            timeout_ms: 1000,
        }
    }

    /// Feed a key. Returns the action if a binding was matched.
    pub fn feed(
        &mut self,
        key: Key,
        trie: &KeyTrie,
    ) -> KeyMatchResult {
        self.pending.push(key);

        // Walk the trie with the pending sequence.
        let mut node = trie;
        for k in &self.pending {
            match node.get(k) {
                Some(KeymapEntry::Action(action)) => {
                    let action = action.clone();
                    self.pending.clear();
                    return KeyMatchResult::Matched(action);
                }
                Some(KeymapEntry::Prefix(sub)) => {
                    node = sub;
                }
                None => {
                    self.pending.clear();
                    return KeyMatchResult::NoMatch;
                }
            }
        }

        KeyMatchResult::Pending
    }

    /// Clear pending state.
    pub fn reset(&mut self) {
        self.pending.clear();
    }
}

/// Result of feeding a key to the matcher.
#[derive(Debug, Clone)]
pub enum KeyMatchResult {
    /// A complete binding was matched.
    Matched(Action),
    /// More keys needed.
    Pending,
    /// No matching binding.
    NoMatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_key_binding() {
        let mut trie = KeyTrie::new();
        trie.bind(Key::char('j'), Action::MoveCursor(
            kjxlkj_core_types::Motion::Down,
            1,
        ));
        assert!(trie.get(&Key::char('j')).is_some());
    }

    #[test]
    fn multi_key_binding() {
        let mut trie = KeyTrie::new();
        trie.bind_seq(
            &[Key::char('g'), Key::char('g')],
            Action::MoveCursor(
                kjxlkj_core_types::Motion::GotoFirstLine,
                1,
            ),
        );
        assert!(trie.has_prefix(&Key::char('g')));
    }

    #[test]
    fn matcher_single() {
        let mut trie = KeyTrie::new();
        trie.bind(Key::char('j'), Action::Nop);
        let mut m = KeyMatcher::new();
        let result = m.feed(Key::char('j'), &trie);
        assert!(matches!(result, KeyMatchResult::Matched(_)));
    }
}
