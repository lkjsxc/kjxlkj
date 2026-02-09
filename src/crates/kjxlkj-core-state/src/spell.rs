//! Spell checking integration stub.
//!
//! Provides a basic spell checking service that maintains a word list
//! and checks words against it. Full integration with external spell
//! engines (aspell/hunspell) is planned for future waves.

use crate::editor::EditorState;

/// Simple spell checker with a built-in word list.
#[derive(Debug, Default)]
pub struct SpellChecker {
    /// Whether spell checking is enabled.
    pub enabled: bool,
    /// Custom good-word list (user additions via `zg`).
    pub good_words: Vec<String>,
    /// Custom bad-word list (user additions via `zw`).
    pub bad_words: Vec<String>,
    /// Language for spell checking (default: "en").
    pub lang: String,
}

impl SpellChecker {
    pub fn new() -> Self {
        Self { enabled: false, good_words: Vec::new(), bad_words: Vec::new(), lang: "en".into() }
    }

    /// Check if a word is in the good-word list.
    pub fn is_good(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.good_words.iter().any(|w| w.to_lowercase() == lower)
    }

    /// Check if a word is in the bad-word list.
    pub fn is_bad(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.bad_words.iter().any(|w| w.to_lowercase() == lower)
    }

    /// Add a word to the good-word list (zg command).
    pub fn add_good(&mut self, word: String) {
        if !self.is_good(&word) { self.good_words.push(word); }
    }

    /// Add a word to the bad-word list (zw command).
    pub fn add_bad(&mut self, word: String) {
        if !self.is_bad(&word) { self.bad_words.push(word); }
    }

    /// Remove a word from the good-word list (zug command).
    pub fn undo_good(&mut self, word: &str) {
        let lower = word.to_lowercase();
        self.good_words.retain(|w| w.to_lowercase() != lower);
    }
}

impl EditorState {
    /// Toggle spell checking on/off via :set spell / :set nospell.
    pub fn toggle_spell(&mut self, enable: bool) {
        self.spell.enabled = enable;
        let msg = if enable { "Spell checking enabled" } else { "Spell checking disabled" };
        self.notify_info(msg);
    }

    /// Handle `zg` command: add word under cursor to good-word list.
    #[allow(dead_code)]
    pub(crate) fn spell_add_good(&mut self) {
        let word = self.word_under_cursor_pub();
        if word.is_empty() { return self.notify_error("E756: No word under cursor"); }
        self.spell.add_good(word.clone());
        self.notify_info(&format!("Word added to spellfile: {word}"));
    }

    /// Handle `zw` command: mark word under cursor as bad.
    #[allow(dead_code)]
    pub(crate) fn spell_add_bad(&mut self) {
        let word = self.word_under_cursor_pub();
        if word.is_empty() { return self.notify_error("E756: No word under cursor"); }
        self.spell.add_bad(word.clone());
        self.notify_info(&format!("Word marked as wrong: {word}"));
    }

    /// Public wrapper for word_under_cursor.
    #[allow(dead_code)]
    pub(crate) fn word_under_cursor_pub(&self) -> String {
        let (buf_id, cursor) = (self.current_buffer_id(), self.windows.focused().cursor);
        if let Some(buf) = self.buffers.get(buf_id) {
            if cursor.line < buf.content.len_lines() {
                let line: String = buf.content.line(cursor.line).chars().collect();
                let bytes = line.as_bytes();
                let col = cursor.grapheme.min(bytes.len().saturating_sub(1));
                if col < bytes.len() && ((bytes[col] as char).is_alphanumeric() || bytes[col] == b'_') {
                    let start = (0..=col).rev().take_while(|&i| { let c = bytes[i] as char; c.is_alphanumeric() || c == '_' }).last().unwrap_or(col);
                    let end = (col..bytes.len()).take_while(|&i| { let c = bytes[i] as char; c.is_alphanumeric() || c == '_' }).last().unwrap_or(col);
                    return line[start..=end].to_string();
                }
            }
        }
        String::new()
    }
}
