//! Search dispatch: forward/backward search, next/prev, word search.
//! Supports both literal and regex patterns (Vim-style magic mode).

use crate::EditorState;
use kjxlkj_core_types::Position;
use regex::Regex;

/// Convert a Vim-style search pattern to a Rust regex.
/// Supports basic magic-mode metacharacters: ., *, ^, $, [], \d, \w, \s, \+, \?, \|, \(, \).
fn vim_pattern_to_regex(pattern: &str) -> Option<Regex> {
    // If pattern contains no regex metacharacters, do literal search
    let has_meta = pattern.chars().any(|c| matches!(c, '.' | '*' | '^' | '$' | '[' | '\\'));
    if !has_meta { return None; }
    // Convert Vim magic-mode pattern to Rust regex
    let mut out = String::with_capacity(pattern.len() + 8);
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                'd' => { out.push_str(r"\d"); i += 2; }
                'w' => { out.push_str(r"\w"); i += 2; }
                's' => { out.push_str(r"\s"); i += 2; }
                'n' => { out.push('\n'); i += 2; }
                't' => { out.push('\t'); i += 2; }
                '+' => { out.push('+'); i += 2; }
                '?' => { out.push('?'); i += 2; }
                '|' => { out.push('|'); i += 2; }
                '(' => { out.push('('); i += 2; }
                ')' => { out.push(')'); i += 2; }
                '<' => { out.push_str(r"\b"); i += 2; }
                '>' => { out.push_str(r"\b"); i += 2; }
                c => { out.push('\\'); out.push(c); i += 2; }
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    Regex::new(&out).ok()
}

/// Search forward from current position for `pattern`.
pub(crate) fn dispatch_search_forward(state: &mut EditorState, pattern: &str) {
    if pattern.is_empty() { return; }
    state.search_pattern = Some(pattern.to_string());
    state.search_forward = true;
    search_next_impl(state, true);
}

/// Search backward from current position for `pattern`.
pub(crate) fn dispatch_search_backward(state: &mut EditorState, pattern: &str) {
    if pattern.is_empty() { return; }
    state.search_pattern = Some(pattern.to_string());
    state.search_forward = false;
    search_next_impl(state, false);
}

/// Repeat search in same direction.
pub(crate) fn dispatch_search_next(state: &mut EditorState) {
    let forward = state.search_forward;
    search_next_impl(state, forward);
}

/// Repeat search in opposite direction.
pub(crate) fn dispatch_search_prev(state: &mut EditorState) {
    let forward = !state.search_forward;
    search_next_impl(state, forward);
}

/// Search for word under cursor forward (with word boundaries).
pub(crate) fn dispatch_search_word_forward(state: &mut EditorState) {
    if let Some(word) = word_under_cursor(state) {
        let pat = format!(r"\<{}\>", regex::escape(&word));
        state.search_pattern = Some(pat);
        state.search_forward = true;
        search_next_impl(state, true);
    }
}

/// Search for word under cursor backward (with word boundaries).
pub(crate) fn dispatch_search_word_backward(state: &mut EditorState) {
    if let Some(word) = word_under_cursor(state) {
        let pat = format!(r"\<{}\>", regex::escape(&word));
        state.search_pattern = Some(pat);
        state.search_forward = false;
        search_next_impl(state, false);
    }
}

/// Get the word under the cursor.
fn word_under_cursor(state: &EditorState) -> Option<String> {
    let wid = state.active_window?;
    let win = state.windows.get(&wid)?;
    let buf = state.buffers.get(&win.buffer_id)?;
    let line = buf.text.line_to_string(win.cursor_line);
    let chars: Vec<char> = line.chars().collect();
    if win.cursor_col >= chars.len() { return None; }
    if !chars[win.cursor_col].is_alphanumeric() && chars[win.cursor_col] != '_' { return None; }
    let mut start = win.cursor_col;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') { start -= 1; }
    let mut end = win.cursor_col;
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') { end += 1; }
    Some(chars[start..end].iter().collect())
}

/// Core search: find next occurrence (regex or literal) in direction.
fn search_next_impl(state: &mut EditorState, forward: bool) {
    let pattern = match &state.search_pattern {
        Some(p) => p.clone(),
        None => { state.message = Some("No previous search".into()); return; }
    };
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (cur_line, cur_col) = (win.cursor_line, win.cursor_col);
    let buf = match state.buffers.get(&bid) { Some(b) => b, None => return };
    let line_count = buf.text.line_count();
    let re = vim_pattern_to_regex(&pattern);
    let case_insensitive = state.options.ignorecase
        && !(state.options.smartcase && pattern.chars().any(|c| c.is_uppercase()));
    // Build a case-insensitive regex if needed and no explicit regex
    let re = re.or_else(|| {
        if case_insensitive {
            Regex::new(&format!("(?i){}", regex::escape(&pattern))).ok()
        } else { None }
    });

    if forward {
        for offset in 0..line_count {
            let idx = (cur_line + offset) % line_count;
            let text = buf.text.line_to_string(idx);
            let start_col = if offset == 0 { cur_col + 1 } else { 0 };
            let found = if let Some(ref re) = re {
                find_regex_in_line(re, &text, start_col)
            } else {
                find_in_line(&text, &pattern, start_col)
            };
            if let Some(col) = found {
                let win = state.windows.get_mut(&wid).unwrap();
                win.set_cursor(Position::new(idx, col));
                win.ensure_cursor_visible();
                if offset >= line_count / 2 { state.message = Some("search wrapped".into()); }
                return;
            }
        }
    } else {
        for offset in 0..line_count {
            let idx = if offset <= cur_line { cur_line - offset } else { line_count - (offset - cur_line) };
            let text = buf.text.line_to_string(idx);
            let max = if offset == 0 { cur_col.saturating_sub(1) } else { text.len() };
            let found = if let Some(ref re) = re {
                rfind_regex_in_line(re, &text, max)
            } else {
                rfind_in_line(&text, &pattern, max)
            };
            if let Some(col) = found {
                let win = state.windows.get_mut(&wid).unwrap();
                win.set_cursor(Position::new(idx, col));
                win.ensure_cursor_visible();
                if offset >= line_count / 2 { state.message = Some("search wrapped".into()); }
                return;
            }
        }
    }
    state.message = Some(format!("Pattern not found: {}", pattern));
}

/// Find literal pattern in a line starting from `from_col`.
fn find_in_line(line: &str, pattern: &str, from_col: usize) -> Option<usize> {
    if from_col >= line.len() { return None; }
    line[from_col..].find(pattern).map(|i| i + from_col)
}

/// Find regex match in a line starting from `from_col`.
fn find_regex_in_line(re: &Regex, line: &str, from_col: usize) -> Option<usize> {
    if from_col >= line.len() { return None; }
    re.find(&line[from_col..]).map(|m| m.start() + from_col)
}

/// Reverse-find literal pattern in a line, ending before `max_col`.
fn rfind_in_line(line: &str, pattern: &str, max_col: usize) -> Option<usize> {
    let end = max_col.min(line.len());
    if end == 0 { return None; }
    line[..end].rfind(pattern)
}

/// Reverse-find regex match in a line, ending before `max_col`.
fn rfind_regex_in_line(re: &Regex, line: &str, max_col: usize) -> Option<usize> {
    let end = max_col.min(line.len());
    if end == 0 { return None; }
    let slice = &line[..end];
    let mut last = None;
    for m in re.find_iter(slice) { last = Some(m.start()); }
    last
}
