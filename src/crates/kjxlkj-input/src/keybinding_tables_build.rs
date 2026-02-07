//! Builder for the default normal-mode keybinding table.

use crate::keybinding_tables::{ActionCategory, BindingTable};
use ActionCategory::*;

/// Build the default normal-mode binding table (60+ bindings).
pub fn build_normal_table() -> BindingTable {
    let mut t = BindingTable::default();
    // Motions (26)
    for (k, a, d) in [
        ("h", "move_left", "left"),
        ("j", "move_down", "down"),
        ("k", "move_up", "up"),
        ("l", "move_right", "right"),
        ("w", "word_forward", "word fwd"),
        ("b", "word_backward", "word back"),
        ("e", "word_end", "word end"),
        ("W", "WORD_forward", "WORD fwd"),
        ("B", "WORD_backward", "WORD back"),
        ("E", "WORD_end", "WORD end"),
        ("0", "line_start", "line start"),
        ("$", "line_end", "line end"),
        ("^", "first_non_blank", "first non-blank"),
        ("gg", "goto_top", "top"),
        ("G", "goto_bottom", "bottom"),
        ("%", "match_bracket", "bracket match"),
        ("f", "find_char", "find char"),
        ("F", "find_char_back", "find back"),
        ("t", "till_char", "till char"),
        ("T", "till_char_back", "till back"),
        (";", "repeat_find", "repeat find"),
        (",", "repeat_find_rev", "reverse find"),
        ("{", "para_back", "para back"),
        ("}", "para_forward", "para fwd"),
        ("(", "sentence_back", "sentence back"),
        (")", "sentence_fwd", "sentence fwd"),
    ] {
        t.add(k, a, Motion, d);
    }
    // Operators (9)
    for (k, a, d) in [
        ("d", "delete", "delete"),
        ("c", "change", "change"),
        ("y", "yank", "yank"),
        (">", "indent", "indent"),
        ("<", "dedent", "dedent"),
        ("=", "format", "format"),
        ("gq", "wrap", "line wrap"),
        ("gu", "lowercase", "lowercase"),
        ("gU", "uppercase", "uppercase"),
    ] {
        t.add(k, a, Operator, d);
    }
    // Mode switches (11)
    for (k, a, d) in [
        ("i", "insert", "insert"),
        ("a", "append", "append"),
        ("o", "open_below", "open below"),
        ("O", "open_above", "open above"),
        ("v", "visual", "visual"),
        ("V", "visual_line", "visual line"),
        ("<C-v>", "visual_block", "v-block"),
        ("R", "replace_mode", "replace"),
        (":", "command_mode", "cmdline"),
        ("I", "insert_bol", "insert BOL"),
        ("A", "append_eol", "append EOL"),
    ] {
        t.add(k, a, ModeSwitch, d);
    }
    // Commands (10)
    for (k, a, d) in [
        ("u", "undo", "undo"),
        ("<C-r>", "redo", "redo"),
        ("p", "paste_after", "paste after"),
        ("P", "paste_before", "paste before"),
        ("x", "delete_char", "del char"),
        ("r", "replace_char", "replace char"),
        ("J", "join_lines", "join"),
        (".", "repeat", "repeat"),
        ("ZZ", "wq", "write quit"),
        ("ZQ", "q!", "force quit"),
    ] {
        t.add(k, a, Command, d);
    }
    // Search (6)
    for (k, a, d) in [
        ("/", "search_fwd", "search fwd"),
        ("?", "search_back", "search back"),
        ("n", "next_match", "next match"),
        ("N", "prev_match", "prev match"),
        ("*", "word_under_cursor", "star search"),
        ("#", "word_under_cursor_back", "hash search"),
    ] {
        t.add(k, a, Search, d);
    }
    // Scroll (7)
    for (k, a, d) in [
        ("<C-d>", "half_down", "half down"),
        ("<C-u>", "half_up", "half up"),
        ("<C-f>", "page_down", "page down"),
        ("<C-b>", "page_up", "page up"),
        ("zz", "center", "center"),
        ("zt", "scroll_top", "scroll top"),
        ("zb", "scroll_bot", "scroll bot"),
    ] {
        t.add(k, a, Scroll, d);
    }
    // Marks, registers, macros, text objects, repeat, window
    t.add("m", "set_mark", Mark, "set mark");
    t.add("'", "goto_mark", Mark, "goto mark");
    t.add("`", "goto_mark_exact", Mark, "goto mark exact");
    t.add("\"", "select_register", Register, "register");
    t.add("q", "record_macro", Macro, "record macro");
    t.add("@", "play_macro", Macro, "play macro");
    t.add("iw", "inner_word", TextObject, "inner word");
    t.add("aw", "a_word", TextObject, "a word");
    t.add("<C-w>", "window_prefix", Window, "window prefix");
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keybinding_tables::coverage_stats;

    #[test]
    fn normal_table_has_60_plus() {
        let t = build_normal_table();
        assert!(t.entries.len() >= 60, "got {}", t.entries.len());
    }

    #[test]
    fn find_by_key_works() {
        let t = build_normal_table();
        let e = t.find_by_key("j").unwrap();
        assert_eq!(e.action, "move_down");
    }

    #[test]
    fn find_by_category_works() {
        let t = build_normal_table();
        let motions = t.find_by_category(ActionCategory::Motion);
        assert!(motions.len() > 10);
    }

    #[test]
    fn coverage_stats_works() {
        let t = build_normal_table();
        let stats = coverage_stats(&t);
        assert!(stats.contains_key(&ActionCategory::Motion));
        assert!(stats[&ActionCategory::Motion] >= 20);
    }
}
