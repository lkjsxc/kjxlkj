//! Completion support.

use serde::{Deserialize, Serialize};

/// Completion item kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionKind {
    /// Text completion.
    Text,
    /// Method/function.
    Method,
    /// Function.
    Function,
    /// Constructor.
    Constructor,
    /// Field.
    Field,
    /// Variable.
    Variable,
    /// Class.
    Class,
    /// Interface.
    Interface,
    /// Module.
    Module,
    /// Property.
    Property,
    /// Unit.
    Unit,
    /// Value.
    Value,
    /// Enum.
    Enum,
    /// Keyword.
    Keyword,
    /// Snippet.
    Snippet,
    /// Color.
    Color,
    /// File.
    File,
    /// Reference.
    Reference,
    /// Folder.
    Folder,
    /// Enum member.
    EnumMember,
    /// Constant.
    Constant,
    /// Struct.
    Struct,
    /// Event.
    Event,
    /// Operator.
    Operator,
    /// Type parameter.
    TypeParameter,
}

/// A completion item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    /// Display label.
    pub label: String,
    /// Kind of item.
    pub kind: CompletionKind,
    /// Text to insert.
    pub insert_text: String,
    /// Filter text for matching.
    pub filter_text: Option<String>,
    /// Sort text.
    pub sort_text: Option<String>,
    /// Detail description.
    pub detail: Option<String>,
    /// Documentation.
    pub documentation: Option<String>,
}

impl CompletionItem {
    /// Creates a new completion item.
    pub fn new(label: impl Into<String>, kind: CompletionKind) -> Self {
        let label = label.into();
        Self {
            insert_text: label.clone(),
            label,
            kind,
            filter_text: None,
            sort_text: None,
            detail: None,
            documentation: None,
        }
    }

    /// Sets the insert text.
    pub fn with_insert(mut self, text: impl Into<String>) -> Self {
        self.insert_text = text.into();
        self
    }

    /// Sets the detail.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Sets the documentation.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.documentation = Some(doc.into());
        self
    }
}

/// Completion state.
#[derive(Debug, Clone, Default)]
pub struct CompletionState {
    /// Available items.
    items: Vec<CompletionItem>,
    /// Selected index.
    selected: usize,
    /// Filter prefix.
    prefix: String,
    /// Whether menu is visible.
    visible: bool,
}

impl CompletionState {
    /// Creates a new completion state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens the completion menu with items.
    pub fn open(&mut self, items: Vec<CompletionItem>, prefix: String) {
        self.items = items;
        self.prefix = prefix;
        self.selected = 0;
        self.visible = !self.items.is_empty();
    }

    /// Closes the completion menu.
    pub fn close(&mut self) {
        self.items.clear();
        self.visible = false;
    }

    /// Returns if visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the items.
    pub fn items(&self) -> &[CompletionItem] {
        &self.items
    }

    /// Returns filtered items matching prefix.
    pub fn filtered_items(&self) -> Vec<&CompletionItem> {
        if self.prefix.is_empty() {
            return self.items.iter().collect();
        }
        self.items.iter()
            .filter(|item| {
                let filter = item.filter_text.as_deref().unwrap_or(&item.label);
                filter.to_lowercase().starts_with(&self.prefix.to_lowercase())
            })
            .collect()
    }

    /// Returns the selected item.
    pub fn selected(&self) -> Option<&CompletionItem> {
        self.items.get(self.selected)
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Selects next item.
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    /// Selects previous item.
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            self.selected = self.selected
                .checked_sub(1)
                .unwrap_or(self.items.len() - 1);
        }
    }

    /// Updates the prefix filter.
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
        self.selected = 0;
    }

    /// Returns the prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_item_new() {
        let item = CompletionItem::new("foo", CompletionKind::Function);
        assert_eq!(item.label, "foo");
        assert_eq!(item.insert_text, "foo");
    }

    #[test]
    fn test_completion_item_with() {
        let item = CompletionItem::new("test", CompletionKind::Method)
            .with_detail("A test method")
            .with_doc("Documentation here");
        assert_eq!(item.detail, Some("A test method".to_string()));
    }

    #[test]
    fn test_completion_state_open_close() {
        let mut state = CompletionState::new();
        assert!(!state.is_visible());
        
        state.open(vec![CompletionItem::new("a", CompletionKind::Text)], "".to_string());
        assert!(state.is_visible());
        
        state.close();
        assert!(!state.is_visible());
    }

    #[test]
    fn test_completion_select() {
        let mut state = CompletionState::new();
        state.open(vec![
            CompletionItem::new("a", CompletionKind::Text),
            CompletionItem::new("b", CompletionKind::Text),
            CompletionItem::new("c", CompletionKind::Text),
        ], "".to_string());
        
        assert_eq!(state.selected_index(), 0);
        state.select_next();
        assert_eq!(state.selected_index(), 1);
        state.select_next();
        state.select_next();
        assert_eq!(state.selected_index(), 0); // Wraps
    }

    #[test]
    fn test_completion_filter() {
        let mut state = CompletionState::new();
        state.open(vec![
            CompletionItem::new("foo", CompletionKind::Text),
            CompletionItem::new("bar", CompletionKind::Text),
            CompletionItem::new("foobar", CompletionKind::Text),
        ], "foo".to_string());
        
        let filtered = state.filtered_items();
        assert_eq!(filtered.len(), 2);
    }
}
