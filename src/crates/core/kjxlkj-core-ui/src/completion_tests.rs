//! Tests for insert completion model.
#[cfg(test)]
mod tests {
    use crate::completion::{CompletionItem, CompletionSource, CompletionState};

    fn items() -> Vec<CompletionItem> {
        vec![
            CompletionItem { label: "foo".into(), insert_text: "foo".into(), detail: None, source: CompletionSource::Buffer, sort_key: 0 },
            CompletionItem { label: "fbar".into(), insert_text: "fbar".into(), detail: None, source: CompletionSource::Lsp, sort_key: 0 },
            CompletionItem { label: "faz".into(), insert_text: "faz".into(), detail: Some("fn".into()), source: CompletionSource::Lsp, sort_key: 1 },
        ]
    }

    #[test]
    fn initial_inactive() { let c = CompletionState::new(); assert!(!c.is_active()); }

    #[test]
    fn start_sorts_by_priority() {
        let mut c = CompletionState::new();
        c.start("f", items());
        assert!(c.is_active());
        assert_eq!(c.items()[0].source, CompletionSource::Lsp);
        assert_eq!(c.items()[2].source, CompletionSource::Buffer);
    }

    #[test]
    fn next_wraps() {
        let mut c = CompletionState::new();
        c.start("f", items());
        assert_eq!(c.selected_index(), Some(0));
        c.next(); assert_eq!(c.selected_index(), Some(1));
        c.next(); assert_eq!(c.selected_index(), Some(2));
        c.next(); assert_eq!(c.selected_index(), Some(0));
    }

    #[test]
    fn prev_wraps() {
        let mut c = CompletionState::new();
        c.start("f", items());
        c.prev(); assert_eq!(c.selected_index(), Some(2));
    }

    #[test]
    fn confirm_returns_text() {
        let mut c = CompletionState::new();
        c.start("f", items());
        let text = c.confirm();
        assert_eq!(text, Some("fbar".to_string()));
        assert!(!c.is_active());
    }

    #[test]
    fn dismiss_clears() {
        let mut c = CompletionState::new();
        c.start("f", items());
        c.dismiss();
        assert!(!c.is_active());
        assert!(c.items().is_empty());
    }

    #[test]
    fn narrow_filters() {
        let mut c = CompletionState::new();
        c.start("f", items());
        c.narrow("fo");
        assert_eq!(c.items().len(), 1);
        assert_eq!(c.items()[0].label, "foo");
    }

    #[test]
    fn narrow_to_empty_dismisses() {
        let mut c = CompletionState::new();
        c.start("f", items());
        c.narrow("xyz");
        assert!(!c.is_active());
    }

    #[test]
    fn source_priority_order() {
        assert!(CompletionSource::Lsp.priority() < CompletionSource::Buffer.priority());
        assert!(CompletionSource::Snippet.priority() < CompletionSource::Path.priority());
    }

    #[test]
    fn source_all_returns_8() { assert_eq!(CompletionSource::all().len(), 8); }

    #[test]
    fn current_item_none_when_empty() {
        let c = CompletionState::new();
        assert!(c.current_item().is_none());
    }
}
