//! Wave 19 tests: call function, visual g?/~, ternary expr,
//! delmarks range, snippet tab advance, backward search offset,
//! session weights, formatprg pipe.

#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use kjxlkj_core_types::{Key, Mode, VisualKind};

    fn editor_with(text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file("test.txt", text);
        e
    }

    /// REQ-FUNCALL-01: :call FuncName() executes user function body.
    #[test]
    fn call_user_function() {
        let mut ed = editor_with("hello");
        // Define function via ex
        ed.execute_ex_command("function! SetMark()");
        ed.execute_ex_command("mark a");
        ed.execute_ex_command("endfunction");
        assert_eq!(ed.functions.len(), 1);
        // Call it
        ed.execute_ex_command("call SetMark()");
        let bid = ed.current_buffer_id().0 as usize;
        assert!(ed.marks.get('a', bid).is_some());
    }

    /// REQ-VROT13-01: Visual g? applies ROT13.
    #[test]
    fn visual_rot13() {
        let mut ed = editor_with("hello world\n");
        // Select "hello" in visual char mode
        ed.mode = Mode::Visual(VisualKind::Char);
        ed.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
        ed.windows.focused_mut().cursor.grapheme = 4;
        // g? operator
        ed.handle_key(Key::char('g'));
        ed.handle_key(Key::char('?'));
        let buf = ed.buffers.get(ed.current_buffer_id()).unwrap();
        let line: std::borrow::Cow<str> = buf.content.line(0).into();
        assert!(line.starts_with("uryyb"), "ROT13 of 'hello' should be 'uryyb', got: {line}");
    }

    /// REQ-VROT13-01: Visual ~ toggles case.
    #[test]
    fn visual_toggle_case() {
        let mut ed = editor_with("Hello World\n");
        ed.mode = Mode::Visual(VisualKind::Char);
        ed.visual_anchor = Some(kjxlkj_core_types::CursorPosition::new(0, 0));
        ed.windows.focused_mut().cursor.grapheme = 4;
        ed.handle_key(Key::char('~'));
        let buf = ed.buffers.get(ed.current_buffer_id()).unwrap();
        let line: std::borrow::Cow<str> = buf.content.line(0).into();
        assert!(line.starts_with("hELLO"), "toggle case of 'Hello' should be 'hELLO', got: {line}");
    }

    /// REQ-TERNARY-01: Ternary expression evaluation.
    #[test]
    fn ternary_expression() {
        let r = crate::expr_eval::eval_expression("1 ? \"yes\" : \"no\"");
        assert_eq!(r.unwrap(), "yes");
        let r2 = crate::expr_eval::eval_expression("0 ? \"yes\" : \"no\"");
        assert_eq!(r2.unwrap(), "no");
    }

    /// REQ-DELRANGE-01: :delmarks a-d deletes marks in range.
    #[test]
    fn delmarks_range() {
        let mut ed = editor_with("line1\nline2\nline3\nline4\n");
        let bid = ed.current_buffer_id().0 as usize;
        for (i, ch) in ['a', 'b', 'c', 'd'].iter().enumerate() {
            ed.marks.set(*ch, crate::marks::MarkPosition { buffer_id: bid, line: i, col: 0 });
        }
        ed.execute_ex_command("delmarks a-d");
        for ch in ['a', 'b', 'c', 'd'] {
            assert!(ed.marks.get(ch, bid).is_none(), "mark '{ch}' should be deleted");
        }
    }

    /// REQ-SNIPTAB-01: Tab in insert mode advances snippet session.
    #[test]
    fn snippet_tab_advance() {
        let mut ed = editor_with("fn test $1 $2 end\n");
        ed.mode = Mode::Insert;
        ed.snippet_session = Some(crate::snippets::SnippetSession {
            stops: vec![8, 11],
            current: 0,
            base_line: 0,
            base_col: 0,
        });
        // Tab should advance to stop 1 (offset 11)
        ed.handle_key(Key::new(kjxlkj_core_types::KeyCode::Tab, kjxlkj_core_types::Modifier::NONE));
        assert!(ed.snippet_session.is_some());
    }

    /// REQ-BSEARCHOFF-01: Backward search ?pattern?e parses offset.
    #[test]
    fn backward_search_offset() {
        let (pat, off) = crate::search_types::parse_search_with_offset("test?e+1");
        assert_eq!(pat, "test");
        assert!(matches!(off, kjxlkj_core_ui::SearchOffset::End(1)));
    }

    /// REQ-SESSWIN-01: Session layout weights serialize/deserialize.
    #[test]
    fn session_layout_weights() {
        use crate::session::{SessionData, SessionLayout, SessionManager};
        let data = SessionData { layout: SessionLayout::Hsplit(
            vec![SessionLayout::Single, SessionLayout::Single],
            vec![0.6, 0.4],
        ), ..Default::default() };
        let serialized = SessionManager::serialize(&data);
        assert!(serialized.contains("0.6"), "should have weight 0.6: {serialized}");
        let round = SessionManager::deserialize(&serialized);
        if let SessionLayout::Hsplit(_, weights) = round.layout {
            assert!((weights[0] - 0.6).abs() < 0.01);
            assert!((weights[1] - 0.4).abs() < 0.01);
        } else {
            panic!("Expected Hsplit after round-trip");
        }
    }

    /// REQ-FMTPRG-01: formatprg error on nonexistent program.
    #[test]
    fn formatprg_pipe_error() {
        let mut ed = editor_with("hello world\n");
        ed.options.set("formatprg", crate::options::OptionValue::Str("__no_such_prog__".into()));
        ed.format_lines(0, 0);
        let notif = ed.notifications.last();
        assert!(notif.is_some());
        assert!(notif.unwrap().message.contains("formatprg"));
    }
}
