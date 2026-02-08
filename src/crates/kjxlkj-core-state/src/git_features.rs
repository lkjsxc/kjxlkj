//! Git integration features per /docs/spec/features/git/.
//!
//! Gitsigns, diff display, blame, merge conflict support.

use crate::EditorState;

/// A gitsign marker for a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitSign {
    /// Line was added.
    Added,
    /// Line was modified.
    Modified,
    /// Line(s) were deleted below this line.
    Deleted,
    /// Top of a deleted block.
    TopDelete,
    /// Changed and deleted combination.
    ChangedDelete,
}

/// A merge conflict region.
#[derive(Debug, Clone)]
pub struct MergeConflict {
    /// Start line of ours marker (<<<<<<).
    pub ours_start: usize,
    /// Start line of theirs marker (>>>>>>).
    pub theirs_end: usize,
    /// Separator line (======).
    pub separator: usize,
    /// Optional base start (|||||||).
    pub base_start: Option<usize>,
}

/// Git feature state tracked in EditorState.
#[derive(Debug, Clone, Default)]
pub struct GitState {
    /// Whether gitsigns are enabled.
    pub signs_enabled: bool,
    /// Gitsigns per line (line index → sign).
    pub signs: Vec<Option<GitSign>>,
    /// Current blame annotation.
    pub blame_text: Option<String>,
    /// Whether blame is active.
    pub blame_active: bool,
    /// Detected merge conflicts.
    pub conflicts: Vec<MergeConflict>,
    /// Diff hunks summary.
    pub diff_added: usize,
    /// Lines deleted in diff.
    pub diff_deleted: usize,
    /// Lines modified in diff.
    pub diff_modified: usize,
}

impl EditorState {
    /// Toggle gitsigns display.
    pub fn do_git_signs(&mut self) {
        self.git_state.signs_enabled =
            !self.git_state.signs_enabled;
        if self.git_state.signs_enabled {
            self.refresh_git_signs();
        } else {
            self.git_state.signs.clear();
        }
    }

    /// Refresh gitsigns for the current buffer.
    fn refresh_git_signs(&mut self) {
        let Some(buf) = self.active_buffer() else {
            return;
        };
        let line_count = buf.content.line_count();
        // Initialize all lines with no sign.
        // Real implementation would compare with git index.
        self.git_state.signs =
            vec![None; line_count];
    }

    /// Show git diff for current file.
    pub fn do_git_diff(&mut self) {
        if self.active_buffer().is_none() {
            return;
        }
        // Would spawn `git diff` subprocess.
        self.git_state.diff_added = 0;
        self.git_state.diff_deleted = 0;
        self.git_state.diff_modified = 0;
    }

    /// Toggle inline blame for current line.
    pub fn do_git_blame(&mut self) {
        self.git_state.blame_active =
            !self.git_state.blame_active;
        if !self.git_state.blame_active {
            self.git_state.blame_text = None;
        }
    }

    /// Detect merge conflicts in current buffer.
    pub fn detect_merge_conflicts(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let buf = match self.buffers.get(&buf_id) {
            Some(b) => b,
            None => return,
        };
        let mut conflicts = Vec::new();
        let mut ours_start = None;
        let mut separator = None;
        for i in 0..buf.content.line_count() {
            let line = buf.content.line_str(i);
            let trimmed = line.trim();
            if trimmed.starts_with("<<<<<<<") {
                ours_start = Some(i);
            } else if trimmed.starts_with("=======")
            {
                separator = Some(i);
            } else if trimmed.starts_with(">>>>>>>") {
                if let (Some(os), Some(sep)) =
                    (ours_start, separator)
                {
                    conflicts
                        .push(MergeConflict {
                            ours_start: os,
                            theirs_end: i,
                            separator: sep,
                            base_start: None,
                        });
                }
                ours_start = None;
                separator = None;
            }
        }
        self.git_state.conflicts = conflicts;
    }
}

#[cfg(test)]
mod tests {
    use crate::EditorState;

    #[test]
    fn git_signs_toggle() {
        let mut ed = EditorState::new(80, 24);
        assert!(!ed.git_state.signs_enabled);
        ed.do_git_signs();
        assert!(ed.git_state.signs_enabled);
        ed.do_git_signs();
        assert!(!ed.git_state.signs_enabled);
    }

    #[test]
    fn git_blame_toggle() {
        let mut ed = EditorState::new(80, 24);
        ed.do_git_blame();
        assert!(ed.git_state.blame_active);
        ed.do_git_blame();
        assert!(!ed.git_state.blame_active);
    }
}
