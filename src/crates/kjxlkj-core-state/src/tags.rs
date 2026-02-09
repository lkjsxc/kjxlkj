//! Ctags support for tag-based navigation.

use std::path::{Path, PathBuf};

/// A single tag entry (from ctags file).
#[derive(Debug, Clone)]
pub struct TagEntry {
    /// Tag name (identifier).
    pub name: String,
    /// File path where the tag is defined.
    pub file: PathBuf,
    /// Line number (1-indexed) or search pattern.
    pub address: TagAddress,
    /// Kind: function, variable, class, etc.
    pub kind: char,
}

/// Tag address can be a line number or pattern.
#[derive(Debug, Clone)]
pub enum TagAddress {
    Line(usize),
    Pattern(String),
}

/// Tag stack for navigation.
#[derive(Debug, Default)]
pub struct TagStack {
    /// Stack of (file, line, col) before jump.
    entries: Vec<(PathBuf, usize, usize)>,
    /// Current position in stack.
    pos: usize,
}

impl TagStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push current position before tag jump.
    pub fn push(&mut self, file: PathBuf, line: usize, col: usize) {
        self.entries.truncate(self.pos);
        self.entries.push((file, line, col));
        self.pos = self.entries.len();
    }

    /// Pop and return previous position.
    pub fn pop(&mut self) -> Option<(PathBuf, usize, usize)> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(self.entries[self.pos].clone())
        } else {
            None
        }
    }

    /// Check if we can go back.
    pub fn can_pop(&self) -> bool {
        self.pos > 0
    }
}

/// Parse a ctags file.
pub fn parse_tags_file(path: &Path) -> Vec<TagEntry> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut tags = Vec::new();
    for line in content.lines() {
        if line.starts_with('!') {
            continue;
        }
        if let Some(entry) = parse_tag_line(line) {
            tags.push(entry);
        }
    }
    tags
}

/// Parse a single ctags line.
fn parse_tag_line(line: &str) -> Option<TagEntry> {
    let mut parts = line.splitn(3, '\t');
    let name = parts.next()?.to_string();
    let file = PathBuf::from(parts.next()?);
    let rest = parts.next()?;
    let address = if let Some(pattern) = rest.strip_prefix("/^") {
        let end = pattern.find("$/").unwrap_or(pattern.len());
        TagAddress::Pattern(pattern[..end].to_string())
    } else {
        let num_end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let line_num = rest[..num_end].parse().unwrap_or(1);
        TagAddress::Line(line_num)
    };
    let kind = rest
        .rfind("\tkind:")
        .and_then(|i| rest[i + 6..].chars().next())
        .unwrap_or('?');
    Some(TagEntry {
        name,
        file,
        address,
        kind,
    })
}

/// Find tags matching a name.
pub fn find_tags<'a>(tags: &'a [TagEntry], name: &str) -> Vec<&'a TagEntry> {
    tags.iter().filter(|t| t.name == name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_stack_push_pop() {
        let mut stack = TagStack::new();
        stack.push(PathBuf::from("a.rs"), 10, 0);
        assert!(stack.can_pop());
        let entry = stack.pop();
        assert!(entry.is_some());
        let (f, l, _) = entry.unwrap();
        assert_eq!(f, PathBuf::from("a.rs"));
        assert_eq!(l, 10);
    }

    #[test]
    fn parse_tag_line_number() {
        let line = "main\tsrc/main.rs\t5";
        let entry = parse_tag_line(line);
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.name, "main");
    }

    #[test]
    fn find_tags_by_name() {
        let tags = vec![
            TagEntry {
                name: "foo".to_string(),
                file: PathBuf::from("a.rs"),
                address: TagAddress::Line(1),
                kind: 'f',
            },
            TagEntry {
                name: "bar".to_string(),
                file: PathBuf::from("b.rs"),
                address: TagAddress::Line(2),
                kind: 'f',
            },
        ];
        let found = find_tags(&tags, "foo");
        assert_eq!(found.len(), 1);
    }
}
