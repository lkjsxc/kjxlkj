//! Intent dispatch: process parsed intents against editor state.

use crate::EditorState;
use kjxlkj_core_edit::{apply_motion, apply_operator, compute_motion_range};
use kjxlkj_core_types::{
    Intent, InsertPosition, Mode, MotionKind, OperatorKind, Position, Range,
};

/// Process a single intent, mutating editor state.
pub fn dispatch_intent(state: &mut EditorState, intent: Intent) {
    match intent {
        Intent::Noop => {}
        Intent::Motion(kind, count) => dispatch_motion(state, kind, count),
        Intent::EnterMode(mode) => {
            state.mode.transition(mode);
            state.parser.reset();
        }
        Intent::EnterInsert(pos) => dispatch_enter_insert(state, pos),
        Intent::InsertChar(c) => dispatch_insert_char(state, c),
        Intent::InsertNewline => dispatch_insert_newline(state),
        Intent::DeleteCharBefore => dispatch_delete_char_before(state),
        Intent::DeleteCharAt => dispatch_delete_char_at(state),
        Intent::Operator(op, motion, count) => {
            dispatch_operator(state, op, motion, count);
        }
        Intent::LineOperator(op, count) => {
            dispatch_line_operator(state, op, count);
        }
        Intent::Undo => {
            state.message = Some("undo: not yet wired".into());
        }
        Intent::Redo => {
            state.message = Some("redo: not yet wired".into());
        }
        Intent::Paste(_, paste_pos) => dispatch_paste(state, paste_pos),
        Intent::YankLine(count) => dispatch_yank_line(state, count),
        Intent::DeleteToEnd => dispatch_delete_to_end(state),
        Intent::ChangeToEnd => {
            dispatch_delete_to_end(state);
            state.mode.transition(Mode::Insert);
        }
        Intent::OpenLine(below) => dispatch_open_line(state, below),
        Intent::JoinLines(spaces, count) => {
            dispatch_join_lines(state, spaces, count);
        }
        Intent::ReplaceChar(c) => dispatch_replace_char(state, c),
        Intent::Scroll(kind) => dispatch_scroll(state, kind),
        Intent::ExCommand(cmd) => dispatch_ex_command(state, &cmd),
        Intent::ToggleCase => dispatch_toggle_case(state),
        Intent::SubstituteChar => {
            dispatch_delete_char_at(state);
            state.mode.transition(Mode::Insert);
        }
        Intent::SubstituteLine => {
            dispatch_line_operator(state, OperatorKind::Change, 1);
        }
        Intent::ReplaceInsert(c) => dispatch_replace_insert(state, c),
        Intent::Indent(indent, count) => {
            dispatch_indent(state, indent, count);
        }
        _ => {} // Remaining intents stubbed
    }
}

fn dispatch_motion(state: &mut EditorState, kind: MotionKind, count: usize) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let bid = match state.windows.get(&wid) {
        Some(w) => w.buffer_id,
        None => return,
    };
    let buf = match state.buffers.get(&bid) {
        Some(b) => b,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let pos = Position::new(win.cursor_line, win.cursor_col);
    let new_pos = apply_motion(&buf.text, pos, kind, count);
    let win = state.windows.get_mut(&wid).unwrap();
    win.set_cursor(new_pos);
    win.ensure_cursor_visible();
}

fn dispatch_enter_insert(state: &mut EditorState, pos: InsertPosition) {
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            match pos {
                InsertPosition::AfterCursor => {
                    win.cursor_col += 1;
                }
                InsertPosition::EndOfLine => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        let line_len = buf.text.line_len(win.cursor_line);
                        win.cursor_col = line_len;
                    }
                }
                InsertPosition::FirstNonBlank => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        let line = buf.text.line_to_string(win.cursor_line);
                        let col = line
                            .chars()
                            .position(|c| !c.is_whitespace())
                            .unwrap_or(0);
                        win.cursor_col = col;
                    }
                }
                InsertPosition::BeforeCursor => {}
            }
        }
    }
    state.mode.transition(Mode::Insert);
}

fn dispatch_insert_char(state: &mut EditorState, c: char) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get(&wid) {
        Some(w) => w,
        None => return,
    };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_char(pos, c);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col += 1;
    }
}

fn dispatch_insert_newline(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get(&wid) {
        Some(w) => w,
        None => return,
    };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_char(pos, '\n');
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_line += 1;
        win.cursor_col = 0;
        win.ensure_cursor_visible();
    }
}

fn dispatch_delete_char_before(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if col == 0 && line == 0 {
        return;
    }
    if col > 0 {
        let range = Range::new(Position::new(line, col - 1), Position::new(line, col));
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.delete_range(range);
            buf.modified = true;
        }
        state.windows.get_mut(&wid).unwrap().cursor_col -= 1;
    } else {
        // Join with previous line
        if let Some(buf) = state.buffers.get(&bid) {
            let prev_len = buf.text.line_len(line - 1);
            let range = Range::new(
                Position::new(line - 1, prev_len),
                Position::new(line, 0),
            );
            if let Some(buf) = state.buffers.get_mut(&bid) {
                buf.text.delete_range(range);
                buf.modified = true;
            }
            let win = state.windows.get_mut(&wid).unwrap();
            win.cursor_line -= 1;
            win.cursor_col = prev_len;
        }
    }
}

fn dispatch_delete_char_at(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let range = Range::new(
        Position::new(win.cursor_line, win.cursor_col),
        Position::new(win.cursor_line, win.cursor_col + 1),
    );
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(range);
        buf.modified = true;
    }
}

fn dispatch_operator(
    state: &mut EditorState,
    op: OperatorKind,
    motion: MotionKind,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);

    if let Some(buf) = state.buffers.get_mut(&bid) {
        let range = compute_motion_range(&buf.text, pos, motion, count);
        let result = apply_operator(&mut buf.text, op, range, false);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            if op == OperatorKind::Yank {
                state.registers.yank(&reg.text, false);
            } else {
                state.registers.delete(&reg.text, false);
            }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert {
            state.mode.transition(Mode::Insert);
        }
        win.ensure_cursor_visible();
    }
}

fn dispatch_line_operator(
    state: &mut EditorState,
    op: OperatorKind,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;

    if let Some(buf) = state.buffers.get_mut(&bid) {
        let end_line = (line + count).min(buf.text.line_count()).saturating_sub(1);
        let range = Range::new(
            Position::new(line, 0),
            Position::new(end_line, buf.text.line_len(end_line)),
        );
        let result = apply_operator(&mut buf.text, op, range, true);
        buf.modified = true;

        if let Some(ref reg) = result.deleted_text {
            match op {
                OperatorKind::Yank => state.registers.yank(&reg.text, true),
                _ => state.registers.delete(&reg.text, true),
            }
        }

        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert {
            state.mode.transition(Mode::Insert);
        }
        win.ensure_cursor_visible();
    }
}

fn dispatch_undo(state: &mut EditorState) {
    state.message = Some("undo: not yet wired to undo tree".into());
}

fn dispatch_redo(state: &mut EditorState) {
    state.message = Some("redo: not yet wired to undo tree".into());
}

fn dispatch_paste(
    state: &mut EditorState,
    paste_pos: kjxlkj_core_types::PastePosition,
) {
    let text = match state.registers.unnamed_text() {
        Some(t) => t.to_string(),
        None => return,
    };
    let linewise = state.registers.unnamed_type()
        == Some(kjxlkj_core_types::RegisterType::Linewise);

    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;

    use kjxlkj_core_types::PastePosition;
    if linewise {
        let line = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => {
                win.cursor_line + 1
            }
            PastePosition::Before | PastePosition::BeforeCursorEnd => {
                win.cursor_line
            }
        };
        let pos = Position::new(line, 0);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            // Insert text as new lines
            let insert_text = if text.ends_with('\n') {
                text.clone()
            } else {
                format!("{}\n", text)
            };
            buf.text.insert_text(pos, &insert_text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_line = line;
        win.cursor_col = 0;
    } else {
        let col = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => {
                win.cursor_col + 1
            }
            PastePosition::Before | PastePosition::BeforeCursorEnd => {
                win.cursor_col
            }
        };
        let pos = Position::new(win.cursor_line, col);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.insert_text(pos, &text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_col = col + text.len().saturating_sub(1);
    }
}

fn dispatch_yank_line(state: &mut EditorState, count: usize) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    if let Some(buf) = state.buffers.get(&win.buffer_id) {
        let start = win.cursor_line;
        let end = (start + count).min(buf.text.line_count());
        let start_pos = Position::new(start, 0);
        let end_pos = if end >= buf.text.line_count() {
            let last = buf.text.line_count() - 1;
            Position::new(last, buf.text.line_len(last))
        } else {
            Position::new(end, 0)
        };
        let text = buf.text.text_in_range(start_pos, end_pos);
        state.registers.yank(&text, true);
    }
}

fn dispatch_delete_to_end(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let line_len = buf.text.line_len(line);
        if col < line_len {
            let start = Position::new(line, col);
            let end = Position::new(line, line_len);
            let text = buf.text.text_in_range(start, end);
            buf.text.delete_range(Range::new(start, end));
            buf.modified = true;
            state.registers.delete(&text, false);
        }
    }
    // Clamp cursor
    if let Some(win) = state.windows.get_mut(&wid) {
        if let Some(buf) = state.buffers.get(&bid) {
            let new_len = buf.text.line_len(win.cursor_line);
            if win.cursor_col >= new_len && new_len > 0 {
                win.cursor_col = new_len.saturating_sub(1);
            }
        }
    }
}

fn dispatch_open_line(state: &mut EditorState, below: bool) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;

    if below {
        if let Some(buf) = state.buffers.get_mut(&bid) {
            let insert_line = line + 1;
            let pos = if insert_line >= buf.text.line_count() {
                let last = buf.text.line_count() - 1;
                let len = buf.text.line_len(last);
                Position::new(last, len)
            } else {
                Position::new(insert_line, 0)
            };
            buf.text.insert_text(pos, "\n");
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_line = line + 1;
        win.cursor_col = 0;
    } else {
        let pos = Position::new(line, 0);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.insert_text(pos, "\n");
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_col = 0;
    }
    state.mode.transition(Mode::Insert);
}

fn dispatch_join_lines(state: &mut EditorState, spaces: bool, count: usize) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let mut line = win.cursor_line;

    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..count {
            if line + 1 >= buf.text.line_count() {
                break;
            }
            let curr_len = buf.text.line_len(line);
            // Delete the newline at end of current line
            let r = Range::new(Position::new(line, curr_len), Position::new(line + 1, 0));
            buf.text.delete_range(r);
            // Remove leading whitespace from joined line
            let joined = buf.text.line_to_string(line);
            let trimmed_start = joined[curr_len..]
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            if trimmed_start > 0 {
                buf.text.delete_range(Range::new(
                    Position::new(line, curr_len),
                    Position::new(line, curr_len + trimmed_start),
                ));
            }
            if spaces && curr_len > 0 {
                buf.text.insert_char(Position::new(line, curr_len), ' ');
            }
            buf.modified = true;
        }
    }
}

fn dispatch_replace_char(state: &mut EditorState, c: char) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    let end = Position::new(win.cursor_line, win.cursor_col + 1);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(Range::new(pos, end));
        buf.text.insert_char(pos, c);
        buf.modified = true;
    }
}

fn dispatch_replace_insert(state: &mut EditorState, c: char) {
    dispatch_replace_char(state, c);
    if let Some(win) = state.active_window_mut() {
        win.cursor_col += 1;
    }
}

fn dispatch_scroll(
    state: &mut EditorState,
    kind: kjxlkj_core_types::ScrollKind,
) {
    use kjxlkj_core_types::ScrollKind;
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get_mut(&wid).unwrap();
    let half = win.height / 2;
    match kind {
        ScrollKind::HalfPageDown => {
            win.cursor_line += half;
            win.top_line += half;
        }
        ScrollKind::HalfPageUp => {
            win.cursor_line = win.cursor_line.saturating_sub(half);
            win.top_line = win.top_line.saturating_sub(half);
        }
        ScrollKind::FullPageDown => {
            win.cursor_line += win.height;
            win.top_line += win.height;
        }
        ScrollKind::FullPageUp => {
            win.cursor_line = win.cursor_line.saturating_sub(win.height);
            win.top_line = win.top_line.saturating_sub(win.height);
        }
        ScrollKind::LineDown => {
            win.top_line += 1;
            if win.cursor_line < win.top_line {
                win.cursor_line = win.top_line;
            }
        }
        ScrollKind::LineUp => {
            win.top_line = win.top_line.saturating_sub(1);
            if win.cursor_line >= win.top_line + win.height {
                win.cursor_line = win.top_line + win.height - 1;
            }
        }
        ScrollKind::CursorCenter | ScrollKind::CursorCenterFirstNonBlank => {
            win.top_line = win.cursor_line.saturating_sub(half);
        }
        ScrollKind::CursorTop | ScrollKind::CursorTopFirstNonBlank => {
            win.top_line = win.cursor_line;
        }
        ScrollKind::CursorBottom | ScrollKind::CursorBottomFirstNonBlank => {
            win.top_line = win.cursor_line.saturating_sub(win.height.saturating_sub(1));
        }
    }
    // Clamp cursor to buffer
    if let Some(buf) = state.buffers.get(&win.buffer_id) {
        let max_line = buf.text.line_count().saturating_sub(1);
        if win.cursor_line > max_line {
            win.cursor_line = max_line;
        }
    }
}

fn dispatch_toggle_case(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        if let Some(c) = buf.text.char_at(pos) {
            if c.is_alphabetic() {
                let toggled: char = if c.is_uppercase() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c.to_uppercase().next().unwrap()
                };
                let end = Position::new(win.cursor_line, win.cursor_col + 1);
                buf.text.delete_range(Range::new(pos, end));
                buf.text.insert_char(pos, toggled);
                buf.modified = true;
            }
        }
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col += 1;
    }
}

fn dispatch_indent(state: &mut EditorState, indent: bool, count: usize) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;

    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..count {
            if indent {
                // Add tab/spaces at start of line
                let pos = Position::new(line, 0);
                buf.text.insert_text(pos, "    ");
            } else {
                // Remove up to 4 leading spaces
                let text = buf.text.line_to_string(line);
                let spaces: usize = text.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    buf.text.delete_range(Range::new(
                        Position::new(line, 0),
                        Position::new(line, spaces),
                    ));
                }
            }
        }
        buf.modified = true;
    }
}

fn dispatch_ex_command(state: &mut EditorState, cmd: &str) {
    match cmd.trim() {
        ":q" | ":quit" => {
            state.should_quit = true;
        }
        ":q!" | ":quit!" => {
            state.should_quit = true;
        }
        ":w" | ":write" => {
            state.message = Some("write: not yet implemented".into());
        }
        ":wq" | ":x" => {
            state.message = Some("write: not yet implemented".into());
            state.should_quit = true;
        }
        _ => {
            state.message = Some(format!("unknown command: {}", cmd));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;
    use kjxlkj_core_types::Size;

    fn setup_editor(text: &str) -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text(text);
        s.create_window(bid);
        s
    }

    #[test]
    fn insert_char() {
        let mut s = setup_editor("hello");
        s.mode.transition(Mode::Insert);
        dispatch_intent(&mut s, Intent::InsertChar('x'));
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('x'));
    }

    #[test]
    fn motion_down() {
        let mut s = setup_editor("line1\nline2\nline3");
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
        assert_eq!(s.cursor().line, 1);
    }

    #[test]
    fn dd_deletes_line() {
        let mut s = setup_editor("aaa\nbbb\nccc");
        dispatch_intent(
            &mut s,
            Intent::LineOperator(OperatorKind::Delete, 1),
        );
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('b'));
    }

    #[test]
    fn quit_command() {
        let mut s = setup_editor("hello");
        dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
        assert!(s.should_quit);
    }

    #[test]
    fn enter_insert_mode() {
        let mut s = setup_editor("hello");
        dispatch_intent(
            &mut s,
            Intent::EnterInsert(InsertPosition::BeforeCursor),
        );
        assert_eq!(s.current_mode(), Mode::Insert);
    }

    #[test]
    fn open_line_below() {
        let mut s = setup_editor("hello\nworld");
        dispatch_intent(&mut s, Intent::OpenLine(true));
        assert_eq!(s.current_mode(), Mode::Insert);
        assert_eq!(s.cursor().line, 1);
    }

    #[test]
    fn toggle_case() {
        let mut s = setup_editor("Hello");
        dispatch_intent(&mut s, Intent::ToggleCase);
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('h'));
    }

    #[test]
    fn scroll_half_page_down() {
        let mut s = setup_editor(
            &(0..50).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n"),
        );
        dispatch_intent(
            &mut s,
            Intent::Scroll(kjxlkj_core_types::ScrollKind::HalfPageDown),
        );
        let win = s.active_window_state().unwrap();
        assert!(win.cursor_line > 0);
    }
}
