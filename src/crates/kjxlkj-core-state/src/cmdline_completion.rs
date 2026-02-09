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
                // Command name completion.
                let prefix = content.to_string();
                let matches: Vec<String> = COMMANDS
                    .iter()
                    .filter(|c| c.starts_with(&prefix))
                    .map(|c| c.to_string())
                    .collect();
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
