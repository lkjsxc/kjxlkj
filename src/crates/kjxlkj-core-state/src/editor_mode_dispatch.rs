//! Mode-specific key dispatch routing.

use kjxlkj_core_mode::KeyDispatchResult;
use kjxlkj_core_types::{Key, Mode};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn dispatch_in_mode(&mut self, key: Key) {
        match &self.mode {
            Mode::Normal => {
                let result = self.dispatch.dispatch(&key);
                if let KeyDispatchResult::Action(action) = result {
                    self.handle_action(action);
                }
            }
            Mode::Insert => {
                if self.insert_register_pending {
                    if let kjxlkj_core_types::KeyCode::Char(c) = &key.code {
                        self.handle_insert_register(*c);
                    } else {
                        self.insert_register_pending = false;
                    }
                    return;
                }
                match &key.code {
                    kjxlkj_core_types::KeyCode::Char(c) => {
                        if key.modifiers.contains(kjxlkj_core_types::Modifier::CTRL) && *c == 'r' {
                            self.insert_register_prompt();
                        } else {
                            self.insert_char(*c);
                        }
                    }
                    kjxlkj_core_types::KeyCode::Enter => self.insert_newline(),
                    kjxlkj_core_types::KeyCode::Backspace => {
                        self.delete_char_backward();
                    }
                    kjxlkj_core_types::KeyCode::Delete => {
                        self.delete_char_forward();
                    }
                    kjxlkj_core_types::KeyCode::Tab => self.insert_text("    "),
                    _ => {}
                }
            }
            Mode::Command(_) => self.dispatch_command_key(key),
            Mode::Replace => {
                if let kjxlkj_core_types::KeyCode::Char(c) = &key.code {
                    self.replace_char(*c);
                    self.move_cursor_right(1);
                }
            }
            Mode::OperatorPending(op) => {
                let op = *op;
                self.dispatch_op_pending(key, op);
            }
            Mode::Visual(kind) => {
                let kind = *kind;
                if self.visual_replace_pending {
                    self.visual_replace_pending = false;
                    if let kjxlkj_core_types::KeyCode::Char(c) = &key.code {
                        self.visual_replace(*c, kind);
                    }
                    return;
                }
                self.dispatch_visual(key, kind);
            }
            _ => {}
        }
    }
}
