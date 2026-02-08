//! Insert mode state: character insertion and escape handling.

use kjxlkj_core_types::{Action, Key, KeyCode, KeyModifiers};

/// State maintained during Insert mode.
#[derive(Debug, Default)]
pub struct InsertModeState {
    /// Characters inserted in this insert session (for dot repeat).
    pub inserted: Vec<char>,
    /// Whether we are in a Ctrl-O insert-normal sub-state.
    pub insert_normal: bool,
}

impl InsertModeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset state when entering insert mode.
    pub fn reset(&mut self) {
        self.inserted.clear();
        self.insert_normal = false;
    }

    /// Process a key event in Insert mode.
    pub fn process_key(&mut self, key: &Key) -> Action {
        match (&key.code, key.modifiers) {
            // Escape → back to Normal.
            (KeyCode::Esc, _) => {
                let action = Action::ReturnToNormal;
                action
            }

            // Ctrl-O → insert-normal (single Normal cmd).
            (KeyCode::Char('o'), m) if m.contains(KeyModifiers::CTRL) => {
                self.insert_normal = true;
                Action::InsertNormal
            }

            // Ctrl-C → also return to Normal (vim compat).
            (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::ReturnToNormal
            }

            // Backspace
            (KeyCode::Backspace, _) => {
                self.inserted.push('\x08');
                Action::CmdlineBackspace // repurpose: delete char backward
            }

            // Enter
            (KeyCode::Enter, _) => {
                self.inserted.push('\n');
                Action::InsertChar('\n')
            }

            // Tab
            (KeyCode::Tab, _) => {
                self.inserted.push('\t');
                Action::InsertChar('\t')
            }

            // Ctrl-W → delete word backward.
            (KeyCode::Char('w'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::DeleteCharBackward
            }

            // Ctrl-U → delete to start of line.
            (KeyCode::Char('u'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::ChangeToEnd // simplified: delete line content
            }

            // Ctrl-H → backspace alias.
            (KeyCode::Char('h'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::DeleteCharBackward
            }

            // Ctrl-R → paste from register (needs register char).
            (KeyCode::Char('r'), m) if m.contains(KeyModifiers::CTRL) => {
                Action::Put(true)
            }

            // Arrow keys for cursor movement in insert mode.
            (KeyCode::Left, _) => {
                Action::MoveCursor(kjxlkj_core_types::Motion::Left, 1)
            }
            (KeyCode::Right, _) => {
                Action::MoveCursor(kjxlkj_core_types::Motion::Right, 1)
            }
            (KeyCode::Up, _) => {
                Action::MoveCursor(kjxlkj_core_types::Motion::Up, 1)
            }
            (KeyCode::Down, _) => {
                Action::MoveCursor(kjxlkj_core_types::Motion::Down, 1)
            }

            // Regular character insertion.
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                self.inserted.push(*c);
                Action::InsertChar(*c)
            }

            _ => Action::Nop,
        }
    }

    /// Get a copy of the insert buffer for dot-repeat.
    pub fn get_inserted(&self) -> Vec<char> {
        self.inserted.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_returns_to_normal() {
        let mut s = InsertModeState::new();
        assert_eq!(s.process_key(&Key::esc()), Action::ReturnToNormal);
    }

    #[test]
    fn char_insertion() {
        let mut s = InsertModeState::new();
        assert_eq!(
            s.process_key(&Key::char('a')),
            Action::InsertChar('a')
        );
        assert_eq!(s.inserted, vec!['a']);
    }

    #[test]
    fn enter_inserts_newline() {
        let mut s = InsertModeState::new();
        assert_eq!(
            s.process_key(&Key::enter()),
            Action::InsertChar('\n')
        );
    }

    #[test]
    fn ctrl_o_insert_normal() {
        let mut s = InsertModeState::new();
        let action = s.process_key(&Key::ctrl('o'));
        assert_eq!(action, Action::InsertNormal);
        assert!(s.insert_normal);
    }
}
