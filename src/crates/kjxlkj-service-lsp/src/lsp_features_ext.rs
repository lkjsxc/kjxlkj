//! Extended LSP types: rich completion items and lists.

use serde::{Deserialize, Serialize};

/// LSP completion item kind (25 kinds per spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

impl CompletionItemKind {
    /// Icon character for the kind.
    pub fn icon(self) -> &'static str {
        match self {
            Self::Text => "T",
            Self::Method | Self::Function => "f",
            Self::Constructor => "C",
            Self::Field | Self::Property => ".",
            Self::Variable => "v",
            Self::Class | Self::Struct => "S",
            Self::Interface => "I",
            Self::Module => "M",
            Self::Unit | Self::Value => "=",
            Self::Enum | Self::EnumMember => "E",
            Self::Keyword => "k",
            Self::Snippet => "{}",
            Self::Color => "#",
            Self::File => "F",
            Self::Reference => "&",
            Self::Folder => "D",
            Self::Constant => "c",
            Self::Event => "!",
            Self::Operator => "o",
            Self::TypeParameter => "P",
        }
    }

    /// Convert from LSP numeric kind.
    pub fn from_lsp(value: u8) -> Option<Self> {
        if value >= 1 && value <= 25 {
            // SAFETY: repr(u8) and range is validated.
            Some(unsafe { std::mem::transmute(value) })
        } else {
            None
        }
    }
}

/// A rich completion item with extra metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemEx {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub filter_text: Option<String>,
    pub sort_text: Option<String>,
    pub preselect: bool,
    pub deprecated: bool,
}

/// A filterable, selectable completion list.
#[derive(Debug, Default)]
pub struct CompletionList {
    pub items: Vec<CompletionItemEx>,
    pub selected: usize,
}

impl CompletionList {
    pub fn new(items: Vec<CompletionItemEx>) -> Self {
        Self { items, selected: 0 }
    }

    /// Filter items by prefix match on label or filter_text.
    pub fn filter(&self, prefix: &str) -> Vec<&CompletionItemEx> {
        let lower = prefix.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                let text = item.filter_text.as_deref().unwrap_or(&item.label);
                text.to_lowercase().starts_with(&lower)
            })
            .collect()
    }

    /// Select next item (wrapping).
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    /// Select previous item (wrapping).
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            self.selected = if self.selected == 0 {
                self.items.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Get the currently selected item.
    pub fn current(&self) -> Option<&CompletionItemEx> {
        self.items.get(self.selected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_roundtrip() {
        assert_eq!(CompletionItemKind::from_lsp(3), Some(CompletionItemKind::Function));
        assert_eq!(CompletionItemKind::from_lsp(0), None);
        assert_eq!(CompletionItemKind::from_lsp(26), None);
    }

    #[test]
    fn filter_prefix() {
        let list = CompletionList::new(vec![
            CompletionItemEx {
                label: "foo_bar".into(),
                kind: CompletionItemKind::Function,
                detail: None,
                filter_text: None,
                sort_text: None,
                preselect: false,
                deprecated: false,
            },
            CompletionItemEx {
                label: "baz".into(),
                kind: CompletionItemKind::Variable,
                detail: None,
                filter_text: None,
                sort_text: None,
                preselect: false,
                deprecated: false,
            },
        ]);
        assert_eq!(list.filter("foo").len(), 1);
        assert_eq!(list.filter("").len(), 2);
    }
}
