//! Action dispatch for EditorState.
//!
//! Extracted from editor.rs to keep each file ≤ 200 lines.

use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{Action, ContentKind, Mode, Motion};

use crate::editor::EditorState;
use crate::search::SearchDirection;
use crate::search_util::word_at;

impl EditorState {
    /// Apply a typed action to editor state.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::InsertChar(c) => {
                self.insert_text.push(c);
                self.insert_char(c);
            }
            Action::DeleteCharForward => self.delete_char_forward(),
            Action::DeleteCharBackward => self.delete_char_backward(),
            Action::Motion(ref motion) => {
                match motion {
                    Motion::SearchNext => {
                        let d = self.search.direction;
                        self.jump_to_match(d);
                    }
                    Motion::SearchPrev => {
                        let d = match self.search.direction {
                            SearchDirection::Forward => SearchDirection::Backward,
                            SearchDirection::Backward => SearchDirection::Forward,
                        };
                        self.jump_to_match(d);
                    }
                    _ => self.apply_cursor_motion(motion),
                }
            }
            Action::Quit => self.quit_requested = true,
            Action::ForceQuit => self.quit_requested = true,
            Action::WriteQuit => self.quit_requested = true,
            Action::Resize(cols, rows) => self.terminal_size = (cols, rows),
            Action::AppendEndOfLine => self.cursor_to_eol(),
            Action::InsertFirstNonBlank => self.cursor_to_first_nonblank(),
            Action::OpenLineBelow => self.open_line_below(),
            Action::OpenLineAbove => self.open_line_above(),
            Action::SplitVertical => self.split_vertical(),
            Action::SplitHorizontal => self.split_horizontal(),
            Action::CloseWindow => self.close_window(),
            Action::ExitToNormal => {
                self.mode = Mode::Normal;
                let win = self.focused_window_mut();
                if win.cursor.col > 0 { win.cursor.col -= 1; }
            }
            Action::OperatorLine(op) => self.apply_operator_line(op),
            Action::OperatorMotion(op, motion, count) => {
                self.apply_operator_motion(op, motion, count);
            }
            Action::SubstituteChar => self.delete_char_forward(),
            Action::SubstituteLine => self.delete_current_line_content(),
            Action::ChangeToEnd => self.delete_to_eol(),
            Action::DeleteWordBackward => self.delete_word_backward(),
            Action::DeleteToLineStart => self.delete_to_line_start(),
            Action::DeleteToEnd => self.delete_to_eol(),
            Action::JoinLinesNoSpace => self.join_lines_no_space(),
            Action::PutAfter => self.put_after(),
            Action::PutBefore => self.put_before(),
            Action::ShowRegisters => {}
            Action::StarSearchForward => self.star_search(SearchDirection::Forward),
            Action::StarSearchBackward => self.star_search(SearchDirection::Backward),
            Action::ClearSearchHighlight => self.search.clear_highlight(),
            Action::GStarSearchForward => self.g_star_search(SearchDirection::Forward),
            Action::GStarSearchBackward => self.g_star_search(SearchDirection::Backward),
            Action::IncrementNumber => self.increment_number(),
            Action::DecrementNumber => self.decrement_number(),
            Action::SetOption(ref name, ref val) => self.apply_set_option(name, val),
            Action::VisualOperator(op) => self.apply_visual_operator(op),
            Action::VisualSwapAnchor => self.swap_visual_anchor(),
            Action::NextBuffer => self.next_buffer(),
            Action::PreviousBuffer => self.prev_buffer(),
            Action::SwitchBuffer(id) => self.switch_to_buffer(id),
            Action::DeleteBuffer => self.delete_buffer(),
            Action::OpenFile(ref path) => { let p = path.clone(); self.open_file(&p); }
            Action::SwitchAlternate => self.switch_alternate(),
            Action::FirstBuffer => self.first_buffer(),
            Action::LastBuffer => self.last_buffer(),
            Action::ListBuffers => {}
            Action::WindowOnly => self.window_only(),
            Action::FocusCycle => self.focus_cycle(),
            Action::FocusPrevious => self.focus.toggle_previous(),
            Action::FocusDirection(dir) => self.focus_direction(dir),
            Action::FocusTopLeft => self.focus_top_left(),
            Action::FocusBottomRight => self.focus_bottom_right(),
            Action::WindowEqualize => self.window_equalize(),
            Action::WindowResize(edge, delta) => self.window_resize(edge, delta),
            Action::WindowMaxHeight => self.window_max_height(),
            Action::WindowMaxWidth => self.window_max_width(),
            Action::OpenExplorer => self.open_explorer(),
            Action::CloseExplorer => self.close_explorer(),
            Action::OpenTerminal => self.open_terminal(),
            Action::FocusCycleReverse => self.focus_cycle_reverse(),
            Action::WindowMoveEdge(_) => {} // placeholder
            Action::WindowRotate(_) => {} // placeholder
            Action::WindowExchange => {} // placeholder
            Action::JumpOlder | Action::JumpNewer => self.navigate_jumplist(&action),
            Action::ChangeOlder | Action::ChangeNewer => self.navigate_changelist(&action),
            Action::SetMark(c) => self.set_mark_at_cursor(c),
            Action::GotoMarkLine(c) => self.goto_mark_line(c),
            Action::GotoMarkExact(c) => self.goto_mark_exact(c),
            Action::MacroRecordStart(c) => self.start_macro_recording(c),
            Action::MacroRecordStop => self.stop_macro_recording(),
            Action::MacroPlay(c) => self.play_macro(c),
            Action::FoldOpen => self.fold_open(),
            Action::FoldClose => self.fold_close(),
            Action::FoldToggle => self.fold_toggle(),
            Action::FoldOpenAll => self.fold_state.open_all(),
            Action::FoldCloseAll => self.fold_close_all(),
            Action::FoldReduce => self.fold_state.reduce(),
            Action::FoldMore => self.fold_state.more(),
            Action::FoldNext => self.fold_next(),
            Action::FoldPrev => self.fold_prev(),
            _ => {}
        }
    }

    fn apply_cursor_motion(&mut self, motion: &Motion) {
        let wid = self.focus.focused;
        let win = self.windows.get(&wid).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) =
                self.buffers.get(&buf_id)
            {
                let cur = win.cursor;
                let new_cur =
                    apply_motion(&cur, motion, buf);
                self.windows
                    .get_mut(&wid)
                    .unwrap()
                    .cursor = new_cur;
            }
        }
    }

    /// Search for word under cursor (* or #).
    fn star_search(&mut self, dir: SearchDirection) {
        let wid = self.focus.focused;
        let win = match self.windows.get(&wid) { Some(w) => w, None => return };
        let buf_id = match win.content { ContentKind::Buffer(id) => id, _ => return };
        let buf = match self.buffers.get(&buf_id) { Some(b) => b, None => return };
        let col = win.cursor.col;
        let line = match buf.line(win.cursor.line) { Some(l) => l, None => return };
        let word = match word_at(&line, col) { Some(w) => w, None => return };
        let display = format!(r"\<{}\>", word);
        let rust_pat = format!(r"\b{}\b", regex::escape(&word));
        if self.search.set_raw_pattern(&display, &rust_pat, dir).is_ok() {
            self.registers.set_readonly('/', display);
            self.jump_to_match(dir);
        }
    }

    fn apply_set_option(&mut self, name: &str, val: &str) {
        match name {
            "ignorecase" | "ic" => self.search.ignorecase = val == "true",
            "smartcase" | "scs" => self.search.smartcase = val == "true",
            "hlsearch" | "hls" => self.search.hlsearch = val == "true",
            _ => {}
        }
    }

    /// g* / g# search — partial match (no word boundaries).
    fn g_star_search(&mut self, dir: SearchDirection) {
        let wid = self.focus.focused;
        let win = match self.windows.get(&wid) { Some(w) => w, None => return };
        let buf_id = match win.content { ContentKind::Buffer(id) => id, _ => return };
        let buf = match self.buffers.get(&buf_id) { Some(b) => b, None => return };
        let col = win.cursor.col;
        let line = match buf.line(win.cursor.line) { Some(l) => l, None => return };
        let word = match word_at(&line, col) { Some(w) => w, None => return };
        let rust_pat = regex::escape(&word);
        if self.search.set_raw_pattern(&word, &rust_pat, dir).is_ok() {
            self.registers.set_readonly('/', word);
            self.jump_to_match(dir);
        }
    }
}
