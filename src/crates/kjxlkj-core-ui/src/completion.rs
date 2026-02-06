//! Insert-mode completion menu state and types.

/// Source of completion candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionSource {
    Buffer,     // Ctrl-N / Ctrl-P: words from current buffer
    Path,       // Ctrl-X Ctrl-F: file path completion
    Line,       // Ctrl-X Ctrl-L: whole line completion
    Lsp,        // Ctrl-X Ctrl-O: omni/LSP completion
    Dictionary, // Ctrl-X Ctrl-K: dictionary words
    Command,    // Ctrl-X Ctrl-V: Vim command names
}

/// A single completion candidate.
#[derive(Debug, Clone)]
pub struct CompletionCandidate {
    pub word: String,
    pub kind: Option<CompletionKind>,
    pub menu: Option<String>,
    pub info: Option<String>,
}

/// Kind tag for completion items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Variable, Function, Method, Class, Module, Keyword,
    Snippet, File, Folder, Text, Constant, Field, Property,
}

/// The popup completion menu state.
#[derive(Debug, Clone)]
pub struct CompletionMenu {
    pub visible: bool,
    pub candidates: Vec<CompletionCandidate>,
    pub selected: usize,
    pub prefix: String,
    pub source: CompletionSource,
    pub start_col: usize,
}

impl CompletionMenu {
    pub fn new() -> Self {
        Self { visible: false, candidates: Vec::new(), selected: 0,
               prefix: String::new(), source: CompletionSource::Buffer, start_col: 0 }
    }

    /// Open the menu with candidates.
    pub fn open(&mut self, candidates: Vec<CompletionCandidate>, prefix: &str, col: usize, source: CompletionSource) {
        self.candidates = candidates;
        self.prefix = prefix.to_string();
        self.start_col = col;
        self.selected = 0;
        self.source = source;
        self.visible = !self.candidates.is_empty();
    }

    /// Close the menu.
    pub fn close(&mut self) { self.visible = false; self.candidates.clear(); self.selected = 0; }

    /// Select next candidate.
    pub fn select_next(&mut self) {
        if !self.candidates.is_empty() { self.selected = (self.selected + 1) % self.candidates.len(); }
    }

    /// Select previous candidate.
    pub fn select_prev(&mut self) {
        if !self.candidates.is_empty() {
            self.selected = if self.selected == 0 { self.candidates.len() - 1 } else { self.selected - 1 };
        }
    }

    /// Get the currently selected candidate.
    pub fn current(&self) -> Option<&CompletionCandidate> { self.candidates.get(self.selected) }

    /// Filter candidates by prefix.
    pub fn filter(&mut self, prefix: &str) {
        let lower = prefix.to_lowercase();
        self.candidates.retain(|c| c.word.to_lowercase().contains(&lower));
        self.prefix = prefix.to_string();
        if self.selected >= self.candidates.len() { self.selected = 0; }
        self.visible = !self.candidates.is_empty();
    }

    /// Number of visible candidates.
    pub fn len(&self) -> usize { self.candidates.len() }

    /// Whether the menu has no candidates.
    pub fn is_empty(&self) -> bool { self.candidates.is_empty() }
}

impl Default for CompletionMenu { fn default() -> Self { Self::new() } }

/// Collect word completions from buffer text (Ctrl-N source).
pub fn collect_buffer_words(text: &str, prefix: &str) -> Vec<CompletionCandidate> {
    let lower = prefix.to_lowercase();
    let mut seen = std::collections::HashSet::new();
    let mut results = Vec::new();
    for word in text.split(|c: char| !c.is_alphanumeric() && c != '_') {
        if word.len() <= 1 || word == prefix { continue; }
        let wl = word.to_lowercase();
        if wl.starts_with(&lower) && seen.insert(wl) {
            results.push(CompletionCandidate { word: word.to_string(), kind: Some(CompletionKind::Text), menu: None, info: None });
        }
    }
    results
}

/// Collect line completions (Ctrl-X Ctrl-L source).
pub fn collect_line_completions(text: &str, prefix: &str) -> Vec<CompletionCandidate> {
    let lower = prefix.to_lowercase();
    let mut seen = std::collections::HashSet::new();
    text.lines()
        .filter(|l| { let t = l.trim(); !t.is_empty() && t.to_lowercase().starts_with(&lower) && seen.insert(t.to_string()) })
        .map(|l| CompletionCandidate { word: l.trim().to_string(), kind: Some(CompletionKind::Text), menu: Some("line".into()), info: None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_open_close() {
        let mut menu = CompletionMenu::new();
        assert!(!menu.visible);
        let items = vec![CompletionCandidate { word: "hello".into(), kind: None, menu: None, info: None }];
        menu.open(items, "he", 5, CompletionSource::Buffer);
        assert!(menu.visible);
        assert_eq!(menu.len(), 1);
        menu.close();
        assert!(!menu.visible);
        assert!(menu.is_empty());
    }

    #[test]
    fn menu_navigation() {
        let mut menu = CompletionMenu::new();
        let items = vec![
            CompletionCandidate { word: "abc".into(), kind: None, menu: None, info: None },
            CompletionCandidate { word: "abd".into(), kind: None, menu: None, info: None },
            CompletionCandidate { word: "abe".into(), kind: None, menu: None, info: None },
        ];
        menu.open(items, "ab", 0, CompletionSource::Buffer);
        assert_eq!(menu.current().unwrap().word, "abc");
        menu.select_next();
        assert_eq!(menu.current().unwrap().word, "abd");
        menu.select_next();
        menu.select_next(); // wraps
        assert_eq!(menu.current().unwrap().word, "abc");
        menu.select_prev(); // wraps back
        assert_eq!(menu.current().unwrap().word, "abe");
    }

    #[test]
    fn buffer_words() {
        let text = "hello world hello_world help";
        let results = collect_buffer_words(text, "hel");
        assert!(results.iter().any(|c| c.word == "hello"));
        assert!(results.iter().any(|c| c.word == "hello_world"));
        assert!(results.iter().any(|c| c.word == "help"));
        assert!(!results.iter().any(|c| c.word == "world"));
    }

    #[test]
    fn line_completions() {
        let text = "let x = 1;\nlet y = 2;\nfn main() {}";
        let results = collect_line_completions(text, "let");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn filter_candidates() {
        let mut menu = CompletionMenu::new();
        let items = vec![
            CompletionCandidate { word: "apple".into(), kind: None, menu: None, info: None },
            CompletionCandidate { word: "banana".into(), kind: None, menu: None, info: None },
        ];
        menu.open(items, "a", 0, CompletionSource::Buffer);
        menu.filter("ban");
        assert_eq!(menu.len(), 1);
        assert_eq!(menu.current().unwrap().word, "banana");
    }
}
