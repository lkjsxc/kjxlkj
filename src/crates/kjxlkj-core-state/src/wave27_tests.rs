//! Wave 27 tests: regex collection, try/catch, fuzzy completion, bitwise ops,
//! alternate file mark, dot-repeat macro, snippet transforms, range function calls.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::expr_eval::eval_expression;
    use crate::regex_translate::translate_vim_to_rust;

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    #[test]
    fn test_regex_collection_class() {
        let r = translate_vim_to_rust(r"\%[abc]").pattern;
        assert_eq!(r, "[abc]");
    }

    #[test]
    fn test_regex_non_capturing_group() {
        let r = translate_vim_to_rust(r"\%(foo\|bar\)").pattern;
        assert!(r.contains("(?:"), "expected non-capturing group, got {r}");
    }

    #[test]
    fn test_try_catch_endtry() {
        let mut ed = editor_with("hello");
        // source a script with try/catch — should not crash
        ed.execute_ex_command("let g:tried = 0");
        // Simple try/endtry via individual lines exercising the skip stack
        let script = "try\nlet g:tried = 1\ncatch\nlet g:tried = 2\nendtry";
        let path = "/tmp/kjxlkj_test_try.vim";
        std::fs::write(path, script).unwrap();
        ed.execute_ex_command(&format!("source {path}"));
        let val = ed.options.get_str("g:tried");
        // try succeeded so g:tried should be 1 (catch skipped)
        assert_eq!(val, "1");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_fuzzy_path_scoring() {
        use crate::cmdline_completion_ctx::fuzzy_score;
        assert!(fuzzy_score("abc", "a_b_c").is_some(), "subsequence should match");
        assert!(fuzzy_score("xyz", "abc").is_none(), "non-match should be None");
        assert!(fuzzy_score("fc", "foo_config").unwrap() > 0);
    }

    #[test]
    fn test_bitwise_operators() {
        assert_eq!(eval_expression("and(12, 10)").unwrap(), "8");
        assert_eq!(eval_expression("or(12, 10)").unwrap(), "14");
        assert_eq!(eval_expression("xor(12, 10)").unwrap(), "6");
    }

    #[test]
    fn test_alternate_file_mark() {
        use crate::marks::{MarkFile, MarkPosition};
        let mut mf = MarkFile::new();
        mf.set_alternate(MarkPosition::new(1, 5, 0));
        let alt = mf.get_alternate();
        assert!(alt.is_some());
        assert_eq!(alt.unwrap().line, 5);
    }

    #[test]
    fn test_dot_repeat_macro_field() {
        let mut ed = editor_with("test");
        // Set last_macro to 'a' and verify DotRepeat doesn't crash
        ed.last_macro = Some('a');
        // DotRepeat with no recorded macro for 'a' should be harmless
        ed.handle_action(kjxlkj_core_types::Action::DotRepeat);
        assert!(true, "dot repeat with last_macro did not crash");
    }

    #[test]
    fn test_snippet_transformation_parse() {
        use crate::snippets::parse_tab_stops;
        // First define default for stop 1, then apply transformation
        let (text, stops) = parse_tab_stops("${1:hello} ${1/hello/world/g}");
        // The transformation should replace "hello" with "world" in the mirrored text
        assert!(text.contains("world"), "transform should apply: got {text}");
        assert!(!stops.is_empty(), "should have tab stops");
    }

    #[test]
    fn test_range_user_function_call() {
        use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
        use std::collections::HashMap;
        let lines_data = vec!["a", "b", "c", "d", "e"];
        let call_fn = |expr: &str| -> Option<String> {
            if expr.starts_with("GetLine(") { Some("3".to_string()) } else { None }
        };
        let ctx = RangeContext {
            current_line: 0, total_lines: 5, lines: &lines_data,
            mark_line: None, last_search: None,
            vars: Some(&HashMap::new()), call_fn: Some(&call_fn),
        };
        let (range, rest) = parse_range_ctx("(GetLine())d", &ctx);
        assert!(range.is_some(), "function call range should parse");
        // GetLine() returns 3, which becomes line index 2 (1-indexed → 0-indexed)
        assert_eq!(range.unwrap().start, 2);
        assert_eq!(rest, "d");
    }
}
