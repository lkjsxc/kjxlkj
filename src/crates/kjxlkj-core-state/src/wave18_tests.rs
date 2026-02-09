//! Wave 18 tests: visual u/U case, expression comparisons,
//! delmarks!, snippet tab-stop, search offsets, function defs,
//! popup menu snapshot.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, VisualKind};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    #[test]
    fn visual_lowercase_selection() {
        let mut e = editor_with("HELLO\n");
        e.handle_key(Key::char('v'));
        for _ in 0..4 { e.handle_key(Key::char('l')); }
        assert!(matches!(e.mode, Mode::Visual(VisualKind::Char)));
        e.handle_key(Key::char('u'));
        assert!(matches!(e.mode, Mode::Normal));
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        assert!(buf.content.to_string().starts_with("hello"));
    }

    #[test]
    fn visual_uppercase_selection() {
        let mut e = editor_with("hello\n");
        e.handle_key(Key::char('v'));
        for _ in 0..4 { e.handle_key(Key::char('l')); }
        e.handle_key(Key::char('U'));
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        assert!(buf.content.to_string().starts_with("HELLO"));
    }

    #[test]
    fn expr_comparison_operators() {
        use crate::expr_eval::eval_expression;
        assert_eq!(eval_expression("3==3").unwrap(), "1");
        assert_eq!(eval_expression("3==4").unwrap(), "0");
        assert_eq!(eval_expression("5!=3").unwrap(), "1");
        assert_eq!(eval_expression("5!=5").unwrap(), "0");
        assert_eq!(eval_expression("2<5").unwrap(), "1");
        assert_eq!(eval_expression("5<2").unwrap(), "0");
        assert_eq!(eval_expression("5>2").unwrap(), "1");
        assert_eq!(eval_expression("5<=5").unwrap(), "1");
        assert_eq!(eval_expression("6>=5").unwrap(), "1");
    }

    #[test]
    fn delmarks_bang_clears_local() {
        let mut e = editor_with("line1\nline2\n");
        let bid = e.current_buffer_id();
        e.marks.set('a', crate::marks::MarkPosition::new(bid.0 as usize, 0, 0));
        assert!(e.marks.get('a', bid.0 as usize).is_some());
        e.execute_ex_command("delmarks!");
        assert!(e.marks.get('a', bid.0 as usize).is_none());
    }

    #[test]
    fn snippet_expand_at_first_tabstop() {
        let mut reg = crate::snippets::SnippetRegistry::new();
        reg.add("fn", "fn $1() {\n    $2\n}$0", "function");
        let (text, mut session) = reg.expand_at("fn", 0, 0).unwrap();
        assert_eq!(text, "fn () {\n    \n}");
        // First tab-stop should be offset 3 (after "fn ")
        assert_eq!(session.current_offset(), Some(3));
        assert!(session.advance());
        // Second tab-stop
        assert_eq!(session.current_offset(), Some(12));
        assert!(session.advance());
        // $0 end position
        assert_eq!(session.current_offset(), Some(14));
        assert!(!session.advance());
    }

    #[test]
    fn search_offset_parsing() {
        use crate::search_types::{parse_search_with_offset, SearchOffset};
        let (pat, off) = parse_search_with_offset("hello/e");
        assert_eq!(pat, "hello");
        assert_eq!(off, SearchOffset::End(0));
        let (pat, off) = parse_search_with_offset("hello/s+2");
        assert_eq!(pat, "hello");
        assert_eq!(off, SearchOffset::Start(2));
        let (pat, off) = parse_search_with_offset("hello/+3");
        assert_eq!(pat, "hello");
        assert_eq!(off, SearchOffset::Lines(3));
        let (pat, off) = parse_search_with_offset("simple");
        assert_eq!(pat, "simple");
        assert_eq!(off, SearchOffset::None);
    }

    #[test]
    fn function_definition_via_ex() {
        let mut e = editor_with("x\n");
        e.execute_ex_command("function! MyFunc(a, b)");
        assert!(e.function_body_acc.is_some());
        e.execute_ex_command("echo a:a");
        e.execute_ex_command("endfunction");
        assert!(e.function_body_acc.is_none());
        let f = e.functions.get("MyFunc").unwrap();
        assert_eq!(f.params, vec!["a", "b"]);
        assert_eq!(f.body, vec!["echo a:a"]);
    }

    #[test]
    fn popup_menu_from_completion() {
        let mut e = editor_with("x\n");
        // Manually populate completion candidates
        e.cmdline.completion.candidates =
            vec!["set".into(), "setlocal".into()];
        e.cmdline.completion.index = Some(0);
        let snap = e.snapshot();
        let menu = snap.popup_menu.unwrap();
        assert_eq!(menu.items.len(), 2);
        assert_eq!(menu.selected, Some(0));
    }
}
