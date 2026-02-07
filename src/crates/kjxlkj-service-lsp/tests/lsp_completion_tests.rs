use kjxlkj_service_lsp::{CompletionItemEx, CompletionItemKind, CompletionList};

fn make_item(label: &str, kind: CompletionItemKind) -> CompletionItemEx {
    CompletionItemEx {
        label: label.into(), kind, detail: None, filter_text: None,
        sort_text: None, preselect: false, deprecated: false,
    }
}

#[test]
fn completion_kind_from_lsp_valid() {
    assert_eq!(CompletionItemKind::from_lsp(1), Some(CompletionItemKind::Text));
    assert_eq!(CompletionItemKind::from_lsp(3), Some(CompletionItemKind::Function));
    assert_eq!(CompletionItemKind::from_lsp(25), Some(CompletionItemKind::TypeParameter));
}

#[test]
fn completion_kind_from_lsp_invalid() {
    assert_eq!(CompletionItemKind::from_lsp(0), None);
    assert_eq!(CompletionItemKind::from_lsp(26), None);
}

#[test]
fn completion_kind_icons_not_empty() {
    let kinds = [
        CompletionItemKind::Text, CompletionItemKind::Method,
        CompletionItemKind::Function, CompletionItemKind::Variable,
    ];
    for k in kinds {
        assert!(!k.icon().is_empty());
    }
}

#[test]
fn completion_list_filter_prefix() {
    let list = CompletionList::new(vec![
        make_item("foo_bar", CompletionItemKind::Function),
        make_item("baz", CompletionItemKind::Variable),
    ]);
    assert_eq!(list.filter("foo").len(), 1);
    assert_eq!(list.filter("").len(), 2);
}

#[test]
fn completion_list_select_next_wraps() {
    let mut list = CompletionList::new(vec![
        make_item("a", CompletionItemKind::Text),
        make_item("b", CompletionItemKind::Text),
    ]);
    assert_eq!(list.selected, 0);
    list.select_next();
    assert_eq!(list.selected, 1);
    list.select_next();
    assert_eq!(list.selected, 0);
}

#[test]
fn completion_list_select_prev_wraps() {
    let mut list = CompletionList::new(vec![
        make_item("a", CompletionItemKind::Text),
        make_item("b", CompletionItemKind::Text),
    ]);
    list.select_prev();
    assert_eq!(list.selected, 1);
}

#[test]
fn completion_list_current() {
    let list = CompletionList::new(vec![make_item("x", CompletionItemKind::Keyword)]);
    assert_eq!(list.current().unwrap().label, "x");
}

#[test]
fn completion_item_ex_fields() {
    let item = CompletionItemEx {
        label: "test".into(), kind: CompletionItemKind::Snippet,
        detail: Some("detail".into()), filter_text: Some("ft".into()),
        sort_text: Some("00".into()), preselect: true, deprecated: true,
    };
    assert!(item.preselect);
    assert!(item.deprecated);
}
