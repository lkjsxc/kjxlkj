//! Command-line helpers: tab completion, char append,
//! history navigation for EditorState.

use kjxlkj_core_types::Direction;

use crate::EditorState;

/// Known ex commands for tab completion.
const COMMANDS: &[&str] = &[
    "buffer",
    "bdelete",
    "bnext",
    "bprevious",
    "close",
    "cquit",
    "edit",
    "find",
    "global",
    "help",
    "history",
    "ls",
    "buffers",
    "marks",
    "new",
    "nohlsearch",
    "only",
    "quit",
    "qall",
    "read",
    "registers",
    "set",
    "source",
    "split",
    "substitute",
    "tab",
    "tabnew",
    "terminal",
    "vglobal",
    "vnew",
    "vsplit",
    "wall",
    "write",
    "wq",
    "wqa",
];

impl EditorState {
    /// Append a character to the command line buffer.
    pub(crate) fn do_cmdline_char(&mut self, ch: char) {
        if let Some(cs) = &mut self.command_state {
            cs.buffer.insert(cs.cursor, ch);
            cs.cursor += ch.len_utf8();
        }
    }

    /// Backspace in the command line buffer.
    pub(crate) fn do_cmdline_backspace(&mut self) {
        if let Some(cs) = &mut self.command_state {
            if cs.cursor > 0 {
                let prev = cs.buffer[..cs.cursor]
                    .char_indices()
                    .last()
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                cs.buffer.drain(prev..cs.cursor);
                cs.cursor = prev;
            }
        }
    }

    /// Navigate command history up/down.
    pub(crate) fn do_cmdline_history(&mut self, dir: Direction) {
        if let Some(cs) = &mut self.command_state {
            cs.navigate_history(dir);
        }
    }

    /// Tab completion: complete the current command
    /// word against known command names.
    pub(crate) fn do_cmdline_complete(&mut self) {
        let prefix = match &self.command_state {
            Some(cs) => cs.buffer.clone(),
            None => return,
        };
        let prefix = prefix.trim().to_string();
        if prefix.is_empty() {
            return;
        }

        // Find matching commands.
        let matches: Vec<&str> = COMMANDS
            .iter()
            .filter(|c| c.starts_with(&prefix))
            .copied()
            .collect();

        if matches.len() == 1 {
            if let Some(cs) = &mut self.command_state {
                cs.buffer = matches[0].to_string();
                cs.cursor = cs.buffer.len();
            }
        } else if matches.len() > 1 {
            // Complete to longest common prefix.
            let lcp = longest_common_prefix(&matches);
            if lcp.len() > prefix.len() {
                if let Some(cs) = &mut self.command_state {
                    cs.buffer = lcp;
                    cs.cursor = cs.buffer.len();
                }
            }
        }
    }
}

/// Find longest common prefix among a set of strings.
fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = strs[0];
    let mut len = first.len();
    for s in &strs[1..] {
        len = len.min(s.len());
        for (i, (a, b)) in first.bytes().zip(s.bytes()).enumerate() {
            if a != b {
                len = len.min(i);
                break;
            }
        }
    }
    first[..len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Mode;

    #[test]
    fn tab_completes_unique() {
        let mut ed = EditorState::new(80, 24);
        ed.enter_command(kjxlkj_core_types::ActionCommandKind::Ex);
        if let Some(cs) = &mut ed.command_state {
            cs.buffer = "termi".to_string();
            cs.cursor = 5;
        }
        ed.do_cmdline_complete();
        let buf = ed.command_state.as_ref().unwrap().buffer.clone();
        assert_eq!(buf, "terminal");
    }

    #[test]
    fn tab_completes_prefix() {
        let mut ed = EditorState::new(80, 24);
        ed.enter_command(kjxlkj_core_types::ActionCommandKind::Ex);
        if let Some(cs) = &mut ed.command_state {
            cs.buffer = "w".to_string();
            cs.cursor = 1;
        }
        ed.do_cmdline_complete();
        let buf = ed.command_state.as_ref().unwrap().buffer.clone();
        // "w" matches "wall", "write", "wq", "wqa" → lcp = "w"
        assert!(buf.starts_with("w"));
    }

    #[test]
    fn longest_common_prefix_works() {
        assert_eq!(longest_common_prefix(&["write", "wq", "wqa"]), "w");
        assert_eq!(longest_common_prefix(&["split", "substitute"]), "s");
    }
}
