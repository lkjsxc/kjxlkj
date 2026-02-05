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


mod extra_api_tests {
    use super::*;
    use kjxlkj_core_types::Position;

    #[test]
    fn test_count_default() {
        let state = ModeState::new();
        assert_eq!(state.count(), 1);
        assert_eq!(state.raw_count(), None);
    }

    #[test]
    fn test_set_count() {
        let mut state = ModeState::new();
        state.set_count(5);
        assert_eq!(state.count(), 5);
        assert_eq!(state.raw_count(), Some(5));
    }

    #[test]
    fn test_accumulate_count() {
        let mut state = ModeState::new();
        state.accumulate_count('2');
        assert_eq!(state.count(), 2);
        state.accumulate_count('3');
        assert_eq!(state.count(), 23);
    }

    #[test]
    fn test_clear_count() {
        let mut state = ModeState::new();
        state.set_count(10);
        state.clear_count();
        assert_eq!(state.raw_count(), None);
        assert_eq!(state.count(), 1);
    }

    #[test]
    fn test_operator() {
        let mut state = ModeState::new();
        assert!(state.operator().is_none());
        state.set_operator('d');
        assert_eq!(state.operator(), Some('d'));
    }

    #[test]
    fn test_clear_operator() {
        let mut state = ModeState::new();
        state.set_operator('y');
        state.clear_operator();
        assert!(state.operator().is_none());
    }

    #[test]
    fn test_visual_anchor() {
        let mut state = ModeState::new();
        assert!(state.visual_anchor().is_none());
        state.set_visual_anchor(Position::new(5, 10));
        assert_eq!(state.visual_anchor(), Some(Position::new(5, 10)));
    }

    #[test]
    fn test_visual_anchor_cleared_on_non_visual_mode() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Visual);
        state.set_visual_anchor(Position::new(1, 2));
        state.set_mode(Mode::Normal);
        assert!(state.visual_anchor().is_none());
    }

    #[test]
    fn test_last_find() {
        let mut state = ModeState::new();
        assert!(state.last_find().is_none());
        state.set_last_find('x', true, false);
        assert_eq!(state.last_find(), Some(('x', true, false)));
    }

    #[test]
    fn test_register() {
        let mut state = ModeState::new();
        assert!(state.register().is_none());
        state.set_register('a');
        assert_eq!(state.register(), Some('a'));
    }

    #[test]
    fn test_clear_register() {
        let mut state = ModeState::new();
        state.set_register('b');
        state.clear_register();
        assert!(state.register().is_none());
    }

    #[test]
    fn test_command_line_push() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Command);
        state.command_line_push('w');
        state.command_line_push('q');
        assert_eq!(state.command_line(), "wq");
    }

    #[test]
    fn test_command_line_pop() {
        let mut state = ModeState::new();
        state.set_command_line("abc".to_string());
        let popped = state.command_line_pop();
        assert_eq!(popped, Some('c'));
        assert_eq!(state.command_line(), "ab");
    }

    #[test]
    fn test_command_line_clear() {
        let mut state = ModeState::new();
        state.set_command_line("hello".to_string());
        state.command_line_clear();
        assert_eq!(state.command_line(), "");
    }

    #[test]
    fn test_set_command_line() {
        let mut state = ModeState::new();
        state.set_command_line("write".to_string());
        assert_eq!(state.command_line(), "write");
    }

    #[test]
    fn test_reset_pending() {
        let mut state = ModeState::new();
        state.set_count(5);
        state.set_operator('d');
        state.set_register('a');
        state.reset_pending();
        assert!(state.raw_count().is_none());
        assert!(state.operator().is_none());
        assert!(state.register().is_none());
    }

    #[test]
    fn test_modestate_clone() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        state.set_count(3);
        let cloned = state.clone();
        assert_eq!(cloned.mode(), Mode::Insert);
        assert_eq!(cloned.count(), 3);
    }

    #[test]
    fn test_modestate_debug() {
        let state = ModeState::new();
        let debug = format!("{:?}", state);
        assert!(debug.contains("ModeState"));
    }

    #[test]
    fn test_pop_empty_command_line() {
        let mut state = ModeState::new();
        let popped = state.command_line_pop();
        assert_eq!(popped, None);
    }

    #[test]
    fn test_accumulate_count_from_zero() {
        let mut state = ModeState::new();
        state.accumulate_count('0');
        state.accumulate_count('5');
        assert_eq!(state.count(), 5);
    }

    #[test]
    fn test_large_count() {
        let mut state = ModeState::new();
        state.set_count(9999);
        state.accumulate_count('9');
        assert_eq!(state.count(), 99999);
    }
}
