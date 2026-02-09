//! Spell checking with word lists and dictionary file loading.
use crate::editor::EditorState;

/// Spell checker with built-in word list and dictionary file support.
#[derive(Debug, Default)]
pub struct SpellChecker {
    pub enabled: bool,
    pub good_words: Vec<String>,
    pub bad_words: Vec<String>,
    pub lang: String,
}

impl SpellChecker {
    pub fn new() -> Self {
        Self { enabled: false, good_words: Vec::new(), bad_words: Vec::new(), lang: "en".into() }
    }

    /// Load a dictionary file (one word per line; '#' = comment; word/affix format supported).
    #[rustfmt::skip]
    pub fn load_dictionary(&mut self, path: &str) -> Result<usize, String> {
        let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot read {path}: {e}"))?;
        let mut count = 0;
        for line in content.lines() {
            let word = line.trim();
            if word.is_empty() || word.starts_with('#') { continue; }
            let word = word.split('/').next().unwrap_or(word).trim();
            if !word.is_empty() && !self.is_good(word) { self.good_words.push(word.to_string()); count += 1; }
        }
        Ok(count)
    }

    pub fn is_good(&self, word: &str) -> bool { let l = word.to_lowercase(); self.good_words.iter().any(|w| w.to_lowercase() == l) }
    pub fn is_bad(&self, word: &str) -> bool { let l = word.to_lowercase(); self.bad_words.iter().any(|w| w.to_lowercase() == l) }
    pub fn add_good(&mut self, word: String) { if !self.is_good(&word) { self.good_words.push(word); } }
    pub fn add_bad(&mut self, word: String) { if !self.is_bad(&word) { self.bad_words.push(word); } }
    pub fn undo_good(&mut self, word: &str) { let l = word.to_lowercase(); self.good_words.retain(|w| w.to_lowercase() != l); }

    /// Generate spelling suggestions using edit-distance heuristic. Returns up to `max` sorted by relevance.
    #[rustfmt::skip]
    pub fn suggest(&self, word: &str, max: usize) -> Vec<String> {
        let lower = word.to_lowercase();
        let mut cands: Vec<(usize, String)> = Vec::new();
        for gw in &self.good_words { let d = edit_distance(&lower, &gw.to_lowercase()); if d <= 3 && d > 0 { cands.push((d, gw.clone())); } }
        let (alpha, bytes) = ("abcdefghijklmnopqrstuvwxyz", lower.as_bytes());
        for i in 0..bytes.len() { let mut w = lower.clone(); w.remove(i); if !w.is_empty() && !cands.iter().any(|(_, c)| c == &w) { cands.push((1, w)); } }
        for i in 0..bytes.len().saturating_sub(1) { let mut w: Vec<u8> = bytes.to_vec(); w.swap(i, i+1); let s = String::from_utf8_lossy(&w).to_string(); if !cands.iter().any(|(_, c)| c == &s) { cands.push((1, s)); } }
        for i in 0..bytes.len() { for c in alpha.bytes() { if c != bytes[i] { let mut w: Vec<u8> = bytes.to_vec(); w[i] = c; let s = String::from_utf8_lossy(&w).to_string(); if !cands.iter().any(|(_, c2)| c2 == &s) { cands.push((1, s)); } } } }
        cands.sort_by_key(|&(d, _)| d);
        cands.into_iter().take(max).map(|(_, w)| w).collect()
    }
}

impl EditorState {
    /// Toggle spell checking on/off via :set spell / :set nospell.
    pub fn toggle_spell(&mut self, enable: bool) {
        self.spell.enabled = enable;
        if enable { self.try_load_spell_dictionary(); }
        let msg = if enable { "Spell checking enabled" } else { "Spell checking disabled" };
        self.notify_info(msg);
    }

    /// Try to load default spell dictionary from XDG or cwd paths.
    /// Supports comma-separated languages in `spelllang` option (e.g., "en,fr").
    #[rustfmt::skip]
    fn try_load_spell_dictionary(&mut self) {
        let lang_str = self.options.get_str("spelllang").to_string();
        let default_lang = self.spell.lang.clone();
        let langs: Vec<String> = if lang_str.is_empty() { vec![default_lang] } else { lang_str.split(',').map(|s| s.trim().to_string()).collect() };
        for lang in &langs { self.load_spell_for_lang(lang); }
    }

    #[rustfmt::skip]
    fn load_spell_for_lang(&mut self, lang: &str) {
        let cands = [format!("spell/{lang}.dic"), format!("spell/{lang}.utf-8.spl")];
        if let Some(base) = std::env::var("XDG_CONFIG_HOME").ok().or_else(|| std::env::var("HOME").ok().map(|h| format!("{h}/.config"))) {
            for c in &cands { let p = format!("{base}/kjxlkj/{c}");
                if std::path::Path::new(&p).exists() { match self.spell.load_dictionary(&p) { Ok(n) => { self.notify_info(&format!("Loaded {n} words from {p}")); return; } Err(e) => { self.notify_error(&e); return; } } }
            }
        }
        for c in &cands { if std::path::Path::new(c).exists() { match self.spell.load_dictionary(c) { Ok(n) => { self.notify_info(&format!("Loaded {n} words from {c}")); return; } Err(e) => { self.notify_error(&e); return; } } } }
    }

    #[allow(dead_code)]
    pub(crate) fn spell_add_good(&mut self) { let w = self.word_under_cursor_pub(); if w.is_empty() { return self.notify_error("E756: No word under cursor"); } self.spell.add_good(w.clone()); self.notify_info(&format!("Word added to spellfile: {w}")); }
    #[allow(dead_code)]
    pub(crate) fn spell_add_bad(&mut self) { let w = self.word_under_cursor_pub(); if w.is_empty() { return self.notify_error("E756: No word under cursor"); } self.spell.add_bad(w.clone()); self.notify_info(&format!("Word marked as wrong: {w}")); }

    /// Handle `z=`: show spelling suggestions for word under cursor.
    #[allow(dead_code)]
    #[rustfmt::skip]
    pub(crate) fn spell_suggest(&mut self) {
        let word = self.word_under_cursor_pub();
        if word.is_empty() { return self.notify_error("E756: No word under cursor"); }
        let sug = self.spell.suggest(&word, 10);
        if sug.is_empty() { self.notify_info(&format!("No suggestions for \"{word}\"")); }
        else { let mut l = vec![format!("Suggestions for \"{word}\":")]; for (i, s) in sug.iter().enumerate() { l.push(format!(" {:>2}: {s}", i+1)); } self.notify_info(&l.join("\n")); }
    }

    /// Public wrapper for word_under_cursor.
    #[allow(dead_code)]
    #[rustfmt::skip]
    pub(crate) fn word_under_cursor_pub(&self) -> String {
        let (buf_id, cursor) = (self.current_buffer_id(), self.windows.focused().cursor);
        if let Some(buf) = self.buffers.get(buf_id) {
            if cursor.line < buf.content.len_lines() {
                let line: String = buf.content.line(cursor.line).chars().collect();
                let (bytes, col) = (line.as_bytes(), cursor.grapheme.min(line.len().saturating_sub(1)));
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
