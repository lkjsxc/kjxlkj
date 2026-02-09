//! Tests for normal mode key processing.

#[cfg(test)]
mod tests {
    use crate::normal::NormalModeState;
    use kjxlkj_core_types::{
        Action, InsertPosition, Key, Motion, Operator,
    };

    #[test]
    fn count_accumulation() {
        let mut s = NormalModeState::new();
        assert!(
            s.process_key(&Key::char('3')).is_none()
        );
        let a = s.process_key(&Key::char('j'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(Motion::Down, 3))
        ));
    }

    #[test]
    fn insert_entry() {
        let mut s = NormalModeState::new();
        let a = s.process_key(&Key::char('i'));
        assert!(matches!(
            a,
            Some(Action::EnterInsert(
                InsertPosition::BeforeCursor
            ))
        ));
    }

    #[test]
    fn operator_pending() {
        let mut s = NormalModeState::new();
        let a = s.process_key(&Key::char('d'));
        assert!(matches!(
            a,
            Some(Action::EnterOperatorPending(
                Operator::Delete
            ))
        ));
    }

    #[test]
    fn gg_motion() {
        let mut s = NormalModeState::new();
        assert!(
            s.process_key(&Key::char('g')).is_none()
        );
        let a = s.process_key(&Key::char('g'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(
                Motion::GotoFirstLine,
                1
            ))
        ));
    }

    #[test]
    fn find_char_forward() {
        let mut s = NormalModeState::new();
        assert!(
            s.process_key(&Key::char('f')).is_none()
        );
        let a = s.process_key(&Key::char('x'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(
                Motion::FindCharForward('x'),
                1
            ))
        ));
    }

    #[test]
    fn ctrl_w_h_focuses_left() {
        let mut s = NormalModeState::new();
        let ctrl_w = Key::ctrl('w');
        assert!(s.process_key(&ctrl_w).is_none());
        let a = s.process_key(&Key::char('h'));
        assert!(matches!(
            a,
            Some(Action::FocusWindow(
                kjxlkj_core_types::Direction::Left
            ))
        ));
    }

    #[test]
    fn scroll_half_down() {
        let mut s = NormalModeState::new();
        let ctrl_d = Key::ctrl('d');
        let a = s.process_key(&ctrl_d);
        assert!(matches!(
            a,
            Some(Action::Scroll(
                kjxlkj_core_types::ScrollDirection::HalfDown,
                1
            ))
        ));
    }

    #[test]
    fn bracket_match_key() {
        let mut s = NormalModeState::new();
        let a = s.process_key(&Key::char('%'));
        assert!(matches!(
            a,
            Some(Action::MoveCursor(
                Motion::MatchingBracket,
                1
            ))
        ));
    }
}
