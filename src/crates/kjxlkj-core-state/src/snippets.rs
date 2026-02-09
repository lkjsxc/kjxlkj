//! Snippet engine.

#[derive(Debug, Clone)]
pub struct SnippetDef {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: String,
    pub filetype: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TabStop {
    pub number: u32,
    pub default_text: String,
    pub line_offset: usize,
    pub col_offset: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SnippetState {
    pub active: bool,
    pub tabstops: Vec<TabStop>,
    pub current_stop: usize,
    pub start_line: usize,
    pub start_col: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SnippetRegistry {
    pub snippets: Vec<SnippetDef>,
}

impl SnippetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, def: SnippetDef) {
        self.snippets.push(def);
    }

    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&SnippetDef> {
        self.snippets
            .iter()
            .filter(|s| s.prefix.starts_with(prefix))
            .collect()
    }

    pub fn find_exact(&self, trigger: &str, ft: Option<&str>) -> Option<&SnippetDef> {
        self.snippets
            .iter()
            .find(|s| s.prefix == trigger && (s.filetype.is_none() || s.filetype.as_deref() == ft))
    }
}

impl SnippetState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, tabstops: Vec<TabStop>, line: usize, col: usize) {
        self.active = true;
        self.tabstops = tabstops;
        self.current_stop = 0;
        self.start_line = line;
        self.start_col = col;
    }

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

    pub fn prev_stop(&mut self) -> Option<&TabStop> {
        if !self.active || self.current_stop == 0 {
            return None;
        }
        self.current_stop -= 1;
        Some(&self.tabstops[self.current_stop])
    }

    pub fn current(&self) -> Option<&TabStop> {
        if !self.active {
            return None;
        }
        self.tabstops.get(self.current_stop)
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.tabstops.clear();
        self.current_stop = 0;
    }
}

pub fn parse_snippet_body(body: &[String]) -> (String, Vec<TabStop>) {
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
                        let num = (chars.next().unwrap() as u32) - ('0' as u32);
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
    stops.sort_by_key(|s| if s.number == 0 { u32::MAX } else { s.number });
    (result, stops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_snippet() {
        let body = vec!["fn $1() {".into(), "    $0".into(), "}".into()];
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
        assert_eq!(state.current().unwrap().number, 1);
        state.next_stop();
        assert_eq!(state.current().unwrap().number, 2);
        assert!(state.next_stop().is_none());
        assert!(!state.active);
    }
}
