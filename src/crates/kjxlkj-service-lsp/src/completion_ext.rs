//! Extended LSP completion types: CompletionItemKind, CompletionList, filtering.

use serde::{Deserialize, Serialize};

/// LSP completion item kind (maps to LSP spec CompletionItemKind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Text = 1, Method = 2, Function = 3, Constructor = 4, Field = 5,
    Variable = 6, Class = 7, Interface = 8, Module = 9, Property = 10,
    Unit = 11, Value = 12, Enum = 13, Keyword = 14, Snippet = 15,
    Color = 16, File = 17, Reference = 18, Folder = 19, EnumMember = 20,
    Constant = 21, Struct = 22, Event = 23, Operator = 24, TypeParameter = 25,
}

impl CompletionItemKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Text => "t", Self::Method | Self::Function => "f",
            Self::Constructor => "C", Self::Field | Self::Property => ".",
            Self::Variable => "v", Self::Class | Self::Struct => "S",
            Self::Interface => "I", Self::Module => "M", Self::Unit | Self::Value => "V",
            Self::Enum | Self::EnumMember => "E", Self::Keyword => "k",
            Self::Snippet => "s", Self::Color => "#", Self::File | Self::Folder => "/",
            Self::Reference => "&", Self::Constant => "c",
            Self::Event => "!", Self::Operator => "o", Self::TypeParameter => "T",
        }
    }
    pub fn from_lsp(n: u32) -> Option<Self> {
        match n {
            1 => Some(Self::Text), 2 => Some(Self::Method), 3 => Some(Self::Function),
            4 => Some(Self::Constructor), 5 => Some(Self::Field), 6 => Some(Self::Variable),
            7 => Some(Self::Class), 8 => Some(Self::Interface), 9 => Some(Self::Module),
            10 => Some(Self::Property), 11 => Some(Self::Unit), 12 => Some(Self::Value),
            13 => Some(Self::Enum), 14 => Some(Self::Keyword), 15 => Some(Self::Snippet),
            16 => Some(Self::Color), 17 => Some(Self::File), 18 => Some(Self::Reference),
            19 => Some(Self::Folder), 20 => Some(Self::EnumMember), 21 => Some(Self::Constant),
            22 => Some(Self::Struct), 23 => Some(Self::Event), 24 => Some(Self::Operator),
            25 => Some(Self::TypeParameter), _ => None,
        }
    }
}

/// An extended completion item with rich metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemEx {
    pub label: String,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub filter_text: Option<String>,
    pub sort_text: Option<String>,
    pub preselect: bool,
    pub deprecated: bool,
}

impl CompletionItemEx {
    pub fn simple(label: impl Into<String>) -> Self {
        Self { label: label.into(), kind: None, detail: None, documentation: None,
               insert_text: None, filter_text: None, sort_text: None,
               preselect: false, deprecated: false }
    }
    pub fn text_to_insert(&self) -> &str {
        self.insert_text.as_deref().unwrap_or(&self.label)
    }
    pub fn matches_prefix(&self, prefix: &str) -> bool {
        let filter = self.filter_text.as_deref().unwrap_or(&self.label);
        filter.to_lowercase().starts_with(&prefix.to_lowercase())
    }
}

/// A completion list with pagination support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionList {
    pub items: Vec<CompletionItemEx>,
    pub is_incomplete: bool,
    pub selected: Option<usize>,
}

impl CompletionList {
    pub fn new(items: Vec<CompletionItemEx>, incomplete: bool) -> Self {
        let selected = if items.is_empty() { None } else { Some(0) };
        Self { items, is_incomplete: incomplete, selected }
    }
    pub fn empty() -> Self { Self { items: Vec::new(), is_incomplete: false, selected: None } }

    pub fn filter(&self, prefix: &str) -> Self {
        let items: Vec<_> = self.items.iter().filter(|i| i.matches_prefix(prefix)).cloned().collect();
        Self::new(items, self.is_incomplete)
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = Some(self.selected.map_or(0, |i| (i + 1) % self.items.len()));
    }
    pub fn select_prev(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = Some(self.selected.map_or(0, |i| if i == 0 { self.items.len() - 1 } else { i - 1 }));
    }
    pub fn selected_item(&self) -> Option<&CompletionItemEx> {
        self.selected.and_then(|i| self.items.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_icons() {
        assert_eq!(CompletionItemKind::Function.icon(), "f");
        assert_eq!(CompletionItemKind::Variable.icon(), "v");
        assert_eq!(CompletionItemKind::Struct.icon(), "S");
    }

    #[test]
    fn from_lsp_roundtrip() {
        for n in 1..=25 {
            assert!(CompletionItemKind::from_lsp(n).is_some());
        }
        assert!(CompletionItemKind::from_lsp(0).is_none());
        assert!(CompletionItemKind::from_lsp(99).is_none());
    }

    #[test]
    fn item_matches_prefix() {
        let item = CompletionItemEx::simple("HashMap");
        assert!(item.matches_prefix("hash"));
        assert!(item.matches_prefix("Hash"));
        assert!(!item.matches_prefix("Vec"));
    }

    #[test]
    fn item_insert_text_fallback() {
        let item = CompletionItemEx::simple("foo");
        assert_eq!(item.text_to_insert(), "foo");
        let mut item2 = CompletionItemEx::simple("bar");
        item2.insert_text = Some("bar()".into());
        assert_eq!(item2.text_to_insert(), "bar()");
    }

    #[test]
    fn list_filter() {
        let items = vec![CompletionItemEx::simple("HashMap"), CompletionItemEx::simple("HashSet"),
                          CompletionItemEx::simple("Vec")];
        let list = CompletionList::new(items, false);
        let filtered = list.filter("Hash");
        assert_eq!(filtered.items.len(), 2);
    }

    #[test]
    fn list_select_next_prev() {
        let items = vec![CompletionItemEx::simple("a"), CompletionItemEx::simple("b"), CompletionItemEx::simple("c")];
        let mut list = CompletionList::new(items, false);
        assert_eq!(list.selected, Some(0));
        list.select_next();
        assert_eq!(list.selected, Some(1));
        list.select_next(); list.select_next(); // wraps
        assert_eq!(list.selected, Some(0));
        list.select_prev(); // wraps to end
        assert_eq!(list.selected, Some(2));
    }

    #[test]
    fn empty_list() {
        let list = CompletionList::empty();
        assert!(list.items.is_empty());
        assert!(list.selected_item().is_none());
    }
}
