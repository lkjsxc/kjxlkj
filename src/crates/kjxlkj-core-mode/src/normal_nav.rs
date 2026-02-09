//! Normal mode: navigation, search, and scroll keys.

use kjxlkj_core_types::{Action, Key, KeyCode, KeyModifiers, Motion, ScrollDirection};

use crate::normal::{FindCharPending, NormalModeState};

impl NormalModeState {
    /// Try to dispatch navigation/search/scroll keys.
    ///
    /// Returns `Some(Some(action))` when a direct action
    /// matched, `Some(None)` when a pending state was set,
    /// or `None` when the key is not a navigation key.
    pub(crate) fn try_dispatch_nav(&mut self, key: &Key, count: u32) -> Option<Option<Action>> {
        match (&key.code, key.modifiers) {
            (KeyCode::Char('n'), KeyModifiers::NONE) => Some(Some(Action::NextMatch)),
            (KeyCode::Char('N'), KeyModifiers::NONE) => Some(Some(Action::PrevMatch)),
            (KeyCode::Char('*'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::StarForward, 1)))
            }
            (KeyCode::Char('#'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::StarBackward, 1)))
            }
            (KeyCode::Char('%'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::MatchingBracket, 1)))
            }
            (KeyCode::Char('H'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::ScreenTop, 1)))
            }
            (KeyCode::Char('M'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::ScreenMiddle, 1)))
            }
            (KeyCode::Char('L'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::ScreenBottom, 1)))
            }
            (KeyCode::Char('('), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::SentenceBackward, count)))
            }
            (KeyCode::Char(')'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::SentenceForward, count)))
            }
            // Repeat char find: ;/,
            (KeyCode::Char(';'), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::RepeatFindForward, count)))
            }
            (KeyCode::Char(','), KeyModifiers::NONE) => {
                Some(Some(Action::MoveCursor(Motion::RepeatFindBackward, count)))
            }
            // Jump list: Ctrl-o / Ctrl-i
            (KeyCode::Char('o'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::MoveCursor(Motion::JumpListBackward, count)))
            }
            (KeyCode::Char('i'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::MoveCursor(Motion::JumpListForward, count)))
            }
            // f/F/t/T — set pending for next char
            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                self.find_char_pending = Some(FindCharPending::Forward);
                Some(None)
            }
            (KeyCode::Char('F'), KeyModifiers::NONE) => {
                self.find_char_pending = Some(FindCharPending::Backward);
                Some(None)
            }
            (KeyCode::Char('t'), KeyModifiers::NONE) => {
                self.find_char_pending = Some(FindCharPending::TillForward);
                Some(None)
            }
            (KeyCode::Char('T'), KeyModifiers::NONE) => {
                self.find_char_pending = Some(FindCharPending::TillBackward);
                Some(None)
            }
            // Scroll motions
            (KeyCode::Char('d'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::Scroll(ScrollDirection::HalfDown, count)))
            }
            (KeyCode::Char('u'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::Scroll(ScrollDirection::HalfUp, count)))
            }
            (KeyCode::Char('f'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::Scroll(ScrollDirection::PageDown, count)))
            }
            (KeyCode::Char('b'), m) if m.contains(KeyModifiers::CTRL) => {
                Some(Some(Action::Scroll(ScrollDirection::PageUp, count)))
            }
            _ => None,
        }
    }
}
