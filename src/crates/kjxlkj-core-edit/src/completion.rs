//! Buffer-local completion menu and word collection.

use kjxlkj_core_text::TextBuffer;
use serde::{Deserialize, Serialize};

/// Source of completion candidates.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompletionSource {
    Buffer,
    Path,
    Line,
    Lsp,
    Dictionary,
    Command,
}

/// A single completion candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub kind: Option<String>,
    pub source: CompletionSource,
}

/// An interactive completion menu.
#[derive(Debug, Clone)]
pub struct CompletionMenu {
    pub items: Vec<CompletionItem>,
    pub selected_index: usize,
    active: bool,
}

impl CompletionMenu {
    /// Open the menu with the given items.
    pub fn open(items: Vec<CompletionItem>) -> Self {
        Self {
            items,
            selected_index: 0,
            active: true,
        }
    }

    /// Close the menu.
    pub fn close(&mut self) {
        self.active = false;
        self.items.clear();
        self.selected_index = 0;
    }

    /// Select the next item, cycling to the start.
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    /// Select the previous item, cycling to the end.
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Filter items by prefix, resetting selection.
    pub fn filter(&mut self, prefix: &str) {
        self.items.retain(|item| item.label.starts_with(prefix));
        self.selected_index = 0;
    }

    /// Return the currently selected item.
    pub fn current(&self) -> Option<&CompletionItem> {
        if self.active && !self.items.is_empty() {
            Some(&self.items[self.selected_index])
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Collect unique words from the buffer that start with `prefix`.
pub fn collect_buffer_words(buffer: &TextBuffer, prefix: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for line_idx in 0..buffer.line_count() {
        let line = buffer.line(line_idx).unwrap_or_default();
        for word in split_words(&line) {
            if word.starts_with(prefix) && word != prefix && seen.insert(word.to_string()) {
                words.push(word.to_string());
            }
        }
    }
    words.sort();
    words
}

/// Collect whole lines that start with `prefix`.
pub fn collect_line_completions(buffer: &TextBuffer, prefix: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line_idx in 0..buffer.line_count() {
        let line = buffer.line(line_idx).unwrap_or_default();
        let trimmed = line.trim_start();
        if trimmed.starts_with(prefix) && trimmed != prefix {
            result.push(trimmed.to_string());
        }
    }
    result.sort();
    result.dedup();
    result
}

fn split_words(line: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if is_word_byte(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_word_byte(bytes[i]) {
                i += 1;
            }
            words.push(&line[start..i]);
        } else {
            i += 1;
        }
    }
    words
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn collect_words() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "hello world\nhello there\n");
        let words = collect_buffer_words(&buf, "hel");
        assert_eq!(words, vec!["hello"]);
    }

    #[test]
    fn menu_cycle() {
        let items = vec![
            CompletionItem { label: "a".into(), detail: None, kind: None, source: CompletionSource::Buffer },
            CompletionItem { label: "b".into(), detail: None, kind: None, source: CompletionSource::Buffer },
        ];
        let mut menu = CompletionMenu::open(items);
        assert_eq!(menu.current().unwrap().label, "a");
        menu.select_next();
        assert_eq!(menu.current().unwrap().label, "b");
        menu.select_next();
        assert_eq!(menu.current().unwrap().label, "a");
    }

    #[test]
    fn line_completions() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "fn main() {}\nfn helper() {}\nlet x = 1;\n");
        let lines = collect_line_completions(&buf, "fn");
        assert_eq!(lines.len(), 2);
    }
}
