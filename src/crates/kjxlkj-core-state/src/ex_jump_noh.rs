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
