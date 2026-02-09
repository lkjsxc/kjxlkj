//! Quickfix list operations.

use kjxlkj_core_types::Action;

use crate::EditorState;

impl EditorState {
    /// Navigate to next quickfix entry (`:cnext`).
    pub(crate) fn do_cnext(&mut self) {
        if self.quickfix.is_empty() {
            return;
        }
        if self.quickfix_pos < self.quickfix.len() - 1 {
            self.quickfix_pos += 1;
        }
        self.jump_to_quickfix();
    }

    /// Navigate to previous quickfix entry (`:cprev`).
    pub(crate) fn do_cprev(&mut self) {
        if self.quickfix.is_empty() {
            return;
        }
        if self.quickfix_pos > 0 {
            self.quickfix_pos -= 1;
        }
        self.jump_to_quickfix();
    }

    /// Navigate to first quickfix entry (`:cfirst`).
    pub(crate) fn do_cfirst(&mut self) {
        if self.quickfix.is_empty() {
            return;
        }
        self.quickfix_pos = 0;
        self.jump_to_quickfix();
    }

    /// Navigate to last quickfix entry (`:clast`).
    pub(crate) fn do_clast(&mut self) {
        if self.quickfix.is_empty() {
            return;
        }
        self.quickfix_pos = self.quickfix.len() - 1;
        self.jump_to_quickfix();
    }

    /// Jump to the current quickfix entry.
    fn jump_to_quickfix(&mut self) {
        let entry = match self.quickfix.get(
            self.quickfix_pos,
        ) {
            Some(e) => e.clone(),
            None => return,
        };
        // Open file and go to line.
        let path =
            std::path::PathBuf::from(&entry.file);
        self.do_open_file(&path);
        let line = entry.line.saturating_sub(1);
        let col = entry.col.saturating_sub(1);
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line = line;
            w.cursor.grapheme_offset = col;
            w.viewport.follow_cursor(line, 3, 0);
        }
    }

    /// Load quickfix list from grep-style output.
    pub(crate) fn do_copen(&mut self, _args: &str) {
        // Open quickfix window (stub - just display
        // the list in the message area).
    }

    /// Set quickfix entries from a vector.
    pub fn set_quickfix(
        &mut self,
        entries: Vec<crate::QuickfixEntry>,
    ) {
        self.quickfix = entries;
        self.quickfix_pos = 0;
    }

    /// Parse grep output into quickfix entries.
    pub fn parse_grep_output(
        &mut self,
        output: &str,
    ) {
        let entries: Vec<crate::QuickfixEntry> = output
            .lines()
            .filter_map(|line| {
                parse_grep_line(line)
            })
            .collect();
        self.set_quickfix(entries);
    }
}

/// Parse a single grep-style line.
fn parse_grep_line(
    line: &str,
) -> Option<crate::QuickfixEntry> {
    // file:line:col:text or file:line:text
    let parts: Vec<&str> =
        line.splitn(4, ':').collect();
    if parts.len() < 3 {
        return None;
    }
    let file = parts[0].to_string();
    let line_num = parts[1].parse::<usize>().ok()?;
    let (col, text) = if parts.len() == 4 {
        let c = parts[2].parse::<usize>().unwrap_or(1);
        (c, parts[3].to_string())
    } else {
        (1, parts[2].to_string())
    };
    Some(crate::QuickfixEntry {
        file,
        line: line_num,
        col,
        kind: 'E',
        text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grep_line_valid() {
        let entry =
            parse_grep_line("main.rs:10:5:error here");
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.file, "main.rs");
        assert_eq!(e.line, 10);
        assert_eq!(e.col, 5);
        assert!(e.text.contains("error"));
    }

    #[test]
    fn quickfix_navigation() {
        let mut ed = EditorState::new(80, 24);
        ed.set_quickfix(vec![
            crate::QuickfixEntry {
                file: "a.rs".into(),
                line: 1,
                col: 1,
                kind: 'E',
                text: "err1".into(),
            },
            crate::QuickfixEntry {
                file: "b.rs".into(),
                line: 5,
                col: 3,
                kind: 'W',
                text: "warn1".into(),
            },
        ]);
        assert_eq!(ed.quickfix_pos, 0);
        ed.do_cnext();
        assert_eq!(ed.quickfix_pos, 1);
        ed.do_cprev();
        assert_eq!(ed.quickfix_pos, 0);
    }
}
