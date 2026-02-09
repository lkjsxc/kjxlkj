/// Mode transitions and key dispatch for EditorState.
use kjxlkj_core_mode::ModeTransition;
use kjxlkj_core_types::{CommandKind, ContentSource, Key, Mode};
use kjxlkj_core_ui::{EditorSnapshot, TabSnapshot};

use crate::editor::EditorState;

impl EditorState {
    /// Process a key press.
    pub fn handle_key(&mut self, key: Key) {
        // If recording a macro, intercept `q` to stop recording.
        if self.is_recording() && Self::is_q_key(&key) {
            self.stop_recording();
            return;
        }
        // Record the key for macro playback (before dispatch).
        self.record_key(&key);

        // Pending prefix (m, g, z, ", ', `, q, @) bypasses transition.
        if matches!(self.mode, Mode::Normal) && self.dispatch.has_pending() {
            self.dispatch_in_mode(key);
            return;
        }
        let in_terminal = matches!(self.windows.focused().content, ContentSource::Terminal(_));
        let mt = kjxlkj_core_mode::transition::transition(&self.mode, &key, in_terminal);

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
                    version,
                    content,
                    cursor.line,
                    cursor.grapheme,
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
                let cursor = self.windows.focused().cursor;
                let bid = self.current_buffer_id().0 as usize;
                let mp = crate::marks::MarkPosition {
                    buffer_id: bid,
                    line: cursor.line,
                    col: cursor.grapheme,
                };
                self.marks.set_last_change(mp);
                self.marks.set_last_insert(mp);
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
            (Mode::Command(_), Mode::Normal) => {
                // Enter executes, Esc cancels.
                if matches!(key.code, kjxlkj_core_types::KeyCode::Enter) {
                    self.execute_cmdline();
                    return; // execute_cmdline sets mode
                }
                // Esc: cancel
                self.cmdline.close();
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
                if let Some(anchor) = self.visual_anchor {
                    let cursor = self.windows.focused().cursor;
                    let bid = self.current_buffer_id().0 as usize;
                    let (s, e) = if (anchor.line, anchor.grapheme) <= (cursor.line, cursor.grapheme)
                    {
                        (anchor, cursor)
                    } else {
                        (cursor, anchor)
                    };
                    let sm = crate::marks::MarkPosition {
                        buffer_id: bid,
                        line: s.line,
                        col: s.grapheme,
                    };
                    let em = crate::marks::MarkPosition {
                        buffer_id: bid,
                        line: e.line,
                        col: e.grapheme,
                    };
                    self.marks.set_visual_start(sm);
                    self.marks.set_visual_end(em);
                }
                self.visual_anchor = None;
            }
            (Mode::Visual(_), Mode::Visual(_)) => {
                // Switching visual sub-kind; keep anchor.
            }
            _ => {}
        }
        self.mode = new_mode;
    }
}
