//! Command-line Tab completion for ex command names.
//!
//! Pressing Tab in command mode triggers prefix-based
//! completion. Shift-Tab cycles backward.

use crate::editor::EditorState;

/// Known ex command names for completion.
const COMMANDS: &[&str] = &[
    "autocmd",
    "bdelete",
    "bnext",
    "bprev",
    "bprevious",
    "buffer",
    "close",
    "comclear",
    "command",
    "copy",
    "delete",
    "delcommand",
    "delmarks",
    "edit",
    "global",
    "help",
    "map",
    "mark",
    "marks",
    "move",
    "new",
    "nmap",
    "nnoremap",
    "nohlsearch",
    "noremap",
    "normal",
    "only",
    "quit",
    "read",
    "reg",
    "registers",
    "set",
    "sort",
    "source",
    "split",
    "substitute",
    "tabnew",
    "tabclose",
    "tabprev",
    "tabnext",
    "unmap",
    "vsplit",
    "wall",
    "write",
    "wq",
    "yank",
];

/// Completion state for the command line.
#[derive(Debug, Default)]
pub struct CompletionState {
    /// Current candidates matching the prefix.
    pub candidates: Vec<String>,
    /// Index of the currently selected candidate.
    pub index: Option<usize>,
    /// The original prefix the user typed.
    pub prefix: String,
}

impl CompletionState {
    pub fn clear(&mut self) {
        self.candidates.clear();
        self.index = None;
        self.prefix.clear();
    }
}

impl EditorState {
    /// Handle Tab in command mode for completion.
    pub(crate) fn cmdline_complete_next(&mut self) {
        if self.cmdline.prefix != Some(':') {
            return;
        }
        let content = &self.cmdline.content;
        let cs = &mut self.cmdline.completion;
        if cs.candidates.is_empty() {
            if content.contains(' ') {
                // Context-aware completion after a space.
                self.build_arg_candidates();
            } else {
                // Command name completion with priority ordering:
                // 1. Exact prefix (builtin) > 2. Exact prefix (user) > 3. Fuzzy (builtin) > 4. Fuzzy (user).
                let prefix = content.to_string();
                let mut pri_matches: Vec<(u8, String)> = Vec::new();
                for c in COMMANDS.iter() {
                    if c.starts_with(&prefix) { pri_matches.push((0, c.to_string())); }
                }
                for cmd in self.user_commands.list() {
                    if cmd.name.starts_with(&prefix) { pri_matches.push((1, cmd.name.clone())); }
                }
                if pri_matches.is_empty() {
                    for (s, name) in fuzzy_filter_scored(&prefix, COMMANDS) { pri_matches.push((2 + (100 - s.min(100) as u8), name)); }
                    for cmd in self.user_commands.list() {
                        if fuzzy_matches(&prefix, &cmd.name) { pri_matches.push((3, cmd.name.clone())); }
                    }
                }
                pri_matches.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
                pri_matches.dedup_by(|a, b| a.1 == b.1);
                let matches: Vec<String> = pri_matches.into_iter().map(|(_, n)| n).collect();
                if matches.is_empty() {
                    return;
                }
                let cs = &mut self.cmdline.completion;
                cs.prefix = prefix;
                cs.candidates = matches;
                cs.index = Some(0);
            }
        } else {
            // Cycle forward.
            let next = match cs.index {
                Some(i) => (i + 1) % cs.candidates.len(),
                None => 0,
            };
            cs.index = Some(next);
        }
        let cs = &self.cmdline.completion;
        if let Some(idx) = cs.index {
            if cs.prefix.contains(' ') {
                // File completion: replace only the path part.
                let pfx_space = cs.prefix.rfind(' ').unwrap_or(0) + 1;
                let before = &cs.prefix[..pfx_space];
                let word = &cs.candidates[idx];
                self.cmdline.content = format!("{}{}", before, word);
            } else {
                let word = cs.candidates[idx].clone();
                self.cmdline.content = word;
            }
            self.cmdline.cursor_pos = self.cmdline.content.len();
        }
    }

    /// Handle Shift-Tab in command mode (cycle backward).
    pub(crate) fn cmdline_complete_prev(&mut self) {
        let cs = &mut self.cmdline.completion;
        if cs.candidates.is_empty() {
            return;
        }
        let prev = match cs.index {
            Some(0) => cs.candidates.len() - 1,
            Some(i) => i - 1,
            None => cs.candidates.len() - 1,
        };
        cs.index = Some(prev);
        let word = cs.candidates[prev].clone();
        self.cmdline.content = word;
        self.cmdline.cursor_pos = self.cmdline.content.len();
    }

    /// Reset completion when user types a new character.
    pub(crate) fn cmdline_reset_completion(&mut self) {
        self.cmdline.completion.clear();
    }
}

/// Check if `needle` fuzzy-matches `haystack`: all chars appear in order.
pub(crate) fn fuzzy_matches(needle: &str, haystack: &str) -> bool {
    let mut hi = haystack.chars();
    for nc in needle.chars() {
        let nc_lower = nc.to_ascii_lowercase();
        loop {
            match hi.next() {
                Some(hc) if hc.to_ascii_lowercase() == nc_lower => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

/// Filter command list by fuzzy matching, ranked by score (highest first).
#[rustfmt::skip]
#[allow(dead_code)]
fn fuzzy_filter(needle: &str, commands: &[&str]) -> Vec<String> {
    fuzzy_filter_scored(needle, commands).into_iter().map(|(_, c)| c).collect()
}

/// Filter commands by fuzzy matching, returning (score, name) sorted by score descending.
#[rustfmt::skip]
fn fuzzy_filter_scored(needle: &str, commands: &[&str]) -> Vec<(i32, String)> {
    use crate::cmdline_completion_ctx::fuzzy_score;
    let mut scored: Vec<_> = commands.iter().filter_map(|c| fuzzy_score(needle, c).map(|s| (s, c.to_string()))).collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored
}
