//! Per-mode keybinding tables.

use serde::{Deserialize, Serialize};

/// High-level UX mode names (mirrors `Mode` but oriented toward UX/display).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UxMode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
    Replace,
    Command,
    OperatorPending,
    Terminal,
}

/// A single keybinding entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UxBinding {
    pub key: String,
    pub action: String,
    pub description: String,
}

/// A list of bindings for a mode.
pub type ModeBindingTable = Vec<UxBinding>;

/// Helper to create a `UxBinding`.
fn bind(key: &str, action: &str, desc: &str) -> UxBinding {
    UxBinding {
        key: key.to_string(),
        action: action.to_string(),
        description: desc.to_string(),
    }
}

/// Build the default Normal-mode binding table (25+ bindings).
pub fn build_normal_bindings() -> ModeBindingTable {
    vec![
        bind("h", "move_left", "Move cursor left"),
        bind("j", "move_down", "Move cursor down"),
        bind("k", "move_up", "Move cursor up"),
        bind("l", "move_right", "Move cursor right"),
        bind("0", "line_start", "Move to start of line"),
        bind("$", "line_end", "Move to end of line"),
        bind("^", "first_non_blank", "Move to first non-blank"),
        bind("w", "word_forward", "Move to next word start"),
        bind("b", "word_backward", "Move to previous word start"),
        bind("e", "word_end", "Move to end of word"),
        bind("W", "big_word_forward", "Move to next WORD start"),
        bind("B", "big_word_backward", "Move to previous WORD start"),
        bind("E", "big_word_end", "Move to end of WORD"),
        bind("i", "enter_insert", "Enter Insert mode before cursor"),
        bind("a", "enter_insert_after", "Enter Insert mode after cursor"),
        bind("o", "open_below", "Open line below and enter Insert"),
        bind("O", "open_above", "Open line above and enter Insert"),
        bind("v", "enter_visual", "Enter Visual mode"),
        bind("V", "enter_visual_line", "Enter Visual-Line mode"),
        bind("Ctrl-v", "enter_visual_block", "Enter Visual-Block mode"),
        bind("R", "enter_replace", "Enter Replace mode"),
        bind(":", "enter_command", "Enter Command-line mode"),
        bind("dd", "delete_line", "Delete current line"),
        bind("yy", "yank_line", "Yank current line"),
        bind("cc", "change_line", "Change current line"),
        bind("x", "delete_char", "Delete character under cursor"),
        bind("p", "put_after", "Put after cursor"),
        bind("P", "put_before", "Put before cursor"),
        bind("u", "undo", "Undo last change"),
        bind("Ctrl-r", "redo", "Redo last undone change"),
        bind("gg", "go_file_start", "Go to first line"),
        bind("G", "go_file_end", "Go to last line"),
        bind("/", "search_forward", "Search forward"),
        bind("?", "search_backward", "Search backward"),
        bind("n", "search_next", "Next search match"),
        bind("N", "search_prev", "Previous search match"),
        bind(".", "repeat_change", "Repeat last change"),
    ]
}

/// Check that `table` has at least `expected_count` bindings.
pub fn check_mode_coverage(table: &ModeBindingTable, expected_count: usize) -> bool {
    table.len() >= expected_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_bindings_has_enough() {
        let table = build_normal_bindings();
        assert!(check_mode_coverage(&table, 25));
    }

    #[test]
    fn binding_lookup() {
        let table = build_normal_bindings();
        let h = table.iter().find(|b| b.key == "h");
        assert!(h.is_some());
        assert_eq!(h.unwrap().action, "move_left");
    }

    #[test]
    fn no_duplicate_keys() {
        let table = build_normal_bindings();
        let mut keys: Vec<&str> = table.iter().map(|b| b.key.as_str()).collect();
        let len_before = keys.len();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), len_before, "duplicate keys found");
    }
}
