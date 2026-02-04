//! Key event handler.

use kjxlkj_core::mode::{InputResult, NormalState, ParsedInput};
use kjxlkj_core::types::{Key, KeyCode, Mode};
use kjxlkj_core::EditorState;

use crate::command::execute_command;

/// Handle a key event.
pub async fn handle_key(editor: &mut EditorState, key: Key) {
    // Record key for macros.
    if editor.registers.is_recording() {
        editor.registers.record_key(key);
    }

    match editor.mode.mode {
        Mode::Normal => handle_normal(editor, key).await,
        Mode::Insert => handle_insert(editor, key),
        Mode::Replace => handle_replace(editor, key),
        Mode::Command => handle_command(editor, key).await,
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => handle_visual(editor, key),
    }
}

async fn handle_normal(editor: &mut EditorState, key: Key) {
    let result = NormalState::process_key(&mut editor.mode, key);

    match result {
        InputResult::Parsed(input) => {
            execute_parsed(editor, input).await;
            editor.mode.clear_pending();
        }
        InputResult::ModeChange(mode) => {
            if mode == Mode::Visual || mode == Mode::VisualLine || mode == Mode::VisualBlock {
                start_visual(editor, mode);
            }
        }
        InputResult::Pending => {}
        InputResult::Handled | InputResult::Unhandled => {
            editor.mode.clear_pending();
        }
    }
}

fn handle_insert(editor: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Esc => {
            // Move cursor back one if possible.
            if editor.buffer.cursor.col > 0 {
                editor.buffer.cursor.col -= 1;
            }
            editor.mode.set_mode(Mode::Normal);
        }
        KeyCode::Char(c) if key.mods.ctrl => {
            handle_insert_ctrl(editor, c);
        }
        KeyCode::Char(c) => {
            insert_char(editor, c);
        }
        KeyCode::Backspace => {
            delete_char_before(editor);
        }
        KeyCode::Enter => {
            insert_newline(editor);
        }
        KeyCode::Left => {
            if editor.buffer.cursor.col > 0 {
                editor.buffer.cursor.col -= 1;
            }
        }
        KeyCode::Right => {
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            if editor.buffer.cursor.col < max {
                editor.buffer.cursor.col += 1;
            }
        }
        KeyCode::Up => {
            if editor.buffer.cursor.line > 0 {
                editor.buffer.cursor.line -= 1;
                editor.buffer.clamp_cursor_insert();
            }
        }
        KeyCode::Down => {
            if editor.buffer.cursor.line + 1 < editor.buffer.text.len_lines() {
                editor.buffer.cursor.line += 1;
                editor.buffer.clamp_cursor_insert();
            }
        }
        _ => {}
    }
}

fn handle_insert_ctrl(editor: &mut EditorState, c: char) {
    match c {
        'h' => delete_char_before(editor),
        'j' | 'm' => insert_newline(editor),
        'w' => delete_word_before(editor),
        'u' => delete_to_line_start(editor),
        _ => {}
    }
}

fn handle_replace(editor: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Esc => {
            editor.mode.set_mode(Mode::Normal);
        }
        KeyCode::Char(c) => {
            replace_char_at_cursor(editor, c);
            // Move cursor right.
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            if editor.buffer.cursor.col < max {
                editor.buffer.cursor.col += 1;
            }
        }
        KeyCode::Backspace => {
            if editor.buffer.cursor.col > 0 {
                editor.buffer.cursor.col -= 1;
            }
        }
        _ => {}
    }
}

async fn handle_command(editor: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Esc => {
            editor.command.clear();
            editor.mode.set_mode(Mode::Normal);
        }
        KeyCode::Enter => {
            let cmd = editor.command.submit();
            editor.mode.set_mode(Mode::Normal);
            execute_command(editor, &cmd).await;
        }
        KeyCode::Char(c) => {
            editor.command.insert(c);
        }
        KeyCode::Backspace => {
            if editor.command.cursor == 0 {
                editor.mode.set_mode(Mode::Normal);
            } else {
                editor.command.backspace();
            }
        }
        KeyCode::Left => {
            editor.command.move_left();
        }
        KeyCode::Right => {
            editor.command.move_right();
        }
        KeyCode::Up => {
            editor.command.history_prev();
        }
        KeyCode::Down => {
            editor.command.history_next();
        }
        _ => {}
    }
}

fn handle_visual(editor: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Esc => {
            editor.selection = None;
            editor.mode.set_mode(Mode::Normal);
        }
        KeyCode::Char('o') => {
            // Swap anchor and cursor.
            if let Some(ref mut sel) = editor.selection {
                sel.swap();
                editor.buffer.cursor = sel.cursor;
            }
        }
        KeyCode::Char('d') | KeyCode::Char('x') => {
            delete_selection(editor);
        }
        KeyCode::Char('y') => {
            yank_selection(editor);
        }
        KeyCode::Char('c') | KeyCode::Char('s') => {
            delete_selection(editor);
            editor.mode.set_mode(Mode::Insert);
        }
        _ => {
            // Try to handle as motion.
            let result = NormalState::process_key(&mut editor.mode, key);
            if let InputResult::Parsed(ParsedInput::Motion(motion)) = result {
                apply_visual_motion(editor, motion);
            }
            editor.mode.clear_pending();
        }
    }
}

fn start_visual(editor: &mut EditorState, mode: Mode) {
    use kjxlkj_core::types::{Selection, SelectionKind};

    let kind = match mode {
        Mode::Visual => SelectionKind::Char,
        Mode::VisualLine => SelectionKind::Line,
        Mode::VisualBlock => SelectionKind::Block,
        _ => SelectionKind::Char,
    };

    editor.selection = Some(Selection::new(
        editor.buffer.cursor,
        editor.buffer.cursor,
        kind,
    ));
}

fn apply_visual_motion(editor: &mut EditorState, motion: kjxlkj_core::edit::Motion) {
    // Apply motion to cursor.
    apply_motion(editor, &motion);

    // Update selection cursor.
    if let Some(ref mut sel) = editor.selection {
        sel.cursor = editor.buffer.cursor;
    }
}

async fn execute_parsed(editor: &mut EditorState, input: ParsedInput) {
    match input {
        ParsedInput::Motion(motion) => {
            apply_motion(editor, &motion);
        }
        ParsedInput::InsertAfter => {
            // Move cursor right by one (unless at end of line content).
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            if editor.buffer.cursor.col < max {
                editor.buffer.cursor.col += 1;
            }
            editor.mode.set_mode(Mode::Insert);
        }
        ParsedInput::InsertAtEnd => {
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            editor.buffer.cursor.col = max;
            editor.mode.set_mode(Mode::Insert);
        }
        ParsedInput::InsertAtFirstNonBlank => {
            move_to_first_non_blank(editor);
            editor.mode.set_mode(Mode::Insert);
        }
        ParsedInput::OpenBelow => {
            open_line_below(editor);
        }
        ParsedInput::OpenAbove => {
            open_line_above(editor);
        }
        ParsedInput::DeleteChar(count) => {
            for _ in 0..count {
                delete_char_at_cursor(editor);
            }
        }
        ParsedInput::DeleteCharBefore(count) => {
            for _ in 0..count {
                delete_char_before(editor);
            }
        }
        ParsedInput::DeleteToEnd => {
            delete_to_end_of_line(editor);
        }
        ParsedInput::ChangeToEnd => {
            delete_to_end_of_line(editor);
            editor.mode.set_mode(Mode::Insert);
        }
        ParsedInput::YankLine(count) => {
            yank_lines(editor, count);
        }
        ParsedInput::PasteAfter(count) => {
            paste_after(editor, count);
        }
        ParsedInput::PasteBefore(count) => {
            paste_before(editor, count);
        }
        ParsedInput::Undo(count) => {
            for _ in 0..count {
                undo(editor);
            }
        }
        ParsedInput::Redo(count) => {
            for _ in 0..count {
                redo(editor);
            }
        }
        ParsedInput::OperatorLine(op, count) => {
            execute_operator_line(editor, op, count);
        }
        ParsedInput::OperatorMotion(op, motion) => {
            execute_operator_motion(editor, op, motion);
        }
        ParsedInput::SearchForward => {
            editor.search_forward = true;
            editor.mode.set_mode(Mode::Command);
            editor.command.insert('/');
        }
        ParsedInput::SearchBackward => {
            editor.search_forward = false;
            editor.mode.set_mode(Mode::Command);
            editor.command.insert('?');
        }
        ParsedInput::WriteQuit => {
            write_buffer(editor).await;
            editor.should_quit = true;
        }
        ParsedInput::QuitNoSave => {
            editor.should_quit = true;
        }
        // Many more cases would be handled here...
        _ => {
            editor.set_message("Command not yet implemented");
        }
    }
}

fn apply_motion(editor: &mut EditorState, motion: &kjxlkj_core::edit::Motion) {
    use kjxlkj_core::edit::MotionKind;

    let count = motion.count;

    match motion.kind {
        MotionKind::Left => {
            editor.buffer.cursor.col = editor.buffer.cursor.col.saturating_sub(count);
        }
        MotionKind::Right => {
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            editor.buffer.cursor.col =
                (editor.buffer.cursor.col + count).min(max.saturating_sub(1));
        }
        MotionKind::Up => {
            editor.buffer.cursor.line = editor.buffer.cursor.line.saturating_sub(count);
            editor.buffer.clamp_cursor();
        }
        MotionKind::Down => {
            let max = editor.buffer.text.len_lines().saturating_sub(1);
            editor.buffer.cursor.line = (editor.buffer.cursor.line + count).min(max);
            editor.buffer.clamp_cursor();
        }
        MotionKind::LineStart => {
            editor.buffer.cursor.col = 0;
        }
        MotionKind::LineEnd => {
            let max = editor
                .buffer
                .text
                .line_grapheme_count(editor.buffer.cursor.line);
            editor.buffer.cursor.col = max.saturating_sub(1);
        }
        MotionKind::FirstNonBlank => {
            move_to_first_non_blank(editor);
        }
        MotionKind::FileStart => {
            editor.buffer.cursor.line = 0;
            editor.buffer.cursor.col = 0;
        }
        MotionKind::FileEnd => {
            editor.buffer.cursor.line = editor.buffer.text.len_lines().saturating_sub(1);
            editor.buffer.clamp_cursor();
        }
        MotionKind::GotoLine(n) => {
            let max = editor.buffer.text.len_lines();
            editor.buffer.cursor.line = n.saturating_sub(1).min(max.saturating_sub(1));
            editor.buffer.clamp_cursor();
        }
        MotionKind::WordStart => {
            for _ in 0..count {
                move_word_forward(editor);
            }
        }
        MotionKind::WordBack => {
            for _ in 0..count {
                move_word_backward(editor);
            }
        }
        MotionKind::WordEnd => {
            for _ in 0..count {
                move_word_end(editor);
            }
        }
        MotionKind::NextLineFirstNonBlank => {
            for _ in 0..count {
                if editor.buffer.cursor.line + 1 < editor.buffer.text.len_lines() {
                    editor.buffer.cursor.line += 1;
                }
            }
            move_to_first_non_blank(editor);
        }
        MotionKind::PrevLineFirstNonBlank => {
            for _ in 0..count {
                editor.buffer.cursor.line = editor.buffer.cursor.line.saturating_sub(1);
            }
            move_to_first_non_blank(editor);
        }
        MotionKind::ParagraphForward => {
            for _ in 0..count {
                move_paragraph_forward(editor);
            }
        }
        MotionKind::ParagraphBack => {
            for _ in 0..count {
                move_paragraph_backward(editor);
            }
        }
        _ => {}
    }
}

// Helper functions.

fn move_to_first_non_blank(editor: &mut EditorState) {
    if let Some(line) = editor.buffer.text.line_content(editor.buffer.cursor.line) {
        let first_non_blank = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
        editor.buffer.cursor.col = first_non_blank;
    }
}

fn move_word_forward(editor: &mut EditorState) {
    if let Some(line) = editor.buffer.text.line_content(editor.buffer.cursor.line) {
        let chars: Vec<char> = line.chars().collect();
        let mut col = editor.buffer.cursor.col;

        // Skip current word.
        while col < chars.len() && !chars[col].is_whitespace() {
            col += 1;
        }
        // Skip whitespace.
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }

        if col >= chars.len() && editor.buffer.cursor.line + 1 < editor.buffer.text.len_lines() {
            editor.buffer.cursor.line += 1;
            editor.buffer.cursor.col = 0;
            move_to_first_non_blank(editor);
        } else {
            editor.buffer.cursor.col = col.min(chars.len().saturating_sub(1));
        }
    }
}

fn move_word_backward(editor: &mut EditorState) {
    if let Some(line) = editor.buffer.text.line_content(editor.buffer.cursor.line) {
        let chars: Vec<char> = line.chars().collect();
        let mut col = editor.buffer.cursor.col;

        if col == 0 {
            if editor.buffer.cursor.line > 0 {
                editor.buffer.cursor.line -= 1;
                let new_len = editor
                    .buffer
                    .text
                    .line_grapheme_count(editor.buffer.cursor.line);
                editor.buffer.cursor.col = new_len.saturating_sub(1);
            }
            return;
        }

        // Move back one.
        col = col.saturating_sub(1);

        // Skip whitespace.
        while col > 0 && chars[col].is_whitespace() {
            col -= 1;
        }
        // Find start of word.
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        editor.buffer.cursor.col = col;
    }
}

fn move_word_end(editor: &mut EditorState) {
    if let Some(line) = editor.buffer.text.line_content(editor.buffer.cursor.line) {
        let chars: Vec<char> = line.chars().collect();
        let mut col = editor.buffer.cursor.col;

        if col + 1 < chars.len() {
            col += 1;
        }

        // Skip whitespace.
        while col < chars.len() && chars[col].is_whitespace() {
            col += 1;
        }
        // Find end of word.
        while col + 1 < chars.len() && !chars[col + 1].is_whitespace() {
            col += 1;
        }

        editor.buffer.cursor.col = col.min(chars.len().saturating_sub(1));
    }
}

fn move_paragraph_forward(editor: &mut EditorState) {
    let total = editor.buffer.text.len_lines();
    let mut line = editor.buffer.cursor.line;

    // Skip current non-empty lines.
    while line < total {
        if let Some(content) = editor.buffer.text.line_content(line) {
            if content.trim().is_empty() {
                break;
            }
        }
        line += 1;
    }
    // Skip empty lines.
    while line < total {
        if let Some(content) = editor.buffer.text.line_content(line) {
            if !content.trim().is_empty() {
                break;
            }
        }
        line += 1;
    }

    editor.buffer.cursor.line = line.min(total.saturating_sub(1));
    editor.buffer.cursor.col = 0;
}

fn move_paragraph_backward(editor: &mut EditorState) {
    let mut line = editor.buffer.cursor.line;

    if line == 0 {
        return;
    }
    line -= 1;

    // Skip current non-empty lines.
    while line > 0 {
        if let Some(content) = editor.buffer.text.line_content(line) {
            if content.trim().is_empty() {
                break;
            }
        }
        line -= 1;
    }
    // Skip empty lines.
    while line > 0 {
        if let Some(content) = editor.buffer.text.line_content(line) {
            if !content.trim().is_empty() {
                break;
            }
        }
        line -= 1;
    }

    editor.buffer.cursor.line = line;
    editor.buffer.cursor.col = 0;
}

fn insert_char(editor: &mut EditorState, c: char) {
    let cursor = editor.buffer.cursor;
    editor.buffer.text.insert_at_cursor(cursor, &c.to_string());
    editor.buffer.cursor.col += 1;
    editor.buffer.modified = true;
}

fn insert_newline(editor: &mut EditorState) {
    let cursor = editor.buffer.cursor;
    editor.buffer.text.insert_at_cursor(cursor, "\n");
    editor.buffer.cursor.line += 1;
    editor.buffer.cursor.col = 0;
    editor.buffer.modified = true;
}

fn delete_char_before(editor: &mut EditorState) {
    if editor.buffer.cursor.col > 0 {
        editor.buffer.cursor.col -= 1;
        delete_char_at_cursor(editor);
    } else if editor.buffer.cursor.line > 0 {
        // Join with previous line.
        let prev_line = editor.buffer.cursor.line - 1;
        let prev_len = editor.buffer.text.line_grapheme_count(prev_line);
        editor.buffer.cursor.line = prev_line;
        editor.buffer.cursor.col = prev_len;
        // Delete the newline.
        if let Some(byte) = editor.buffer.text.cursor_to_byte(editor.buffer.cursor) {
            editor.buffer.text.delete_range(byte, byte + 1);
            editor.buffer.modified = true;
        }
    }
}

fn delete_char_at_cursor(editor: &mut EditorState) {
    let cursor = editor.buffer.cursor;
    if let Some(byte) = editor.buffer.text.cursor_to_byte(cursor) {
        let line_len = editor.buffer.text.line_grapheme_count(cursor.line);
        if cursor.col < line_len {
            // Get char length at cursor.
            if let Some(line) = editor.buffer.text.line_content(cursor.line) {
                if let Some(c) = line.chars().nth(cursor.col) {
                    editor.buffer.text.delete_range(byte, byte + c.len_utf8());
                    editor.buffer.modified = true;
                }
            }
        }
    }
}

fn delete_word_before(editor: &mut EditorState) {
    if let Some(line) = editor.buffer.text.line_content(editor.buffer.cursor.line) {
        let chars: Vec<char> = line.chars().collect();
        let mut col = editor.buffer.cursor.col;

        if col == 0 {
            return;
        }

        let start_col = col;
        col -= 1;

        // Skip whitespace.
        while col > 0 && chars[col].is_whitespace() {
            col -= 1;
        }
        // Skip word characters.
        while col > 0 && !chars[col - 1].is_whitespace() {
            col -= 1;
        }

        // Delete from col to start_col.
        if let (Some(start_byte), Some(end_byte)) = (
            editor
                .buffer
                .text
                .cursor_to_byte(kjxlkj_core::Cursor::new(editor.buffer.cursor.line, col)),
            editor.buffer.text.cursor_to_byte(kjxlkj_core::Cursor::new(
                editor.buffer.cursor.line,
                start_col,
            )),
        ) {
            editor.buffer.text.delete_range(start_byte, end_byte);
            editor.buffer.cursor.col = col;
            editor.buffer.modified = true;
        }
    }
}

fn delete_to_line_start(editor: &mut EditorState) {
    let cursor = editor.buffer.cursor;
    if cursor.col > 0 {
        if let (Some(start_byte), Some(end_byte)) = (
            editor
                .buffer
                .text
                .cursor_to_byte(kjxlkj_core::Cursor::new(cursor.line, 0)),
            editor.buffer.text.cursor_to_byte(cursor),
        ) {
            editor.buffer.text.delete_range(start_byte, end_byte);
            editor.buffer.cursor.col = 0;
            editor.buffer.modified = true;
        }
    }
}

fn replace_char_at_cursor(editor: &mut EditorState, c: char) {
    let cursor = editor.buffer.cursor;
    let line_len = editor.buffer.text.line_grapheme_count(cursor.line);

    if cursor.col < line_len {
        delete_char_at_cursor(editor);
    }
    insert_char(editor, c);
    // Move cursor back since insert moved it forward.
    if cursor.col < line_len {
        editor.buffer.cursor.col -= 1;
    }
}

fn open_line_below(editor: &mut EditorState) {
    let line = editor.buffer.cursor.line;
    let line_len = editor.buffer.text.line_grapheme_count(line);
    editor.buffer.cursor.col = line_len;
    insert_newline(editor);
    editor.mode.set_mode(Mode::Insert);
}

fn open_line_above(editor: &mut EditorState) {
    let line = editor.buffer.cursor.line;
    editor.buffer.cursor.col = 0;

    if let Some(byte) = editor.buffer.text.cursor_to_byte(editor.buffer.cursor) {
        editor.buffer.text.insert(byte, "\n");
        editor.buffer.cursor.line = line;
        editor.buffer.cursor.col = 0;
        editor.buffer.modified = true;
    }

    editor.mode.set_mode(Mode::Insert);
}

fn delete_to_end_of_line(editor: &mut EditorState) {
    let cursor = editor.buffer.cursor;
    let line_len = editor.buffer.text.line_grapheme_count(cursor.line);

    if cursor.col < line_len {
        if let (Some(start_byte), Some(end_byte)) = (
            editor.buffer.text.cursor_to_byte(cursor),
            editor
                .buffer
                .text
                .cursor_to_byte(kjxlkj_core::Cursor::new(cursor.line, line_len)),
        ) {
            let deleted = editor.buffer.text.slice_bytes(start_byte, end_byte);
            editor.registers.yank(None, deleted, false);
            editor.buffer.text.delete_range(start_byte, end_byte);
            editor.buffer.modified = true;
            editor.buffer.clamp_cursor();
        }
    }
}

fn yank_lines(editor: &mut EditorState, count: usize) {
    let start = editor.buffer.cursor.line;
    let end = (start + count).min(editor.buffer.text.len_lines());

    let mut text = String::new();
    for i in start..end {
        if let Some(line) = editor.buffer.text.line(i) {
            text.push_str(&line);
        }
    }

    editor.registers.yank(None, text, true);
    editor.set_message(format!("{} line(s) yanked", end - start));
}

fn delete_selection(editor: &mut EditorState) {
    if let Some(sel) = editor.selection.take() {
        let start = sel.start();
        let end = sel.end();

        if let Some(deleted) = editor.buffer.text.delete_cursor_range(start, end) {
            let linewise = matches!(sel.kind, kjxlkj_core::types::SelectionKind::Line);
            editor.registers.yank(None, deleted, linewise);
        }

        editor.buffer.cursor = start;
        editor.buffer.clamp_cursor();
        editor.buffer.modified = true;
    }
    editor.mode.set_mode(Mode::Normal);
}

fn yank_selection(editor: &mut EditorState) {
    if let Some(sel) = &editor.selection {
        let start = sel.start();
        let end = sel.end();

        if let (Some(start_byte), Some(end_byte)) = (
            editor.buffer.text.cursor_to_byte(start),
            editor.buffer.text.cursor_to_byte(end),
        ) {
            let text = editor.buffer.text.slice_bytes(start_byte, end_byte);
            let linewise = matches!(sel.kind, kjxlkj_core::types::SelectionKind::Line);
            editor.registers.yank(None, text, linewise);
        }
    }
    editor.selection = None;
    editor.mode.set_mode(Mode::Normal);
}

fn paste_after(editor: &mut EditorState, count: usize) {
    if let Some(reg) = editor.registers.unnamed() {
        let text = reg.content.clone();
        let linewise = reg.linewise;

        for _ in 0..count {
            if linewise {
                // Paste on new line below.
                let line = editor.buffer.cursor.line;
                if let Some(line_content) = editor.buffer.text.line(line) {
                    let line_end = editor
                        .buffer
                        .text
                        .cursor_to_byte(kjxlkj_core::Cursor::new(line, 0))
                        .map(|b| b + line_content.len());
                    if let Some(byte) = line_end {
                        editor.buffer.text.insert(byte, &text);
                    }
                }
                editor.buffer.cursor.line += 1;
                move_to_first_non_blank(editor);
            } else {
                // Paste after cursor.
                let cursor = editor.buffer.cursor;
                let col = cursor.col + 1;
                let max_col = editor.buffer.text.line_grapheme_count(cursor.line);
                let paste_col = col.min(max_col);

                if let Some(byte) = editor
                    .buffer
                    .text
                    .cursor_to_byte(kjxlkj_core::Cursor::new(cursor.line, paste_col))
                {
                    editor.buffer.text.insert(byte, &text);
                }
                editor.buffer.cursor.col = paste_col + text.len().saturating_sub(1);
            }
        }
        editor.buffer.modified = true;
    }
}

fn paste_before(editor: &mut EditorState, count: usize) {
    if let Some(reg) = editor.registers.unnamed() {
        let text = reg.content.clone();
        let linewise = reg.linewise;

        for _ in 0..count {
            if linewise {
                // Paste on new line above.
                let cursor = editor.buffer.cursor;
                if let Some(byte) = editor
                    .buffer
                    .text
                    .cursor_to_byte(kjxlkj_core::Cursor::new(cursor.line, 0))
                {
                    editor.buffer.text.insert(byte, &text);
                }
                move_to_first_non_blank(editor);
            } else {
                // Paste before cursor.
                let cursor = editor.buffer.cursor;
                if let Some(byte) = editor.buffer.text.cursor_to_byte(cursor) {
                    editor.buffer.text.insert(byte, &text);
                }
                editor.buffer.cursor.col += text.len();
            }
        }
        editor.buffer.modified = true;
    }
}

fn undo(editor: &mut EditorState) {
    if let Some(change) = editor.buffer.undo.undo() {
        // Apply the inverted change.
        if !change.deleted.is_empty() {
            editor
                .buffer
                .text
                .delete_range(change.offset, change.offset + change.deleted.len());
        }
        if !change.inserted.is_empty() {
            editor.buffer.text.insert(change.offset, &change.inserted);
        }
        editor.buffer.cursor = change.cursor_after;
        editor.buffer.modified = true;
    } else {
        editor.set_message("Already at oldest change");
    }
}

fn redo(editor: &mut EditorState) {
    if let Some(change) = editor.buffer.undo.redo() {
        // Apply the change.
        if !change.deleted.is_empty() {
            editor
                .buffer
                .text
                .delete_range(change.offset, change.offset + change.deleted.len());
        }
        if !change.inserted.is_empty() {
            editor.buffer.text.insert(change.offset, &change.inserted);
        }
        editor.buffer.cursor = change.cursor_after;
        editor.buffer.modified = true;
    } else {
        editor.set_message("Already at newest change");
    }
}

fn execute_operator_line(editor: &mut EditorState, op: kjxlkj_core::edit::Operator, count: usize) {
    use kjxlkj_core::edit::OperatorKind;

    let start_line = editor.buffer.cursor.line;
    let end_line = (start_line + count).min(editor.buffer.text.len_lines());

    match op.kind {
        OperatorKind::Delete => {
            let mut text = String::new();
            for i in start_line..end_line {
                if let Some(line) = editor.buffer.text.line(i) {
                    text.push_str(&line);
                }
            }
            editor.registers.yank(None, text, true);

            // Delete lines.
            if let (Some(start_byte), Some(end_byte)) = (
                editor
                    .buffer
                    .text
                    .cursor_to_byte(kjxlkj_core::Cursor::new(start_line, 0)),
                if end_line < editor.buffer.text.len_lines() {
                    editor
                        .buffer
                        .text
                        .cursor_to_byte(kjxlkj_core::Cursor::new(end_line, 0))
                } else {
                    Some(editor.buffer.text.len_bytes())
                },
            ) {
                editor.buffer.text.delete_range(start_byte, end_byte);
            }

            editor.buffer.clamp_cursor();
            editor.buffer.modified = true;
        }
        OperatorKind::Yank => {
            yank_lines(editor, count);
        }
        OperatorKind::Change => {
            execute_operator_line(
                editor,
                kjxlkj_core::edit::Operator::new(OperatorKind::Delete),
                count,
            );
            open_line_above(editor);
        }
        OperatorKind::Indent => {
            for i in start_line..end_line {
                if let Some(byte) = editor
                    .buffer
                    .text
                    .cursor_to_byte(kjxlkj_core::Cursor::new(i, 0))
                {
                    editor.buffer.text.insert(byte, "    ");
                }
            }
            editor.buffer.modified = true;
        }
        OperatorKind::Outdent => {
            for i in start_line..end_line {
                if let Some(line) = editor.buffer.text.line_content(i) {
                    let spaces: usize = line.chars().take(4).take_while(|c| *c == ' ').count();
                    if spaces > 0 {
                        if let Some(byte) = editor
                            .buffer
                            .text
                            .cursor_to_byte(kjxlkj_core::Cursor::new(i, 0))
                        {
                            editor.buffer.text.delete_range(byte, byte + spaces);
                        }
                    }
                }
            }
            editor.buffer.modified = true;
        }
        _ => {}
    }
}

fn execute_operator_motion(
    editor: &mut EditorState,
    op: kjxlkj_core::edit::Operator,
    motion: kjxlkj_core::edit::Motion,
) {
    use kjxlkj_core::edit::OperatorKind;

    let start = editor.buffer.cursor;
    apply_motion(editor, &motion);
    let end = editor.buffer.cursor;

    // Determine range.
    let (range_start, range_end) = if (start.line, start.col) <= (end.line, end.col) {
        (start, end)
    } else {
        (end, start)
    };

    match op.kind {
        OperatorKind::Delete | OperatorKind::Change => {
            if let Some(deleted) = editor
                .buffer
                .text
                .delete_cursor_range(range_start, range_end)
            {
                editor.registers.yank(None, deleted, false);
            }
            editor.buffer.cursor = range_start;
            editor.buffer.clamp_cursor();
            editor.buffer.modified = true;

            if op.kind == OperatorKind::Change {
                editor.mode.set_mode(Mode::Insert);
            }
        }
        OperatorKind::Yank => {
            if let (Some(start_byte), Some(end_byte)) = (
                editor.buffer.text.cursor_to_byte(range_start),
                editor.buffer.text.cursor_to_byte(range_end),
            ) {
                let text = editor.buffer.text.slice_bytes(start_byte, end_byte);
                editor.registers.yank(None, text, false);
            }
            editor.buffer.cursor = start;
        }
        _ => {}
    }
}

async fn write_buffer(editor: &mut EditorState) {
    if let Some(ref path) = editor.buffer.path {
        let content = editor.buffer.text.to_string();
        match kjxlkj_service_fs::FsService::write_file(std::path::Path::new(path), &content).await {
            Ok(()) => {
                editor.buffer.modified = false;
                editor.set_message(format!("\"{}\" written", path));
            }
            Err(e) => {
                editor.set_error(format!("Error writing file: {}", e));
            }
        }
    } else {
        editor.set_error("No file name");
    }
}
