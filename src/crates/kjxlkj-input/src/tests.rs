//! Tests for input module.

#[cfg(test)]
mod key_tests {
    use crate::{Key, KeyCodeWrapper, KeySequence, Modifiers};

    #[test]
    fn test_key_char() {
        let key = Key::char('a');
        assert_eq!(key.code, KeyCodeWrapper::Char('a'));
        assert!(!key.modifiers.ctrl);
    }

    #[test]
    fn test_key_ctrl() {
        let key = Key::ctrl('c');
        assert_eq!(key.code, KeyCodeWrapper::Char('c'));
        assert!(key.modifiers.ctrl);
    }

    #[test]
    fn test_key_esc() {
        let key = Key::esc();
        assert!(key.is_esc());
    }

    #[test]
    fn test_modifiers_none() {
        let mods = Modifiers::none();
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.shift);
    }

    #[test]
    fn test_modifiers_ctrl() {
        let mods = Modifiers::ctrl();
        assert!(mods.ctrl);
        assert!(!mods.alt);
    }

    #[test]
    fn test_modifiers_alt() {
        let mods = Modifiers::alt();
        assert!(mods.alt);
        assert!(!mods.ctrl);
    }

    #[test]
    fn test_key_sequence_new() {
        let seq = KeySequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_key_sequence_push() {
        let mut seq = KeySequence::new();
        seq.push(Key::char('a'));
        seq.push(Key::char('b'));
        assert_eq!(seq.len(), 2);
    }

    #[test]
    fn test_key_sequence_clear() {
        let mut seq = KeySequence::new();
        seq.push(Key::char('x'));
        seq.clear();
        assert!(seq.is_empty());
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{InputParser, Key};
    use kjxlkj_core_mode::IntentKind;
    use kjxlkj_core_types::Mode;

    #[test]
    fn test_parser_new() {
        let parser = InputParser::new();
        assert!(parser.cmdline().input().is_empty());
    }

    #[test]
    fn test_parse_motion_h() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::char('h'), Mode::Normal);
        assert!(intent.is_some());
        if let Some(i) = intent {
            match i.kind {
                IntentKind::Motion(_) => {}
                _ => panic!("Expected Motion intent"),
            }
        }
    }

    #[test]
    fn test_parse_motion_j() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::char('j'), Mode::Normal);
        assert!(intent.is_some());
    }

    #[test]
    fn test_parse_motion_k() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::char('k'), Mode::Normal);
        assert!(intent.is_some());
    }

    #[test]
    fn test_parse_motion_l() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::char('l'), Mode::Normal);
        assert!(intent.is_some());
    }

    #[test]
    fn test_parse_mode_change_i() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::char('i'), Mode::Normal);
        assert!(intent.is_some());
    }

    #[test]
    fn test_parse_escape() {
        let mut parser = InputParser::new();
        let intent = parser.parse(Key::esc(), Mode::Normal);
        assert!(intent.is_some());
    }

    #[test]
    fn test_parse_count() {
        let mut parser = InputParser::new();
        parser.parse(Key::char('3'), Mode::Normal);
        let intent = parser.parse(Key::char('j'), Mode::Normal);
        assert!(intent.is_some());
        if let Some(i) = intent {
            assert_eq!(i.count, 3);
        }
    }
}

#[cfg(test)]
mod validation_tests {
    use crate::validation::*;

    #[test]
    fn test_validate_register() {
        assert!(validate_register('a').is_valid());
        assert!(validate_register('z').is_valid());
        assert!(validate_register('0').is_valid());
        assert!(validate_register('"').is_valid());
    }

    #[test]
    fn test_validate_mark() {
        assert!(validate_mark('a').is_valid());
        assert!(validate_mark('A').is_valid());
    }

    #[test]
    fn test_validate_line_number() {
        assert!(validate_line_number(1, 100).is_valid());
        assert!(validate_line_number(0, 100).is_valid()); // 0-based indexing
        assert!(!validate_line_number(100, 100).is_valid());
    }

    #[test]
    fn test_validate_column() {
        assert!(validate_column(0, 80).is_valid());
        assert!(!validate_column(81, 80).is_valid());
    }

    #[test]
    fn test_validate_count() {
        assert!(validate_count(1).is_valid());
        assert!(!validate_count(0).is_valid());
    }

    #[test]
    fn test_validate_tab_width() {
        assert!(validate_tab_width(4).is_valid());
        assert!(!validate_tab_width(0).is_valid());
        assert!(!validate_tab_width(17).is_valid());
    }
}
