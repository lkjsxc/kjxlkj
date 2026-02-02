//! Completion item type.

use crate::completion_kind::CompletionKind;
use serde::{Deserialize, Serialize};

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
    fn test_completion_item_with_insert() {
        let item = CompletionItem::new("foo", CompletionKind::Snippet).with_insert("foo()");
        assert_eq!(item.insert_text, "foo()");
    }

    #[test]
    fn test_completion_item_with_detail() {
        let item =
            CompletionItem::new("test", CompletionKind::Method).with_detail("A test method");
        assert_eq!(item.detail, Some("A test method".to_string()));
    }

    #[test]
    fn test_completion_item_with_doc() {
        let item =
            CompletionItem::new("test", CompletionKind::Method).with_doc("Documentation here");
        assert_eq!(item.documentation, Some("Documentation here".to_string()));
    }
}
