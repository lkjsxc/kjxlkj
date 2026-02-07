use super::*;

fn sample_items() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "foo".into(),
            kind: CompletionKind::Function,
            detail: None,
            documentation: None,
            insert_text: None,
            sort_text: None,
            filter_text: None,
        },
        CompletionItem {
            label: "bar".into(),
            kind: CompletionKind::Variable,
            detail: None,
            documentation: None,
            insert_text: None,
            sort_text: None,
            filter_text: None,
        },
        CompletionItem {
            label: "fooBar".into(),
            kind: CompletionKind::Method,
            detail: None,
            documentation: None,
            insert_text: None,
            sort_text: None,
            filter_text: Some("foobar".into()),
        },
    ]
}

#[test]
fn filter_empty_prefix_returns_all() {
    let items = sample_items();
    assert_eq!(filter_completions(&items, "").len(), 3);
}

#[test]
fn filter_by_prefix() {
    let items = sample_items();
    let filtered = filter_completions(&items, "foo");
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|i| i.label.starts_with("foo")));
}

#[test]
fn filter_case_insensitive() {
    let items = sample_items();
    let filtered = filter_completions(&items, "FOO");
    assert_eq!(filtered.len(), 2);
}

#[test]
fn filter_no_match() {
    let items = sample_items();
    let filtered = filter_completions(&items, "xyz");
    assert!(filtered.is_empty());
}

#[test]
fn diagnostic_store_counts() {
    let mut store = DiagnosticStore::new();
    store.set(vec![
        Diagnostic {
            message: "err".into(),
            severity: DiagnosticSeverity::Error,
            line: 0,
            col: 0,
            end_line: None,
            end_col: None,
            source: None,
            code: None,
        },
        Diagnostic {
            message: "warn".into(),
            severity: DiagnosticSeverity::Warning,
            line: 1,
            col: 0,
            end_line: None,
            end_col: None,
            source: None,
            code: None,
        },
    ]);
    assert_eq!(store.error_count(), 1);
    assert_eq!(store.warning_count(), 1);
    assert_eq!(store.get().len(), 2);
}

#[test]
fn diagnostic_store_clear() {
    let mut store = DiagnosticStore::new();
    store.set(vec![Diagnostic {
        message: "x".into(),
        severity: DiagnosticSeverity::Hint,
        line: 0,
        col: 0,
        end_line: None,
        end_col: None,
        source: None,
        code: None,
    }]);
    store.clear();
    assert!(store.get().is_empty());
}

#[test]
fn completion_kind_variants() {
    let kinds = [
        CompletionKind::Text,
        CompletionKind::Method,
        CompletionKind::Function,
        CompletionKind::Constructor,
        CompletionKind::Field,
        CompletionKind::Variable,
        CompletionKind::Class,
        CompletionKind::Interface,
        CompletionKind::Module,
        CompletionKind::Property,
        CompletionKind::Keyword,
        CompletionKind::Snippet,
        CompletionKind::File,
    ];
    assert_eq!(kinds.len(), 13);
}
