//! Completion popup infrastructure.

/// A single completion item.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// The text to insert.
    pub label: String,
    /// Kind icon/type indicator.
    pub kind: CompletionKind,
    /// Details/documentation.
    pub detail: Option<String>,
    /// Sort text (if different from label).
    pub sort_text: Option<String>,
    /// Filter text (if different from label).
    pub filter_text: Option<String>,
    /// Text edit to apply (insert text + range).
    pub insert_text: Option<String>,
    /// Source of completion.
    pub source: CompletionSource,
}

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

/// Source of the completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionSource {
    Lsp,
    Buffer,
    Path,
    Snippet,
    Dictionary,
}

/// Completion popup state.
#[derive(Debug)]
pub struct CompletionPopup {
    /// All items.
    pub items: Vec<CompletionItem>,
    /// Filtered items (indices into `items`).
    pub filtered: Vec<usize>,
    /// Selected index in `filtered`.
    pub selected: usize,
    /// Whether popup is visible.
    pub visible: bool,
    /// Current filter prefix.
    pub prefix: String,
}

impl CompletionPopup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            filtered: Vec::new(),
            selected: 0,
            visible: false,
            prefix: String::new(),
        }
    }

    /// Set items and show popup.
    pub fn show(
        &mut self,
        items: Vec<CompletionItem>,
    ) {
        self.items = items;
        self.prefix.clear();
        self.filter();
        self.selected = 0;
        self.visible = !self.filtered.is_empty();
    }

    /// Hide the popup.
    pub fn hide(&mut self) {
        self.visible = false;
        self.items.clear();
        self.filtered.clear();
    }

    /// Update filter prefix and refilter.
    pub fn update_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
        self.filter();
        self.selected = 0;
        self.visible = !self.filtered.is_empty();
    }

    /// Filter items by prefix.
    fn filter(&mut self) {
        if self.prefix.is_empty() {
            self.filtered =
                (0..self.items.len()).collect();
        } else {
            let pfx = self.prefix.to_lowercase();
            self.filtered = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, it)| {
                    let ft = it
                        .filter_text
                        .as_deref()
                        .unwrap_or(&it.label);
                    ft.to_lowercase()
                        .starts_with(&pfx)
                })
                .map(|(i, _)| i)
                .collect();
        }
    }

    /// Select next item.
    pub fn next(&mut self) {
        if !self.filtered.is_empty() {
            self.selected = (self.selected + 1)
                % self.filtered.len();
        }
    }

    /// Select previous item.
    pub fn prev(&mut self) {
        if !self.filtered.is_empty() {
            if self.selected == 0 {
                self.selected =
                    self.filtered.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    /// Get the currently selected item.
    pub fn selected_item(
        &self,
    ) -> Option<&CompletionItem> {
        self.filtered
            .get(self.selected)
            .and_then(|&i| self.items.get(i))
    }

    /// Get the insert text for confirmed item.
    pub fn confirm(&self) -> Option<String> {
        self.selected_item().map(|it| {
            it.insert_text
                .clone()
                .unwrap_or_else(|| it.label.clone())
        })
    }
}

impl Default for CompletionPopup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_items() -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "println".into(),
                kind: CompletionKind::Function,
                detail: None,
                sort_text: None,
                filter_text: None,
                insert_text: Some(
                    "println!()".into(),
                ),
                source: CompletionSource::Lsp,
            },
            CompletionItem {
                label: "print".into(),
                kind: CompletionKind::Function,
                detail: None,
                sort_text: None,
                filter_text: None,
                insert_text: None,
                source: CompletionSource::Lsp,
            },
        ]
    }

    #[test]
    fn show_and_select() {
        let mut popup = CompletionPopup::new();
        popup.show(make_items());
        assert!(popup.visible);
        assert_eq!(popup.filtered.len(), 2);
        assert_eq!(
            popup.selected_item().unwrap().label,
            "println"
        );
    }

    #[test]
    fn filter_by_prefix() {
        let mut popup = CompletionPopup::new();
        popup.show(make_items());
        popup.update_prefix("printl");
        assert_eq!(popup.filtered.len(), 1);
    }

    #[test]
    fn next_prev_wrap() {
        let mut popup = CompletionPopup::new();
        popup.show(make_items());
        popup.next();
        assert_eq!(
            popup.selected_item().unwrap().label,
            "print"
        );
        popup.next();
        assert_eq!(
            popup.selected_item().unwrap().label,
            "println"
        );
    }
}
