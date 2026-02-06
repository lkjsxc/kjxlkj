//! Autocommand event dispatch and execution.
//!
//! Fires registered `:autocmd` handlers when editor events occur.
//! Provides deterministic ordering (registration order) and group-based filtering.

use crate::autocommands::{AutoCmdTable, AutoEvent};

/// Result of firing an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFired {
    pub event: AutoEvent,
    pub filename: String,
    pub commands: Vec<String>,
}

/// Collect the commands that should fire for a given event + filename.
pub fn fire_event(table: &AutoCmdTable, event: AutoEvent, filename: &str) -> EventFired {
    let commands = table.matching(event, filename);
    EventFired { event, filename: filename.to_string(), commands }
}

/// Fire a sequence of events and collect all resulting commands.
pub fn fire_event_chain(
    table: &AutoCmdTable,
    events: &[(AutoEvent, &str)],
) -> Vec<EventFired> {
    events.iter().map(|&(ev, fname)| fire_event(table, ev, fname)).collect()
}

/// Check if any handler is registered for the given event.
pub fn has_handler(table: &AutoCmdTable, event: AutoEvent) -> bool {
    // Try matching against wildcard — if any commands come back, handlers exist
    !table.matching(event, "__probe__").is_empty()
        || !table.matching(event, "*").is_empty()
}

/// Check if any handler is registered for the given event and specific filename.
pub fn has_handler_for(table: &AutoCmdTable, event: AutoEvent, filename: &str) -> bool {
    !table.matching(event, filename).is_empty()
}

/// Map spec event names to AutoEvent variants.
pub fn parse_spec_event(name: &str) -> Option<AutoEvent> {
    match name.to_lowercase().as_str() {
        "buffer_new" | "bufnewfile" => Some(AutoEvent::BufNewFile),
        "buffer_read" | "bufread" | "bufreadpost" => Some(AutoEvent::BufRead),
        "buffer_write_pre" | "bufwritepre" | "bufwrite" => Some(AutoEvent::BufWrite),
        "buffer_write_post" | "bufwritepost" => Some(AutoEvent::BufWritePost),
        "buffer_enter" | "bufenter" => Some(AutoEvent::BufEnter),
        "buffer_leave" | "bufleave" => Some(AutoEvent::BufLeave),
        "file_type" | "filetype" => Some(AutoEvent::FileType),
        "insert_enter" | "insertenter" => Some(AutoEvent::InsertEnter),
        "insert_leave" | "insertleave" => Some(AutoEvent::InsertLeave),
        "app_enter" | "vimenter" => Some(AutoEvent::VimEnter),
        "exit_pre" | "vimleave" => Some(AutoEvent::VimLeave),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table_with_entries() -> AutoCmdTable {
        let mut t = AutoCmdTable::new();
        t.add(AutoEvent::BufRead, "*.rs", "set ft=rust", None);
        t.add(AutoEvent::BufWrite, "*", "echo 'saving'", None);
        t.add(AutoEvent::BufEnter, "*.md", "set wrap", Some("markdown"));
        t.add(AutoEvent::InsertEnter, "*", "set nocursorline", None);
        t
    }

    #[test]
    fn fire_event_matches_pattern() {
        let t = table_with_entries();
        let result = fire_event(&t, AutoEvent::BufRead, "main.rs");
        assert_eq!(result.commands, vec!["set ft=rust"]);
        assert_eq!(result.filename, "main.rs");
    }

    #[test]
    fn fire_event_no_match() {
        let t = table_with_entries();
        let result = fire_event(&t, AutoEvent::BufRead, "readme.md");
        assert!(result.commands.is_empty());
    }

    #[test]
    fn fire_event_wildcard() {
        let t = table_with_entries();
        let result = fire_event(&t, AutoEvent::BufWrite, "anything.txt");
        assert_eq!(result.commands, vec!["echo 'saving'"]);
    }

    #[test]
    fn fire_event_chain_multiple() {
        let t = table_with_entries();
        let chain = fire_event_chain(&t, &[
            (AutoEvent::BufRead, "main.rs"),
            (AutoEvent::BufEnter, "notes.md"),
        ]);
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].commands, vec!["set ft=rust"]);
        assert_eq!(chain[1].commands, vec!["set wrap"]);
    }

    #[test]
    fn has_handler_for_specific() {
        let t = table_with_entries();
        assert!(has_handler_for(&t, AutoEvent::BufEnter, "notes.md"));
        assert!(!has_handler_for(&t, AutoEvent::BufEnter, "main.rs"));
    }

    #[test]
    fn parse_spec_event_names() {
        assert_eq!(parse_spec_event("buffer_new"), Some(AutoEvent::BufNewFile));
        assert_eq!(parse_spec_event("buffer_read"), Some(AutoEvent::BufRead));
        assert_eq!(parse_spec_event("BufEnter"), Some(AutoEvent::BufEnter));
        assert_eq!(parse_spec_event("insert_enter"), Some(AutoEvent::InsertEnter));
        assert_eq!(parse_spec_event("exit_pre"), Some(AutoEvent::VimLeave));
        assert_eq!(parse_spec_event("unknown"), None);
    }

    #[test]
    fn group_clear_removes_handlers() {
        let mut t = table_with_entries();
        assert!(has_handler_for(&t, AutoEvent::BufEnter, "notes.md"));
        t.clear_group("markdown");
        assert!(!has_handler_for(&t, AutoEvent::BufEnter, "notes.md"));
    }

    #[test]
    fn clear_all_removes_everything() {
        let mut t = table_with_entries();
        t.clear_all();
        let result = fire_event(&t, AutoEvent::BufRead, "main.rs");
        assert!(result.commands.is_empty());
    }
}
