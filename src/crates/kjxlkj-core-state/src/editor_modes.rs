/// Mode transitions and key dispatch for EditorState.
use kjxlkj_core_mode::{KeyDispatchResult, ModeTransition};
use kjxlkj_core_types::{CommandKind, ContentSource, Key, Mode};
use kjxlkj_core_ui::{EditorSnapshot, TabSnapshot};

use crate::editor::EditorState;

impl EditorState {
    /// Process a key press.
    pub fn handle_key(&mut self, key: Key) {
        // Pending prefix (m, g, z, ", ', `) bypasses transition.
        if matches!(self.mode, Mode::Normal)
            && self.dispatch.has_pending()
        {
            self.dispatch_in_mode(key);
            return;
        }
        let in_terminal =
            matches!(self.windows.focused().content, ContentSource::Terminal(_));
        let mt = kjxlkj_core_mode::transition::transition(
            &self.mode, &key, in_terminal,
        );

        match mt {
            ModeTransition::To(new_mode) => {
                self.transition_mode(new_mode, &key);
            }
            ModeTransition::Stay => {
                self.dispatch_in_mode(key);
            }
        }
    }

    /// Build an immutable snapshot for rendering.
    pub fn snapshot(&mut self) -> EditorSnapshot {
        self.sequence += 1;
        let (cols, rows) = self.terminal_size;
        let content_rows = rows.saturating_sub(2);

        let win_snapshots = self.windows.snapshots(cols, content_rows);
        let tab = TabSnapshot {
            layout: self.windows.layout().clone(),
            windows: win_snapshots,
        };

        EditorSnapshot {
            sequence: self.sequence,
            tabs: vec![tab],
            active_tab: 0,
            buffers: self.buffers.all_snapshots(),
            mode: self.mode.clone(),
            cmdline: self.cmdline.snapshot(),
            notifications: self.notifications.drain(..).collect(),
            search: self.search.clone(),
            theme: self.theme.clone(),
            terminal_size: self.terminal_size,
            focused_window: self.windows.focused_id(),
        }
    }

    fn transition_mode(&mut self, new_mode: Mode, key: &Key) {
        match (&self.mode, &new_mode) {
            (Mode::Normal, Mode::Insert) => {
                let cursor = self.windows.focused().cursor;
                let version = self.buffers.current().version;
                let content = self.buffers.current().content.clone();
                self.buffers.current_mut().undo_tree.begin_group(
                    version, content, cursor.line, cursor.grapheme,
                );
                if let kjxlkj_core_types::KeyCode::Char(c) = &key.code {
                    match c {
                        'a' => self.move_cursor_right(1),
                        'A' => self.move_to_line_end_insert(),
                        'I' => self.move_to_first_non_blank(),
                        'o' => self.open_below_impl(),
                        'O' => self.open_above_impl(),
                        's' => self.delete_char_forward(),
                        'S' | 'C' => {}
                        _ => {}
                    }
                }
            }
            (Mode::Insert, Mode::Normal) => {
                self.buffers.current_mut().undo_tree.end_group();
                let cursor = &mut self.windows.focused_mut().cursor;
                if cursor.grapheme > 0 {
                    cursor.grapheme -= 1;
                }
            }
            (Mode::Normal, Mode::Command(kind)) => {
                let prefix = match kind {
                    CommandKind::Ex => ':',
                    CommandKind::Search => {
                        if self.search.forward {
                            '/'
                        } else {
                            '?'
                        }
                    }
                };
                self.cmdline.open(prefix);
            }
            (Mode::Normal, Mode::Visual(_)) => {
                let cursor = self.windows.focused().cursor;
                self.visual_anchor = Some(cursor);
            }
            (Mode::Normal, Mode::OperatorPending(_)) => {
                self.op_count = self.dispatch.take_count();
                self.motion_count = None;
                self.g_prefix = false;
            }
            (Mode::Visual(_), Mode::Normal) => {
                self.visual_anchor = None;
            }
            (Mode::Visual(_), Mode::Visual(_)) => {
                // Switching visual sub-kind; keep anchor.
            }
            _ => {}
        }
        self.mode = new_mode;
    }

    fn dispatch_in_mode(&mut self, key: Key) {
        match &self.mode {
            Mode::Normal => {
                let result = self.dispatch.dispatch(&key);
                if let KeyDispatchResult::Action(action) = result {
                    self.handle_action(action);
                }
            }
            Mode::Insert => match &key.code {
                kjxlkj_core_types::KeyCode::Char(c) => self.insert_char(*c),
                kjxlkj_core_types::KeyCode::Enter => self.insert_newline(),
                kjxlkj_core_types::KeyCode::Backspace => {
                    self.delete_char_backward();
                }
                kjxlkj_core_types::KeyCode::Delete => {
                    self.delete_char_forward();
                }
                kjxlkj_core_types::KeyCode::Tab => self.insert_text("    "),
                _ => {}
            },
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
                self.dispatch_visual(key, kind);
            }
            _ => {}
        }
    }

    fn dispatch_command_key(&mut self, key: Key) {
        match &key.code {
            kjxlkj_core_types::KeyCode::Char(c) => {
                if key
                    .modifiers
                    .contains(kjxlkj_core_types::Modifier::CTRL)
                {
                    match c {
                        'w' => self.cmdline.delete_word_backward(),
                        'u' => self.cmdline.delete_to_start(),
                        'b' => self.cmdline.move_home(),
                        'e' => self.cmdline.move_end(),
                        _ => {}
                    }
                } else {
                    self.cmdline.insert_char(*c);
                }
            }
            kjxlkj_core_types::KeyCode::Backspace => {
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
            kjxlkj_core_types::KeyCode::Right => self.cmdline.move_right(),
            kjxlkj_core_types::KeyCode::Home => self.cmdline.move_home(),
            kjxlkj_core_types::KeyCode::End => self.cmdline.move_end(),
            kjxlkj_core_types::KeyCode::Up => self.cmdline.history_prev(),
            kjxlkj_core_types::KeyCode::Down => self.cmdline.history_next(),
            _ => {}
        }
    }
}
