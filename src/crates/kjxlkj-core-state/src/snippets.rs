//! Snippet engine per /docs/spec/features/editing/snippets.md.
//!
//! Supports snippet expansion with tabstop placeholders.

/// A snippet definition.
#[derive(Debug, Clone)]
pub struct SnippetDef {
    /// Trigger prefix.
    pub prefix: String,
    /// Snippet body with $1, $2, $0 placeholders.
    pub body: Vec<String>,
    /// Description.
    pub description: String,
    /// Filetype scope (empty = global).
    pub filetype: Option<String>,
}

/// Parsed tabstop in a snippet.
#[derive(Debug, Clone)]
pub struct TabStop {
    /// Tabstop number (0 = final).
    pub number: u32,
    /// Default text for this tabstop.
    pub default_text: String,
    /// Line offset from snippet start.
    pub line_offset: usize,
    /// Column offset in that line.
    pub col_offset: usize,
}

/// Active snippet expansion state.
#[derive(Debug, Clone, Default)]
pub struct SnippetState {
    /// Whether a snippet is being expanded.
    pub active: bool,
    /// Tabstops in order.
    pub tabstops: Vec<TabStop>,
    /// Current tabstop index.
    pub current_stop: usize,
    /// Start line of snippet in buffer.
    pub start_line: usize,
    /// Start column of snippet in buffer.
    pub start_col: usize,
}

/// Snippet registry.
#[derive(Debug, Clone, Default)]
pub struct SnippetRegistry {
    /// Registered snippets.
    pub snippets: Vec<SnippetDef>,
}

impl SnippetRegistry {
    /// Create empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a snippet definition.
    pub fn add(&mut self, def: SnippetDef) {
        self.snippets.push(def);
    }

    /// Find snippets matching a prefix.
    pub fn find_by_prefix(
        &self,
        prefix: &str,
    ) -> Vec<&SnippetDef> {
        self.snippets
            .iter()
            .filter(|s| s.prefix.starts_with(prefix))
            .collect()
    }

    /// Find exact match for trigger.
    pub fn find_exact(
        &self,
        trigger: &str,
        ft: Option<&str>,
    ) -> Option<&SnippetDef> {
        self.snippets.iter().find(|s| {
            s.prefix == trigger
                && (s.filetype.is_none()
                    || s.filetype.as_deref() == ft)
        })
    }
}

impl SnippetState {
    /// Create new state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start snippet expansion.
    pub fn start(
        &mut self,
        tabstops: Vec<TabStop>,
        line: usize,
        col: usize,
    ) {
        self.active = true;
        self.tabstops = tabstops;
        self.current_stop = 0;
        self.start_line = line;
        self.start_col = col;
    }

    /// Move to next tabstop. Returns None when done.
    pub fn next_stop(&mut self) -> Option<&TabStop> {
        if !self.active {
            return None;
        }
        self.current_stop += 1;
        if self.current_stop >= self.tabstops.len() {
            self.active = false;
            return None;
        }
        Some(&self.tabstops[self.current_stop])
    }

    /// Move to previous tabstop.
    pub fn prev_stop(&mut self) -> Option<&TabStop> {
        if !self.active || self.current_stop == 0 {
            return None;
        }
        self.current_stop -= 1;
        Some(&self.tabstops[self.current_stop])
    }

    /// Get current tabstop.
    pub fn current(&self) -> Option<&TabStop> {
        if !self.active {
            return None;
        }
        self.tabstops.get(self.current_stop)
    }

    /// Cancel snippet.
    pub fn cancel(&mut self) {
        self.active = false;
        self.tabstops.clear();
        self.current_stop = 0;
    }
}

/// Parse snippet body into text and tabstops.
pub fn parse_snippet_body(
    body: &[String],
) -> (String, Vec<TabStop>) {
    let mut result = String::new();
    let mut stops = Vec::new();
    for (line_idx, line) in body.iter().enumerate() {
        if line_idx > 0 {
            result.push('\n');
        }
        let mut chars = line.chars().peekable();
        let mut col = 0usize;
        while let Some(c) = chars.next() {
            if c == '$' {
                if let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        let num = (chars.next().unwrap()
                            as u32)
                            - ('0' as u32);
                        stops.push(TabStop {
                            number: num,
                            default_text: String::new(),
                            line_offset: line_idx,
                            col_offset: col,
                        });
                        continue;
                    }
                }
            }
            result.push(c);
            col += 1;
        }
    }
    stops.sort_by_key(|s| {
        if s.number == 0 { u32::MAX } else { s.number }
    });
    (result, stops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_snippet() {
        let body =
            vec!["fn $1() {".into(), "    $0".into(), "}".into()];
        let (text, stops) = parse_snippet_body(&body);
        assert!(text.contains("fn "));
        assert_eq!(stops.len(), 2);
        assert_eq!(stops[0].number, 1);
        assert_eq!(stops[1].number, 0);
    }

    #[test]
    fn snippet_navigation() {
        let mut state = SnippetState::new();
        let stops = vec![
            TabStop {
                number: 1,
                default_text: String::new(),
                line_offset: 0,
                col_offset: 3,
            },
            TabStop {
                number: 2,
                default_text: String::new(),
                line_offset: 1,
                col_offset: 4,
            },
        ];
        state.start(stops, 0, 0);
        assert!(state.active);
        assert_eq!(
            state.current().unwrap().number,
            1,
        );
        state.next_stop();
        assert_eq!(
            state.current().unwrap().number,
            2,
        );
        assert!(state.next_stop().is_none());
        assert!(!state.active);
    }
}
