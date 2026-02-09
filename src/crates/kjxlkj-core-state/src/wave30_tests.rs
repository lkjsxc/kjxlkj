#[cfg(test)]
mod tests {
    use crate::editor::EditorState;
    use crate::regex_translate::translate_vim_to_rust;
    use kjxlkj_core_types::{Key, KeyCode, Modifier};

    fn editor_with(path: &str, text: &str) -> EditorState {
        let mut e = EditorState::new(80, 24);
        e.open_file(path, text);
        e
    }

    /// REQ-REGEXZS-01: \zs sets match start, \ze sets match end.
    #[test]
    fn regex_zs_ze_match_bounds() {
        let r = translate_vim_to_rust(r"foo\zsbar\zebaz");
        // Should produce: (?:foo)(bar)(?:baz)
        assert!(r.pattern.contains("(?:foo)"), "prefix group: {}", r.pattern);
        assert!(r.pattern.contains("(bar)"), "match group: {}", r.pattern);
        assert!(r.pattern.contains("(?:baz)"), "suffix group: {}", r.pattern);
        assert!(r.has_match_bounds);
        // Compile and verify it matches.
        let re = regex::Regex::new(&r.pattern).unwrap();
        let caps = re.captures("foobarbaz").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "bar");
    }

    /// REQ-REGEXZS-01: \zs only (no \ze).
    #[test]
    fn regex_zs_only() {
        let r = translate_vim_to_rust(r"prefix\zsword");
        let re = regex::Regex::new(&r.pattern).unwrap();
        let caps = re.captures("prefixword").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "word");
    }

    /// REQ-ECHOMSG-01: :echomsg displays info notification.
    #[test]
    fn echomsg_command() {
        let mut e = editor_with("test.txt", "hello");
        e.execute_ex_command("echomsg \"test message\"");
        let n = e.notifications.last().unwrap();
        assert!(n.message.contains("test message"));
    }

    /// REQ-ECHOMSG-01: :echohl sets highlight group option.
    #[test]
    fn echohl_sets_option() {
        let mut e = editor_with("test.txt", "hello");
        e.execute_ex_command("echohl ErrorMsg");
        assert_eq!(e.options.get_str("echohl"), "ErrorMsg");
        e.execute_ex_command("echohl");
        assert_eq!(e.options.get_str("echohl"), "");
    }

    /// REQ-VBLOCKCO-01: Visual block insert uses tracked column.
    #[test]
    fn visual_block_insert_column() {
        let mut e = editor_with("test.txt", "aaaa\nbbbb\ncccc\n");
        // Set up block insert at column 2.
        e.block_insert_pending = Some((0, 2, 2, false));
        e.last_inserted_text = "X".to_string();
        // Simulate leaving insert mode to trigger block insert.
        e.mode = kjxlkj_core_types::Mode::Insert;
        e.handle_key(Key::new(KeyCode::Esc, Modifier::NONE));
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let line0: String = buf.content.line(0).chars().collect();
        // The X should be inserted at col 2 on all lines.
        assert!(line0.contains("X"), "block insert at col 2: {line0}");
    }

    /// REQ-COMPLPRI-01: Completion priority orders builtin before user commands.
    #[test]
    fn completion_priority_ordering() {
        let mut e = editor_with("test.txt", "hello");
        // Define a user command starting with "W"
        e.execute_ex_command("command! Wtest echo test");
        // Tab-complete "w" — builtin "write" should come before "Wtest"
        e.cmdline.open(':');
        e.cmdline.content = "w".to_string();
        e.cmdline_complete_next();
        let cs = &e.cmdline.completion;
        assert!(!cs.candidates.is_empty());
        // First candidate should be a builtin starting with w.
        assert!(cs.candidates[0].starts_with("w"), "first: {}", cs.candidates[0]);
    }

    /// REQ-EXPRREG-01: Expression register = stores result for put.
    #[test]
    fn expression_register_put() {
        let mut e = editor_with("test.txt", "hello");
        // Simulate "= prompt execute with expression.
        e.cmdline.open('=');
        e.cmdline.content = "2+3".to_string();
        e.execute_cmdline();
        // Result should be stored in expression register.
        let reg = e.registers.get(kjxlkj_core_edit::RegisterName::Expression);
        assert!(reg.is_some());
        assert_eq!(reg.unwrap().content, "5");
        // pending_register should be set to '='
        assert_eq!(e.pending_register, Some('='));
    }

    /// REQ-ROT13-01: g? enters operator-pending with Rot13.
    #[test]
    fn g_question_rot13_operator() {
        let mut e = editor_with("test.txt", "hello world\n");
        // g? should enter OperatorPending(Rot13)
        e.handle_key(Key::char('g'));
        e.handle_key(Key::char('?'));
        // Should be in OperatorPending(Rot13) or already applied (g??) doubled.
        // Actually g? enters OP pending, then we need a motion.
        // After g??, it should apply rot13 to the line.
        e.handle_key(Key::char('?'));
        let buf = e.buffers.get(e.current_buffer_id()).unwrap();
        let line: String = buf.content.line(0).chars().collect();
        assert!(line.starts_with("uryyb"), "ROT13 of 'hello' should be 'uryyb', got: {line}");
    }

    /// REQ-SPELL-01: Spell checker stub with good/bad word lists.
    #[test]
    fn spell_checker_stub() {
        let mut e = editor_with("test.txt", "hello");
        assert!(!e.spell.enabled);
        e.toggle_spell(true);
        assert!(e.spell.enabled);
        e.spell.add_good("hello".to_string());
        assert!(e.spell.is_good("hello"));
        assert!(!e.spell.is_bad("hello"));
        e.spell.add_bad("typo".to_string());
        assert!(e.spell.is_bad("typo"));
        e.spell.undo_good("hello");
        assert!(!e.spell.is_good("hello"));
    }

    /// REQ-EXRC-01: load_local_exrc checks .exrc file existence.
    #[test]
    fn local_exrc_loading() {
        let mut e = editor_with("test.txt", "hello");
        // exrc option disabled by default — should not load.
        e.load_local_exrc();
        // Enable exrc option.
        e.options.set("exrc", crate::options::OptionValue::Bool(true));
        // No .exrc file exists, so nothing happens (no error).
        e.load_local_exrc();
        // Verify no crashes.
        assert!(!e.quit_requested);
    }
}
