//! Snippets engine with tab-stop navigation.
//!
//! Provides a snippet registry and expansion with cursor
//! placement at $1, Tab to advance to $2, etc.

use std::collections::HashMap;

/// A snippet definition.
#[derive(Debug, Clone)]
pub struct Snippet {
    /// Trigger prefix text.
    pub trigger: String,
    /// Expansion body (may contain $1, $2 tab-stop placeholders).
    pub body: String,
    /// Optional description.
    pub description: String,
}

/// Active snippet session for tab-stop navigation.
#[derive(Debug, Clone)]
pub struct SnippetSession {
    /// Ordered tab-stop positions (offset into expansion text).
    pub stops: Vec<usize>,
    /// Current stop index.
    pub current: usize,
    /// Starting line/col of the expansion in the buffer.
    pub base_line: usize,
    pub base_col: usize,
}

/// Registry of available snippets.
#[derive(Debug, Default)]
pub struct SnippetRegistry {
    snippets: HashMap<String, Snippet>,
}

impl SnippetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, trigger: &str, body: &str, desc: &str) {
        self.snippets.insert(
            trigger.to_string(),
            Snippet {
                trigger: trigger.to_string(),
                body: body.to_string(),
                description: desc.to_string(),
            },
        );
    }

    pub fn get(&self, trigger: &str) -> Option<&Snippet> {
        self.snippets.get(trigger)
    }

    /// Expand a trigger: returns (text_with_stops_stripped, tab_stop_offsets).
    pub fn expand(&self, trigger: &str) -> Option<(String, Vec<usize>)> {
        let s = self.snippets.get(trigger)?;
        let (text, stops) = parse_tab_stops(&s.body);
        Some((text, stops))
    }

    pub fn list(&self) -> Vec<&Snippet> {
        self.snippets.values().collect()
    }
    pub fn remove(&mut self, trigger: &str) -> bool {
        self.snippets.remove(trigger).is_some()
    }
    pub fn clear(&mut self) {
        self.snippets.clear();
    }
}

/// Parse tab-stop markers ($0-$9, ${0}-${9}) from body text.
/// Returns (stripped_text, sorted_offsets_for_stops_1_through_9_then_0).
fn parse_tab_stops(body: &str) -> (String, Vec<usize>) {
    let mut out = String::with_capacity(body.len());
    let mut stops: Vec<(u8, usize)> = Vec::new();
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    chars.next();
                    stops.push((d as u8 - b'0', out.len()));
                    continue;
                }
                if d == '{' {
                    chars.next();
                    if let Some(&n) = chars.peek() {
                        if n.is_ascii_digit() {
                            chars.next();
                            if chars.peek() == Some(&'}') {
                                chars.next();
                            }
                            stops.push((n as u8 - b'0', out.len()));
                            continue;
                        }
                    }
                }
            }
        }
        out.push(c);
    }
    // Sort: $1, $2, ... $9, then $0 (end position).
    stops.sort_by_key(|&(n, _)| if n == 0 { 10 } else { n });
    let offsets = stops.into_iter().map(|(_, off)| off).collect();
    (out, offsets)
}
