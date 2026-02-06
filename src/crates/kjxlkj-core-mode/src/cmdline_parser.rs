/// Command-line mode input parsing — entry, editing keys, history navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineAction {
    InsertChar(char), DeleteBack, DeleteWord, DeleteToStart,
    MoveLeft, MoveRight, MoveWordLeft, MoveWordRight, MoveToStart, MoveToEnd,
    HistoryPrev, HistoryNext, Complete, Execute, Cancel, PasteRegister(char),
}

/// State of the command-line buffer.
#[derive(Debug, Clone)]
pub struct CmdlineState {
    pub prefix: char, pub content: String,
    pub cursor: usize, pub history_index: Option<usize>,
}

impl CmdlineState {
    pub fn new(prefix: char) -> Self {
        Self { prefix, content: String::new(), cursor: 0, history_index: None }
    }
    pub fn display(&self) -> String { format!("{}{}", self.prefix, self.content) }

    /// Apply an editing action to the command line.
    pub fn apply(&mut self, action: &CmdlineAction) {
        match action {
            CmdlineAction::InsertChar(c) => {
                self.content.insert(self.cursor, *c);
                self.cursor += c.len_utf8();
            }
            CmdlineAction::DeleteBack => {
                if self.cursor > 0 {
                    let prev = self.prev_char_boundary();
                    self.content.drain(prev..self.cursor);
                    self.cursor = prev;
                }
            }
            CmdlineAction::DeleteWord => {
                let target = self.word_boundary_left();
                self.content.drain(target..self.cursor);
                self.cursor = target;
            }
            CmdlineAction::DeleteToStart => {
                self.content.drain(..self.cursor);
                self.cursor = 0;
            }
            CmdlineAction::MoveLeft => { if self.cursor > 0 { self.cursor = self.prev_char_boundary(); } }
            CmdlineAction::MoveRight => { if self.cursor < self.content.len() { self.cursor = self.next_char_boundary(); } }
            CmdlineAction::MoveWordLeft => { self.cursor = self.word_boundary_left(); }
            CmdlineAction::MoveWordRight => { self.cursor = self.word_boundary_right(); }
            CmdlineAction::MoveToStart => { self.cursor = 0; }
            CmdlineAction::MoveToEnd => { self.cursor = self.content.len(); }
            _ => {}
        }
    }

    fn prev_char_boundary(&self) -> usize {
        let mut pos = self.cursor.saturating_sub(1);
        while pos > 0 && !self.content.is_char_boundary(pos) {
            pos -= 1;
        }
        pos
    }

    fn next_char_boundary(&self) -> usize {
        let mut pos = self.cursor + 1;
        while pos < self.content.len() && !self.content.is_char_boundary(pos) {
            pos += 1;
        }
        pos
    }

    fn word_boundary_left(&self) -> usize {
        let bytes = self.content.as_bytes();
        let mut pos = self.cursor;
        while pos > 0 && bytes[pos - 1] == b' ' { pos -= 1; }
        while pos > 0 && bytes[pos - 1] != b' ' { pos -= 1; }
        pos
    }

    fn word_boundary_right(&self) -> usize {
        let bytes = self.content.as_bytes();
        let len = bytes.len();
        let mut pos = self.cursor;
        while pos < len && bytes[pos] != b' ' { pos += 1; }
        while pos < len && bytes[pos] == b' ' { pos += 1; }
        pos
    }
}

/// Map a key event to a command-line action.
pub fn map_cmdline_key(key: char, ctrl: bool, special: Option<&str>) -> Option<CmdlineAction> {
    if let Some(sp) = special {
        return match sp {
            "Left" => Some(CmdlineAction::MoveLeft), "Right" => Some(CmdlineAction::MoveRight),
            "Home" => Some(CmdlineAction::MoveToStart), "End" => Some(CmdlineAction::MoveToEnd),
            "Up" => Some(CmdlineAction::HistoryPrev), "Down" => Some(CmdlineAction::HistoryNext),
            "Tab" => Some(CmdlineAction::Complete),
            "Enter" | "Return" => Some(CmdlineAction::Execute),
            "Esc" => Some(CmdlineAction::Cancel), "Backspace" => Some(CmdlineAction::DeleteBack),
            _ => None,
        };
    }
    if ctrl {
        return match key {
            'h' => Some(CmdlineAction::DeleteBack), 'w' => Some(CmdlineAction::DeleteWord),
            'u' => Some(CmdlineAction::DeleteToStart), 'a' => Some(CmdlineAction::MoveToStart),
            'e' => Some(CmdlineAction::MoveToEnd), 'b' => Some(CmdlineAction::MoveWordLeft),
            'f' => Some(CmdlineAction::MoveWordRight), 'c' => Some(CmdlineAction::Cancel),
            'r' => Some(CmdlineAction::PasteRegister('"')),
            'p' => Some(CmdlineAction::HistoryPrev), 'n' => Some(CmdlineAction::HistoryNext),
            _ => None,
        };
    }
    if !key.is_control() { Some(CmdlineAction::InsertChar(key)) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state() {
        let s = CmdlineState::new(':');
        assert_eq!(s.display(), ":"); assert_eq!(s.cursor, 0);
    }

    #[test]
    fn insert_chars() {
        let mut s = CmdlineState::new(':');
        s.apply(&CmdlineAction::InsertChar('w'));
        s.apply(&CmdlineAction::InsertChar('q'));
        assert_eq!(s.content, "wq"); assert_eq!(s.cursor, 2);
    }

    #[test]
    fn delete_back() {
        let mut s = CmdlineState::new(':');
        s.apply(&CmdlineAction::InsertChar('a'));
        s.apply(&CmdlineAction::InsertChar('b'));
        s.apply(&CmdlineAction::DeleteBack);
        assert_eq!(s.content, "a");
    }

    #[test]
    fn move_left_right() {
        let mut s = CmdlineState::new(':');
        s.content = "abc".into();
        s.cursor = 3;
        s.apply(&CmdlineAction::MoveLeft);
        assert_eq!(s.cursor, 2);
        s.apply(&CmdlineAction::MoveRight);
        assert_eq!(s.cursor, 3);
    }

    #[test]
    fn delete_word() {
        let mut s = CmdlineState::new(':');
        s.content = "hello world".into();
        s.cursor = 11;
        s.apply(&CmdlineAction::DeleteWord);
        assert_eq!(s.content, "hello ");
    }

    #[test]
    fn delete_to_start() {
        let mut s = CmdlineState::new(':');
        s.content = "abcdef".into();
        s.cursor = 3;
        s.apply(&CmdlineAction::DeleteToStart);
        assert_eq!(s.content, "def");
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn map_special_keys() {
        assert_eq!(map_cmdline_key(' ', false, Some("Esc")), Some(CmdlineAction::Cancel));
        assert_eq!(map_cmdline_key(' ', false, Some("Enter")), Some(CmdlineAction::Execute));
        assert_eq!(map_cmdline_key(' ', false, Some("Tab")), Some(CmdlineAction::Complete));
    }

    #[test]
    fn map_ctrl_keys() {
        assert_eq!(map_cmdline_key('w', true, None), Some(CmdlineAction::DeleteWord));
        assert_eq!(map_cmdline_key('u', true, None), Some(CmdlineAction::DeleteToStart));
        assert_eq!(map_cmdline_key('c', true, None), Some(CmdlineAction::Cancel));
    }

    #[test]
    fn map_regular_char() {
        assert_eq!(map_cmdline_key('x', false, None), Some(CmdlineAction::InsertChar('x')));
    }
}
