//! Comprehensive tests for kjxlkj-core-mode.

use kjxlkj_core_mode::*;
use kjxlkj_core_types::{KeyEvent, Mode, Position};

mod mode_state_tests {
    use super::*;

    #[test]
    fn test_initial_mode_is_normal() {
        let state = ModeState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn test_set_mode() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        assert_eq!(state.mode(), Mode::Insert);
    }

    #[test]
    fn test_set_mode_visual() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Visual);
        assert_eq!(state.mode(), Mode::Visual);
    }

    #[test]
    fn test_set_mode_command() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Command);
        assert_eq!(state.mode(), Mode::Command);
    }

    #[test]
    fn test_count_defaults_to_one() {
        let state = ModeState::new();
        assert_eq!(state.count(), 1);
    }

    #[test]
    fn test_raw_count_none_initially() {
        let state = ModeState::new();
        assert!(state.raw_count().is_none());
    }

    #[test]
    fn test_set_count() {
        let mut state = ModeState::new();
        state.set_count(5);
        assert_eq!(state.count(), 5);
    }

    #[test]
    fn test_accumulate_count() {
        let mut state = ModeState::new();
        state.accumulate_count('3');
        assert_eq!(state.raw_count(), Some(3));
        state.accumulate_count('5');
        assert_eq!(state.raw_count(), Some(35));
    }

    #[test]
    fn test_clear_count() {
        let mut state = ModeState::new();
        state.set_count(10);
        state.clear_count();
        assert!(state.raw_count().is_none());
    }

    #[test]
    fn test_operator_none_initially() {
        let state = ModeState::new();
        assert!(state.operator().is_none());
    }

    #[test]
    fn test_set_operator() {
        let mut state = ModeState::new();
        state.set_operator('d');
        assert_eq!(state.operator(), Some('d'));
    }

    #[test]
    fn test_clear_operator() {
        let mut state = ModeState::new();
        state.set_operator('c');
        state.clear_operator();
        assert!(state.operator().is_none());
    }

    #[test]
    fn test_visual_anchor() {
        let mut state = ModeState::new();
        assert!(state.visual_anchor().is_none());
        state.set_visual_anchor(Position::new(5, 10));
        let anchor = state.visual_anchor().unwrap();
        assert_eq!(anchor.line, 5);
        assert_eq!(anchor.column, 10);
    }

    #[test]
    fn test_register() {
        let mut state = ModeState::new();
        assert!(state.register().is_none());
        state.set_register('a');
        assert_eq!(state.register(), Some('a'));
        state.clear_register();
        assert!(state.register().is_none());
    }

    #[test]
    fn test_last_find() {
        let mut state = ModeState::new();
        assert!(state.last_find().is_none());
        state.set_last_find('x', true, false);
        let find = state.last_find().unwrap();
        assert_eq!(find.0, 'x');
        assert!(find.1);
        assert!(!find.2);
    }

    #[test]
    fn test_command_line() {
        let mut state = ModeState::new();
        assert_eq!(state.command_line(), "");
        state.command_line_push('w');
        assert_eq!(state.command_line(), "w");
        state.command_line_push('q');
        assert_eq!(state.command_line(), "wq");
        let c = state.command_line_pop();
        assert_eq!(c, Some('q'));
        assert_eq!(state.command_line(), "w");
    }
}

mod parser_tests {
    use super::*;

    #[test]
    fn test_parser_feed_no_match() {
        let mut parser = Parser::new();
        let result = parser.feed(KeyEvent::char('x'));
        assert!(matches!(result, ParseResult::NoMatch));
    }

    #[test]
    fn test_parser_feed_incomplete() {
        let mut parser = Parser::new();
        let result = parser.feed(KeyEvent::char('g'));
        assert!(matches!(result, ParseResult::Incomplete));
    }

    #[test]
    fn test_parser_feed_gg() {
        let mut parser = Parser::new();
        let r1 = parser.feed(KeyEvent::char('g'));
        assert!(matches!(r1, ParseResult::Incomplete));
        let r2 = parser.feed(KeyEvent::char('g'));
        assert!(matches!(r2, ParseResult::Complete(_)));
    }

    #[test]
    fn test_parser_feed_zz() {
        let mut parser = Parser::new();
        parser.feed(KeyEvent::char('z'));
        let result = parser.feed(KeyEvent::char('z'));
        assert!(matches!(result, ParseResult::Complete(_)));
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = Parser::new();
        parser.feed(KeyEvent::char('g'));
        parser.reset();
        // After reset, 'g' should start fresh incomplete sequence
        let result = parser.feed(KeyEvent::char('g'));
        assert!(matches!(result, ParseResult::Incomplete));
    }
}

mod key_sequence_tests {
    use super::*;

    #[test]
    fn test_key_sequence_new() {
        let ks = KeySequence::new();
        assert!(ks.is_empty());
    }

    #[test]
    fn test_key_sequence_push_and_keys() {
        let mut ks = KeySequence::new();
        ks.push(KeyEvent::char('a'));
        ks.push(KeyEvent::char('b'));
        assert_eq!(ks.keys().len(), 2);
    }

    #[test]
    fn test_key_sequence_clear() {
        let mut ks = KeySequence::new();
        ks.push(KeyEvent::Escape);
        ks.clear();
        assert!(ks.is_empty());
    }

    #[test]
    fn test_key_sequence_not_empty() {
        let mut ks = KeySequence::new();
        ks.push(KeyEvent::char('x'));
        assert!(!ks.is_empty());
    }
}

mod parse_result_tests {
    use super::*;

    #[test]
    fn test_parse_result_incomplete() {
        let r = ParseResult::Incomplete;
        assert!(matches!(r, ParseResult::Incomplete));
    }

    #[test]
    fn test_parse_result_complete() {
        let r = ParseResult::Complete("test".to_string());
        assert!(matches!(r, ParseResult::Complete(_)));
    }

    #[test]
    fn test_parse_result_no_match() {
        let r = ParseResult::NoMatch;
        assert!(matches!(r, ParseResult::NoMatch));
    }
}

mod mode_state_debug {
    use super::*;

    #[test]
    fn test_mode_state_debug() {
        let state = ModeState::new();
        let debug = format!("{:?}", state);
        assert!(debug.len() > 0);
    }
}
