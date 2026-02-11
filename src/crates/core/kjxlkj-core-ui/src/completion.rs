//! Insert-mode completion model.
//! See /docs/spec/modes/insert/completion/insert-completion-sources.md.

/// Completion source type with priority ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompletionSource {
    Lsp = 1,
    Snippet = 2,
    Path = 3,
    Buffer = 4,
    Tag = 5,
    Dictionary = 6,
    Line = 7,
    Include = 8,
}

impl CompletionSource {
    /// Priority (lower = higher priority).
    pub fn priority(self) -> u8 { self as u8 }

    /// All sources in priority order.
    pub fn all() -> &'static [CompletionSource] {
        &[Self::Lsp, Self::Snippet, Self::Path, Self::Buffer,
          Self::Tag, Self::Dictionary, Self::Line, Self::Include]
    }
}

/// A single completion candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub insert_text: String,
    pub detail: Option<String>,
    pub source: CompletionSource,
    pub sort_key: u32,
}

/// State of the completion menu.
#[derive(Debug, Clone)]
pub struct CompletionState {
    items: Vec<CompletionItem>,
    selected: Option<usize>,
    prefix: String,
    active: bool,
}

impl CompletionState {
    pub fn new() -> Self { Self { items: Vec::new(), selected: None, prefix: String::new(), active: false } }
    pub fn is_active(&self) -> bool { self.active }
    pub fn items(&self) -> &[CompletionItem] { &self.items }
    pub fn selected_index(&self) -> Option<usize> { self.selected }
    pub fn prefix(&self) -> &str { &self.prefix }

    /// Start a completion session with the given prefix and candidates.
    pub fn start(&mut self, prefix: &str, mut items: Vec<CompletionItem>) {
        items.sort_by(|a, b| a.source.priority().cmp(&b.source.priority()).then(a.sort_key.cmp(&b.sort_key)));
        self.prefix = prefix.to_string();
        self.items = items;
        self.selected = if self.items.is_empty() { None } else { Some(0) };
        self.active = !self.items.is_empty();
    }

    /// Move selection forward (Tab).
    pub fn next(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        });
    }

    /// Move selection backward (Shift-Tab).
    pub fn prev(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = Some(match self.selected {
            Some(0) | None => self.items.len().saturating_sub(1),
            Some(i) => i - 1,
        });
    }

    /// Confirm the current selection. Returns the insert text.
    pub fn confirm(&mut self) -> Option<String> {
        let idx = self.selected?;
        let text = self.items.get(idx).map(|it| it.insert_text.clone());
        self.dismiss();
        text
    }

    /// Dismiss the completion menu without inserting.
    pub fn dismiss(&mut self) {
        self.items.clear();
        self.selected = None;
        self.prefix.clear();
        self.active = false;
    }

    /// Current selected item (if any).
    pub fn current_item(&self) -> Option<&CompletionItem> {
        self.selected.and_then(|i| self.items.get(i))
    }

    /// Filter existing items by a new prefix (narrowing).
    pub fn narrow(&mut self, new_prefix: &str) {
        self.items.retain(|it| it.label.starts_with(new_prefix));
        self.prefix = new_prefix.to_string();
        if self.items.is_empty() {
            self.dismiss();
        } else {
            self.selected = Some(self.selected.unwrap_or(0).min(self.items.len() - 1));
        }
    }
}

impl Default for CompletionState { fn default() -> Self { Self::new() } }
