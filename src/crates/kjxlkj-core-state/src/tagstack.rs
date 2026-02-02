//! Tag stack for tag-based navigation.
//!
//! Supports :tag, :pop, Ctrl-], Ctrl-T style navigation.

use crate::tag_types::TagEntry;

/// Maximum tag stack depth.
const MAX_TAG_STACK: usize = 20;

/// Tag stack for navigation.
#[derive(Debug, Default)]
pub struct TagStack {
    /// Stack entries.
    entries: Vec<TagEntry>,
    /// Current position in stack.
    index: usize,
}

impl TagStack {
    /// Creates a new tag stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a tag jump onto the stack.
    pub fn push(&mut self, entry: TagEntry) {
        if self.index < self.entries.len() {
            self.entries.truncate(self.index);
        }

        self.entries.push(entry);
        self.index = self.entries.len();

        if self.entries.len() > MAX_TAG_STACK {
            self.entries.remove(0);
            self.index = self.entries.len();
        }
    }

    /// Pops back to previous location (Ctrl-T).
    pub fn pop(&mut self) -> Option<&TagEntry> {
        if self.index > 0 {
            self.index -= 1;
            self.entries.get(self.index)
        } else {
            None
        }
    }

    /// Gets current entry.
    pub fn current(&self) -> Option<&TagEntry> {
        if self.index > 0 {
            self.entries.get(self.index - 1)
        } else {
            None
        }
    }

    /// Gets all entries.
    pub fn entries(&self) -> &[TagEntry] {
        &self.entries
    }

    /// Gets stack depth.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clears the stack.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag_types::{TagDef, TagLocation, TagMatch};
    use kjxlkj_core_types::{BufferId, Position};
    use std::path::PathBuf;

    fn make_pos(line: usize, col: usize) -> Position {
        Position::new(line, col)
    }

    #[test]
    fn test_tag_entry_new() {
        let entry = TagEntry::new(
            "foo".to_string(),
            BufferId::new(1),
            make_pos(10, 0),
            BufferId::new(2),
            make_pos(100, 5),
        );
        assert_eq!(entry.tag, "foo");
        assert_eq!(entry.from_buffer, BufferId::new(1));
        assert_eq!(entry.to_buffer, BufferId::new(2));
    }

    #[test]
    fn test_tag_stack_push_pop() {
        let mut stack = TagStack::new();
        let entry = TagEntry::new(
            "foo".to_string(),
            BufferId::new(1),
            make_pos(0, 0),
            BufferId::new(2),
            make_pos(50, 0),
        );
        stack.push(entry);
        assert_eq!(stack.len(), 1);
        let popped = stack.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().tag, "foo");
    }

    #[test]
    fn test_tag_stack_empty() {
        let mut stack = TagStack::new();
        assert!(stack.is_empty());
        assert!(stack.pop().is_none());
    }

    #[test]
    fn test_tag_def() {
        let tag = TagDef::new(
            "MyClass".to_string(),
            PathBuf::from("src/main.rs"),
            TagLocation::Line(42),
        )
        .with_kind("class".to_string());

        assert_eq!(tag.name, "MyClass");
        assert_eq!(tag.kind, Some("class".to_string()));
    }

    #[test]
    fn test_tag_match_navigation() {
        let tags = vec![
            TagDef::new("foo".to_string(), PathBuf::from("a.rs"), TagLocation::Line(1)),
            TagDef::new("foo".to_string(), PathBuf::from("b.rs"), TagLocation::Line(2)),
            TagDef::new("foo".to_string(), PathBuf::from("c.rs"), TagLocation::Line(3)),
        ];
        let mut matcher = TagMatch::from_tags(tags);

        assert_eq!(matcher.current().unwrap().file, PathBuf::from("a.rs"));
        matcher.advance_next();
        assert_eq!(matcher.current().unwrap().file, PathBuf::from("b.rs"));
        matcher.prev();
        assert_eq!(matcher.current().unwrap().file, PathBuf::from("a.rs"));
    }

    #[test]
    fn test_tag_location_variants() {
        let line_loc = TagLocation::Line(42);
        let pattern_loc = TagLocation::Pattern("/^fn foo/".to_string());
        assert_eq!(line_loc, TagLocation::Line(42));
        assert_ne!(line_loc, pattern_loc);
    }
}
