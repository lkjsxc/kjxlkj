/// Mode transitions and key dispatch for EditorState.
use kjxlkj_core_mode::ModeTransition;
use kjxlkj_core_types::{CommandKind, ContentSource, Key, Mode};
use kjxlkj_core_ui::{EditorSnapshot, TabSnapshot};

use crate::editor::EditorState;

impl EditorState {
    /// Process a key press.
    pub fn handle_key(&mut self, key: Key) {
        // Intercept `q` to stop macro recording.
        if self.is_recording() && Self::is_q_key(&key) {
            self.stop_recording();
            return;
        }
        self.record_key(&key); // record for macro playback

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
        let mut win_snapshots = self.windows.snapshots(cols, content_rows);
        if let Mode::Visual(kind) = &self.mode {
            if let Some(anchor) = self.visual_anchor {
                let fid = self.windows.focused_id();
                if let Some(ws) = win_snapshots.get_mut(&fid) {
                    ws.visual_selection = Some(kjxlkj_core_ui::VisualSelection { anchor, cursor: ws.cursor, kind: *kind });
                }
            }
        }
        let tab = TabSnapshot { layout: self.windows.layout().clone(), windows: win_snapshots };
        let mut search = self.search.clone();
        search.highlight_ranges = self.compute_hlsearch();
        EditorSnapshot {
            sequence: self.sequence,
            tabs: vec![tab],
            active_tab: 0,
            buffers: self.buffers.all_snapshots(),
            mode: self.mode.clone(),
            cmdline: self.cmdline.snapshot(),
            notifications: self.notifications.drain(..).collect(),
            search,
            theme: self.theme.clone(),
            terminal_size: self.terminal_size,
            focused_window: self.windows.focused_id(),
            popup_menu: if self.cmdline.completion.candidates.is_empty() {
                None
            } else {
                let sel = self.cmdline.completion.index.unwrap_or(0);
                let max_vis = 10usize;
                let scroll = if sel >= max_vis { sel - max_vis + 1 } else { 0 };
                Some(kjxlkj_core_ui::PopupMenu {
                    items: self.cmdline.completion.candidates.clone(),
                    selected: self.cmdline.completion.index,
                    row: self.terminal_size.1.saturating_sub(3),
                    col: 1,
                    max_visible: max_vis,
                    scroll_offset: scroll,
                })
            },
        }
    }

    fn compute_hlsearch(&self) -> Vec<(usize, usize, usize)> {
        let hl = self.options.get_bool("hlsearch") && self.search.active;
        let op_pending = matches!(self.mode, kjxlkj_core_types::Mode::OperatorPending(_));
        let active = hl || (op_pending && self.search.pattern.is_some());
        let pat = match (&self.search.pattern, active) {
            (Some(p), true) if !p.is_empty() => p.clone(),
            _ => return Vec::new(),
        };
        let buf = match self.buffers.get(self.current_buffer_id()) {
            Some(b) => b,
            None => return Vec::new(),
        };
        let mut ranges = Vec::new();
        for line_idx in 0..buf.content.len_lines() {
            let line_s: String = buf.content.line(line_idx).chars().collect();
            let mut pos = 0;
            while let Some(m) = line_s[pos..].find(&*pat) {
                let start = pos + m;
                let end = start + pat.len();
                ranges.push((line_idx, start, end));
                pos = end.max(start + 1);
            }
        }
        ranges
    }

    #[rustfmt::skip]
    fn transition_mode(&mut self, new_mode: Mode, key: &Key) {
        match (&self.mode, &new_mode) {
            (Mode::Normal, Mode::Insert) => {
                let c = self.windows.focused().cursor;
                let ver = self.buffers.current().version;
                let cnt = self.buffers.current().content.clone();
                self.buffers.current_mut().undo_tree.begin_group(ver, cnt, c.line, c.grapheme);
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
                if let Some((sl, el, col, at_end)) = self.block_insert_pending.take() {
                    let text = self.last_inserted_text.clone();
                    let s = kjxlkj_core_types::CursorPosition::new(sl, col);
                    let e = kjxlkj_core_types::CursorPosition::new(el, col);
                    self.visual_block_insert(&text, s, e, at_end);
                }
                let cursor = self.windows.focused().cursor;
                let bid = self.current_buffer_id().0 as usize;
                let mp = crate::marks::MarkPosition::new(bid, cursor.line, cursor.grapheme);
                self.marks.set_last_change(mp);
                self.marks.set_last_insert(mp);
                self.push_changelist(cursor.line, cursor.grapheme);
                if !self.last_inserted_text.is_empty() {
                    let t = self.last_inserted_text.clone();
                    self.registers.set(
                        kjxlkj_core_edit::RegisterName::LastInserted,
                        kjxlkj_core_edit::Register::new(t, false),
                    );
                }
                let cursor = &mut self.windows.focused_mut().cursor;
                if cursor.grapheme > 0 { cursor.grapheme -= 1; }
            }
            (Mode::Normal, Mode::Command(kind)) => {
                let ch = match kind {
                    CommandKind::Ex => ':',
                    CommandKind::Search if self.search.forward => '/',
                    _ => '?',
                };
                self.cmdline.open(ch);
            }
            (Mode::Command(_), Mode::Normal) => {
                if matches!(key.code, kjxlkj_core_types::KeyCode::Enter) {
                    if let Some(idx) = self.cmdline.completion.index {
                        if !self.cmdline.completion.candidates.is_empty() {
                            self.cmdline.content = self.cmdline.completion.candidates[idx].clone();
                            self.cmdline.cursor_pos = self.cmdline.content.len();
                            self.cmdline.completion.clear();
                            self.mode = Mode::Command(CommandKind::Ex); return;
                        }
                    }
                    self.execute_cmdline(); return;
                }
                self.cmdline.close();
            }
            (Mode::Normal, Mode::Visual(_)) => { self.visual_anchor = Some(self.windows.focused().cursor); }
            (Mode::Normal, Mode::OperatorPending(_)) => {
                self.op_count = self.dispatch.take_count();
                self.motion_count = None;
                self.g_prefix = false;
            }
            (Mode::Visual(vk), Mode::Normal) => {
                if let Some(anchor) = self.visual_anchor {
                    let c = self.windows.focused().cursor;
                    self.last_visual = Some((anchor, c, *vk));
                    let bid = self.current_buffer_id().0 as usize;
                    let (s, e) = if (anchor.line, anchor.grapheme) <= (c.line, c.grapheme) { (anchor, c) } else { (c, anchor) };
                    let mk = |p: kjxlkj_core_types::CursorPosition| crate::marks::MarkPosition::new(bid, p.line, p.grapheme);
                    self.marks.set_visual_start(mk(s));
                    self.marks.set_visual_end(mk(e));
                }
                self.visual_anchor = None;
            }
            (Mode::Visual(_), Mode::Visual(_)) => {}
            _ => {}
        }
        self.mode = new_mode;
    }
}
