//! Miscellaneous dispatch: find-char repeat, visual swap, select register,
//! increment/decrement, reselect visual, shell command, put register.

use crate::EditorState;
use kjxlkj_core_types::{FindCharKind, MotionKind, Position, Range};

/// Dispatch a find-char intent (f/t/F/T).
pub(crate) fn dispatch_find_char(state: &mut EditorState, c: char, kind: FindCharKind) {
    state.last_find_char = Some((c, kind));
    do_find_char(state, c, kind);
}

/// Repeat last find-char (;).
pub(crate) fn dispatch_repeat_find_char(state: &mut EditorState) {
    if let Some((c, kind)) = state.last_find_char { do_find_char(state, c, kind); }
}

/// Repeat last find-char reversed (,).
pub(crate) fn dispatch_repeat_find_char_reverse(state: &mut EditorState) {
    if let Some((c, kind)) = state.last_find_char {
        let rev = match kind {
            FindCharKind::Forward => FindCharKind::Backward,
            FindCharKind::Backward => FindCharKind::Forward,
            FindCharKind::TillForward => FindCharKind::TillBackward,
            FindCharKind::TillBackward => FindCharKind::TillForward,
        };
        do_find_char(state, c, rev);
    }
}

fn do_find_char(state: &mut EditorState, c: char, kind: FindCharKind) {
    let motion = match kind {
        FindCharKind::Forward => MotionKind::FindCharForward(c),
        FindCharKind::Backward => MotionKind::FindCharBackward(c),
        FindCharKind::TillForward => MotionKind::TillCharForward(c),
        FindCharKind::TillBackward => MotionKind::TillCharBackward(c),
    };
    crate::dispatch_navigation::dispatch_motion(state, motion, 1);
}

/// Swap cursor/anchor in visual mode (o).
pub(crate) fn dispatch_visual_swap_end(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = match state.windows.get_mut(&wid) { Some(w) => w, None => return };
    if let Some(anchor) = win.visual_anchor {
        let cursor = Position::new(win.cursor_line, win.cursor_col);
        win.visual_anchor = Some(cursor);
        win.cursor_line = anchor.line;
        win.cursor_col = anchor.col;
    }
}

/// Select a register for the next operation.
pub(crate) fn dispatch_select_register(state: &mut EditorState, reg: kjxlkj_core_types::RegisterName) {
    state.registers.select(reg);
}

/// Increment or decrement the number under cursor.
pub(crate) fn dispatch_increment_number(state: &mut EditorState, delta: i64) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (line, col) = (win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let text = buf.text.line_to_string(line);
        let chars: Vec<char> = text.chars().collect();
        let mut start = col;
        while start < chars.len() && !chars[start].is_ascii_digit() { start += 1; }
        if start >= chars.len() { return; }
        let negative = start > 0 && chars[start - 1] == '-';
        if negative { start -= 1; }
        let mut end = if negative { start + 1 } else { start };
        while end < chars.len() && chars[end].is_ascii_digit() { end += 1; }
        let num_str: String = chars[start..end].iter().collect();
        if let Ok(n) = num_str.parse::<i64>() {
            let new_n = n + delta;
            let new_str = new_n.to_string();
            buf.text.delete_range(Range::new(Position::new(line, start), Position::new(line, end)));
            buf.text.insert_text(Position::new(line, start), &new_str);
            buf.modified = true;
            let win = state.windows.get_mut(&wid).unwrap();
            win.cursor_col = start + new_str.len() - 1;
        }
    }
}

/// Reselect the last visual selection (gv).
pub(crate) fn dispatch_reselect_visual(state: &mut EditorState) {
    if let Some((anchor, cursor, mode)) = state.last_visual {
        if let Some(wid) = state.active_window {
            if let Some(win) = state.windows.get_mut(&wid) {
                win.visual_anchor = Some(anchor);
                win.cursor_line = cursor.line;
                win.cursor_col = cursor.col;
            }
        }
        state.mode.transition(mode);
        state.parser.reset();
    }
}

/// Execute a shell command and show output (:! cmd).
pub(crate) fn dispatch_shell_command(state: &mut EditorState, cmd: &str) {
    match std::process::Command::new("sh").arg("-c").arg(cmd)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let msg = if stderr.is_empty() {
                stdout.trim().to_string()
            } else {
                format!("{}\n{}", stdout.trim(), stderr.trim())
            };
            state.message = Some(if msg.is_empty() { "shell command completed".into() } else { msg });
        }
        Err(e) => { state.message = Some(format!("shell error: {}", e)); }
    }
}

/// Put register contents below or above current line (:put / :put!).
pub(crate) fn dispatch_put_register(state: &mut EditorState, before: bool) {
    let text = match state.registers.get(kjxlkj_core_types::RegisterName::Unnamed) {
        Some(entry) => entry.text.clone(),
        None => return,
    };
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let insert_line = if before { line } else { line + 1 };
        let pos = if insert_line >= buf.text.line_count() {
            let last = buf.text.line_count() - 1;
            let len = buf.text.line_len(last);
            buf.text.insert_text(Position::new(last, len), "\n");
            Position::new(last + 1, 0)
        } else {
            Position::new(insert_line, 0)
        };
        let content = if text.ends_with('\n') { text.clone() } else { format!("{}\n", text) };
        buf.text.insert_text(pos, &content);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_line = if before { line } else { line + 1 };
        win.cursor_col = 0;
    }
}
