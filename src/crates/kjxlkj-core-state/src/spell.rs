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

    /// Generate spelling suggestions for a word using edit-distance heuristic.
    /// Returns up to `max` suggestions sorted by relevance.
    #[rustfmt::skip]
    pub fn suggest(&self, word: &str, max: usize) -> Vec<String> {
        let lower = word.to_lowercase();
        let mut candidates: Vec<(usize, String)> = Vec::new();
        // Score good words by edit distance.
        for gw in &self.good_words {
            let dist = edit_distance(&lower, &gw.to_lowercase());
            if dist <= 3 && dist > 0 { candidates.push((dist, gw.clone())); }
        }
        // Common transformations: swap adjacent, delete char, insert common chars, replace.
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        let bytes = lower.as_bytes();
        // Single-char deletions.
        for i in 0..bytes.len() { let mut w = lower.clone(); w.remove(i); if !w.is_empty() && !candidates.iter().any(|(_, c)| c == &w) { candidates.push((1, w)); } }
        // Adjacent swaps.
        for i in 0..bytes.len().saturating_sub(1) {
            let mut w: Vec<u8> = bytes.to_vec(); w.swap(i, i + 1);
            let s = String::from_utf8_lossy(&w).to_string();
            if !candidates.iter().any(|(_, c)| c == &s) { candidates.push((1, s)); }
        }
        // Single-char replacements.
        for i in 0..bytes.len() {
            for c in alpha.bytes() {
                if c != bytes[i] { let mut w: Vec<u8> = bytes.to_vec(); w[i] = c; let s = String::from_utf8_lossy(&w).to_string(); if !candidates.iter().any(|(_, c2)| c2 == &s) { candidates.push((1, s)); } }
            }
        }
        candidates.sort_by_key(|&(d, _)| d);
        candidates.into_iter().take(max).map(|(_, w)| w).collect()
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

    /// Handle `z=` command: show spelling suggestions for word under cursor.
    #[allow(dead_code)]
    pub(crate) fn spell_suggest(&mut self) {
        let word = self.word_under_cursor_pub();
        if word.is_empty() { return self.notify_error("E756: No word under cursor"); }
        let suggestions = self.spell.suggest(&word, 10);
        if suggestions.is_empty() {
            self.notify_info(&format!("No suggestions for \"{word}\""));
        } else {
            let mut lines = vec![format!("Suggestions for \"{word}\":")];
            for (i, s) in suggestions.iter().enumerate() { lines.push(format!(" {:>2}: {s}", i + 1)); }
            self.notify_info(&lines.join("\n"));
        }
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

/// Simple Levenshtein edit distance between two strings.
#[rustfmt::skip]
fn edit_distance(a: &str, b: &str) -> usize {
    let (ab, bb) = (a.as_bytes(), b.as_bytes());
    let (m, n) = (ab.len(), bb.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if ab[i - 1] == bb[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
