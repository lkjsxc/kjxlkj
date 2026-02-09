/// Jump list display and :nohlsearch command handlers.
use crate::editor::EditorState;

impl EditorState {
    /// Handle :jumps — display jump list.
    pub(crate) fn handle_list_jumps(&mut self) {
        if self.jumplist.is_empty() {
            self.notify_info("Jump list is empty");
            return;
        }
        let mut lines = vec!["--- Jump List ---".to_string()];
        for (i, &(bid, line, col)) in self.jumplist.iter().enumerate() {
            let marker = if i == self.jumplist_idx { ">" } else { " " };
            lines.push(format!("{marker} {i}: ({bid}) {line}:{col}"));
        }
        self.notify_info(&lines.join("\n"));
    }

    /// Handle :nohlsearch / :noh — clear search highlight.
    pub(crate) fn handle_nohlsearch(&mut self) {
        self.search.active = false;
    }

    /// Dispatch echo/echon/echomsg/echohl/echoerr commands.
    #[rustfmt::skip]
    pub(crate) fn dispatch_echo(&mut self, rest: &str) {
        if let Some(msg) = rest.strip_prefix("echoerr ") { self.notify_error(msg.trim().trim_matches('"')); }
        else if let Some(hl) = rest.strip_prefix("echohl ") { self.options.set("echohl", crate::options::OptionValue::Str(hl.trim().to_string())); }
        else if rest == "echohl" { self.options.set("echohl", crate::options::OptionValue::Str(String::new())); }
        else if let Some(msg) = rest.strip_prefix("echomsg ") { self.notify_info(msg.trim().trim_matches('"')); }
        else if let Some(msg) = rest.strip_prefix("echon ") { self.notify_info(msg.trim().trim_matches('"')); }
        else { self.notify_info(rest.strip_prefix("echo").unwrap_or("").trim().trim_matches('"')); }
    }

    /// Handle `:for var in list` — start accumulating loop body.
    #[rustfmt::skip]
    pub(crate) fn handle_for_start(&mut self, cmd: &str) {
        let rest = cmd.strip_prefix("for ").unwrap_or("").trim();
        if let Some((var, list_expr)) = rest.split_once(" in ") {
            self.for_loop_acc = Some(crate::editor::ForLoopAcc { var: var.trim().to_string(), list_expr: list_expr.trim().to_string(), body: Vec::new() });
        } else { self.notify_error("E690: Missing \"in\" after :for"); }
    }

    /// Execute accumulated :for loop body for each item in list.
    #[rustfmt::skip]
    pub(crate) fn execute_for_loop(&mut self, var: &str, list_expr: &str, body: &[String]) {
        let list_val = crate::expr_eval::eval_expression(list_expr).unwrap_or_default();
        let inner = list_val.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or(&list_val);
        if inner.is_empty() { return; }
        let items: Vec<&str> = inner.split(',').map(|s| s.trim().trim_matches('"')).collect();
        for item in items {
            self.options.set(var, crate::options::OptionValue::Str(item.to_string()));
            for line in body { self.execute_ex_command(line); }
        }
    }

    /// Handle `:normal[!] {keys}` — execute keys as if typed in normal mode.
    /// With `!`, no mappings are applied.
    pub(crate) fn handle_normal_command(&mut self, cmd: &str) {
        let rest = cmd.strip_prefix("normal!").or_else(|| cmd.strip_prefix("norm!"))
            .or_else(|| cmd.strip_prefix("normal")).or_else(|| cmd.strip_prefix("norm"))
            .unwrap_or("").trim();
        if rest.is_empty() { return; }
        let saved_mode = self.mode.clone();
        self.mode = kjxlkj_core_types::Mode::Normal;
        for ch in rest.chars() {
            let key = kjxlkj_core_types::Key::char(ch);
            self.handle_key(key);
        }
        if matches!(self.mode, kjxlkj_core_types::Mode::Normal) { self.mode = saved_mode; }
    }

    /// Parse modeline from buffer first/last 5 lines.
    /// Supports multiple options: `vim: set ts=4 sw=4 et:` or `vim: ts=4:sw=4:et`.
    pub(crate) fn parse_modeline(&mut self) {
        let buf_id = self.current_buffer_id();
        let text = match self.buffers.get(buf_id) {
            Some(b) => b.content.to_string(),
            None => return,
        };
        let lines: Vec<&str> = text.lines().collect();
        let n = lines.len();
        let check_lines: Vec<&str> = if n <= 10 {
            lines.clone()
        } else {
            let mut v: Vec<&str> = lines[..5].to_vec();
            v.extend_from_slice(&lines[n - 5..]);
            v
        };
        for line in check_lines {
            if let Some(opts) = extract_modeline(line) {
                // Split on whitespace and colons to handle both
                // "ts=4 sw=4 et" and "ts=4:sw=4:et" styles.
                for part in opts.split(|c: char| c.is_whitespace() || c == ':') {
                    let part = part.trim();
                    if part.is_empty() { continue; }
                    let _ = crate::options::parse_set_command(&mut self.options, part);
                }
            }
        }
    }
}

/// Extract options string from a modeline comment.
/// Matches: `vim:` or `vi:` with `set ` prefix or bare options.
/// Returns the FULL options string (caller splits on whitespace/colons).
#[rustfmt::skip]
fn extract_modeline(line: &str) -> Option<&str> {
    for prefix in ["vim:", "vi:", "ex:"] {
        if let Some(idx) = line.find(prefix) {
            let rest = &line[idx + prefix.len()..];
            let rest = rest.trim();
            if let Some(s) = rest.strip_prefix("set ") { return Some(s.trim()); }
            return Some(rest);
        }
    }
    None
}
