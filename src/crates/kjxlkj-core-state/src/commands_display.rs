//! Display commands: :marks, :registers, :jumps, :changes, :file.

use crate::EditorState;

/// Display marks (:marks).
pub(crate) fn dispatch_show_marks(state: &mut EditorState) {
    if state.marks.is_empty() { state.message = Some("No marks set".into()); return; }
    let mut entries: Vec<_> = state.marks.iter()
        .map(|(c, (bid, pos))| format!(" {} {:>5} {:>3}  buffer {}", c, pos.line + 1, pos.col, bid.0))
        .collect();
    entries.sort();
    state.message = Some(format!("mark line  col  file\n{}", entries.join("\n")));
}

/// Display registers (:registers/:reg).
pub(crate) fn dispatch_show_registers(state: &mut EditorState) {
    let display = state.registers.display();
    if display.is_empty() { state.message = Some("No registers".into()); }
    else { state.message = Some(display); }
}

/// Display jump list (:jumps).
pub(crate) fn dispatch_show_jumps(state: &mut EditorState) {
    if state.jump_list.is_empty() { state.message = Some("No jumps".into()); return; }
    let lines: Vec<String> = state.jump_list.iter().enumerate().rev().take(10)
        .map(|(i, (bid, pos))| {
            let marker = if i == state.jump_list_idx { ">" } else { " " };
            format!("{}{:>3} {:>5} {:>3}  buf {}", marker, i, pos.line + 1, pos.col, bid.0)
        }).collect();
    state.message = Some(format!("jump line  col file\n{}", lines.join("\n")));
}

/// Display change list (:changes).
pub(crate) fn dispatch_show_changes(state: &mut EditorState) {
    if state.change_list.is_empty() { state.message = Some("No changes".into()); return; }
    let lines: Vec<String> = state.change_list.iter().enumerate().rev().take(10)
        .map(|(i, (bid, pos))| {
            let marker = if i == state.change_list_idx { ">" } else { " " };
            format!("{}{:>3} {:>5} {:>3}  buf {}", marker, i, pos.line + 1, pos.col, bid.0)
        }).collect();
    state.message = Some(format!("change line  col\n{}", lines.join("\n")));
}

/// Display file info (:file / Ctrl-g).
pub(crate) fn dispatch_show_file_info(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = match state.windows.get(&wid) { Some(w) => w, None => return };
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get(&bid) {
        let name = buf.file_path.as_deref().unwrap_or("[No Name]");
        let modified = if buf.modified { "[+]" } else { "" };
        let lines = buf.text.line_count();
        let pct = if lines > 0 { (win.cursor_line + 1) * 100 / lines } else { 0 };
        state.message = Some(format!("\"{}\" {} {} lines --{}%--", name, modified, lines, pct));
    }
}

/// Display available digraphs (:digraphs).
pub(crate) fn dispatch_show_digraphs(state: &mut EditorState) {
    let mut entries = Vec::new();
    for &(c1, c2, result) in kjxlkj_core_types::DIGRAPH_TABLE {
        entries.push(format!("{}{} {}", c1, c2, result));
    }
    state.message = Some(entries.join("  "));
}
