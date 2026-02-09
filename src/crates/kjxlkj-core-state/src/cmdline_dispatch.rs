//! Command-line mode key dispatch for EditorState.

use kjxlkj_core_types::{Key, Mode};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn dispatch_command_key(&mut self, key: Key) {
        match &key.code {
            kjxlkj_core_types::KeyCode::Char(c) => {
                if key.modifiers.contains(kjxlkj_core_types::Modifier::CTRL) {
                    match c {
                        'w' => self.cmdline.delete_word_backward(),
                        'u' => self.cmdline.delete_to_start(),
                        'b' => self.cmdline.move_home(),
                        'e' => self.cmdline.move_end(),
                        'n' => self.cmdline_complete_next(),
                        'p' => self.cmdline_complete_prev(),
                        _ => {}
                    }
                } else {
                    self.cmdline_reset_completion();
                    self.cmdline.insert_char(*c);
                }
            }
            kjxlkj_core_types::KeyCode::Backspace => {
                self.cmdline_reset_completion();
                self.cmdline.backspace();
                if self.cmdline.content.is_empty() {
                    self.cmdline.close();
                    self.mode = Mode::Normal;
                }
            }
            kjxlkj_core_types::KeyCode::Delete => {
                self.cmdline.delete_at_cursor();
            }
            kjxlkj_core_types::KeyCode::Left => self.cmdline.move_left(),
            kjxlkj_core_types::KeyCode::Right => {
                self.cmdline.move_right();
            }
            kjxlkj_core_types::KeyCode::Home => self.cmdline.move_home(),
            kjxlkj_core_types::KeyCode::End => self.cmdline.move_end(),
            kjxlkj_core_types::KeyCode::Up => {
                self.cmdline.history_prev();
            }
            kjxlkj_core_types::KeyCode::Down => {
                self.cmdline.history_next();
            }
            kjxlkj_core_types::KeyCode::Tab => {
                self.cmdline_complete_next();
            }
            kjxlkj_core_types::KeyCode::BackTab => {
                self.cmdline_complete_prev();
            }
            _ => {}
        }
        // Incremental search: update highlight while typing.
        if matches!(
            self.mode,
            Mode::Command(kjxlkj_core_types::CommandKind::Search)
        ) {
            self.update_incsearch();
        }
    }
}
