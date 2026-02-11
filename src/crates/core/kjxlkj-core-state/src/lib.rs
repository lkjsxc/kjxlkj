use kjxlkj_core_mode::{Mode, NormalResolvedAction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorAction {
    NormalModeKey(char),
    InsertChar(char),
    Esc,
    Quit,
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyResult {
    pub resolved_action: String,
    pub mode_before: Mode,
    pub cursor_before: usize,
    pub cursor_after: usize,
    pub should_quit: bool,
}

#[derive(Debug, Clone)]
pub struct EditorState {
    mode: Mode,
    line: String,
    normal_cursor: usize,
    insert_cursor: usize,
}

impl EditorState {
    pub fn new(line: String, normal_cursor: usize) -> Self {
        let char_len = line.chars().count();
        let clamped_normal = if char_len == 0 {
            0
        } else {
            normal_cursor.min(char_len - 1)
        };
        Self {
            mode: Mode::Normal,
            line,
            normal_cursor: clamped_normal,
            insert_cursor: clamped_normal,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn cursor(&self) -> usize {
        match self.mode {
            Mode::Normal => self.normal_cursor,
            Mode::Insert => self.insert_cursor,
        }
    }

    pub fn apply(&mut self, action: EditorAction) -> ApplyResult {
        let mode_before = self.mode;
        let cursor_before = self.cursor();
        let mut should_quit = false;
        let resolved_action = match action {
            EditorAction::NormalModeKey(ch) if self.mode == Mode::Normal => {
                let (resolved, quit) = self.apply_normal_action(ch);
                should_quit = quit;
                resolved
            }
            EditorAction::InsertChar(ch) if self.mode == Mode::Insert => {
                self.insert_char(ch);
                "InsertChar".to_string()
            }
            EditorAction::Esc if self.mode == Mode::Insert => {
                self.exit_insert();
                "ExitInsert".to_string()
            }
            EditorAction::Quit => {
                should_quit = true;
                "Quit".to_string()
            }
            _ => "Ignore".to_string(),
        };
        ApplyResult {
            resolved_action,
            mode_before,
            cursor_before,
            cursor_after: self.cursor(),
            should_quit,
        }
    }

    fn apply_normal_action(&mut self, ch: char) -> (String, bool) {
        match kjxlkj_core_mode::resolve_normal_char(ch) {
            NormalResolvedAction::EnterInsertAtCursor => {
                self.mode = Mode::Insert;
                self.insert_cursor = self.normal_cursor;
                ("EnterInsertAtCursor".to_string(), false)
            }
            NormalResolvedAction::EnterInsertAfterCursor => {
                self.mode = Mode::Insert;
                self.insert_cursor = (self.normal_cursor + 1).min(self.line_char_len());
                ("EnterInsertAfterCursor".to_string(), false)
            }
            NormalResolvedAction::EnterInsertAtEol => {
                self.mode = Mode::Insert;
                self.insert_cursor = self.line_char_len();
                ("EnterInsertAtEol".to_string(), false)
            }
            NormalResolvedAction::Quit => ("Quit".to_string(), true),
            NormalResolvedAction::Ignore => ("Ignore".to_string(), false),
        }
    }

    fn insert_char(&mut self, ch: char) {
        let byte_idx = char_to_byte_index(&self.line, self.insert_cursor);
        self.line.insert(byte_idx, ch);
        self.insert_cursor += 1;
    }

    fn exit_insert(&mut self) {
        self.mode = Mode::Normal;
        let char_len = self.line_char_len();
        self.normal_cursor = if char_len == 0 {
            0
        } else {
            self.insert_cursor.min(char_len).saturating_sub(1)
        };
    }

    fn line_char_len(&self) -> usize {
        self.line.chars().count()
    }
}

fn char_to_byte_index(line: &str, char_idx: usize) -> usize {
    line.char_indices()
        .nth(char_idx)
        .map_or(line.len(), |(byte_idx, _)| byte_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_at_eol_enters_insert_after_last_grapheme() {
        let mut state = EditorState::new("abc".to_string(), 2);
        let result = state.apply(EditorAction::NormalModeKey('a'));
        assert_eq!(result.resolved_action, "EnterInsertAfterCursor");
        assert_eq!(state.cursor(), 3);
        assert_eq!(state.mode(), Mode::Insert);
    }

    #[test]
    fn i_at_eol_differs_from_a_at_eol() {
        let mut i_state = EditorState::new("abc".to_string(), 2);
        let mut a_state = EditorState::new("abc".to_string(), 2);
        i_state.apply(EditorAction::NormalModeKey('i'));
        a_state.apply(EditorAction::NormalModeKey('a'));
        assert_eq!(i_state.cursor(), 2);
        assert_eq!(a_state.cursor(), 3);
    }

    #[test]
    fn upper_a_enters_insert_at_true_eol() {
        let mut state = EditorState::new("abc".to_string(), 0);
        let result = state.apply(EditorAction::NormalModeKey('A'));
        assert_eq!(result.resolved_action, "EnterInsertAtEol");
        assert_eq!(state.cursor(), 3);
    }
}
