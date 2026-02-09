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

    /// Expand trigger at a position, returning (text, SnippetSession).
    /// Session starts at first tab-stop ($1). Caller inserts text and
    /// positions cursor at `session.cursor_offset()`.
    pub fn expand_at(
        &self,
        trigger: &str,
        base_line: usize,
        base_col: usize,
    ) -> Option<(String, SnippetSession)> {
        let (text, stops) = self.expand(trigger)?;
        let session = SnippetSession {
            stops,
            current: 0,
            base_line,
            base_col,
        };
        Some((text, session))
    }
}

impl SnippetSession {
    /// Byte offset of the current tab-stop within the expanded text.
    /// Returns None if all stops are exhausted.
    pub fn current_offset(&self) -> Option<usize> {
        self.stops.get(self.current).copied()
    }
    /// Advance to next tab-stop. Returns true if there is a next stop.
    pub fn advance(&mut self) -> bool {
        if self.current + 1 < self.stops.len() {
            self.current += 1;
            true
        } else {
            false
        }
    }
}

/// Parse tab-stop markers ($0-$9, ${0}-${9}, ${N:default}) from body text.
/// Supports nested placeholders: ${1:outer ${2:inner}}.
/// Duplicate stop numbers create mirror positions.
/// Returns (stripped_text, sorted_offsets_for_stops_1_through_9_then_0).
pub fn parse_tab_stops(body: &str) -> (String, Vec<usize>) {
    let mut out = String::with_capacity(body.len());
    let mut stops: Vec<(u8, usize)> = Vec::new();
    let mut defaults: std::collections::HashMap<u8, String> = std::collections::HashMap::new();
    let mut chars = body.chars().peekable();
    parse_tab_stops_inner(&mut chars, &mut out, &mut stops, &mut defaults, false);
    // Sort: $1, $2, ... $9, then $0 (end position).
    stops.sort_by_key(|&(n, _)| if n == 0 { 10 } else { n });
    let offsets = stops.into_iter().map(|(_, off)| off).collect();
    (out, offsets)
}

#[rustfmt::skip]
fn parse_tab_stops_inner(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    out: &mut String, stops: &mut Vec<(u8, usize)>,
    defaults: &mut std::collections::HashMap<u8, String>, nested: bool,
) {
    while let Some(c) = chars.next() {
        if nested && c == '}' { return; }
        if c == '$' {
            if let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    chars.next(); let num = d as u8 - b'0';
                    if let Some(def) = defaults.get(&num) { let offset = out.len(); out.push_str(def); stops.push((num, offset)); }
                    else { stops.push((num, out.len())); }
                    continue;
                }
                if d == '{' {
                    chars.next();
                    if let Some(&n) = chars.peek() {
                        if n.is_ascii_digit() {
                            chars.next(); let stop_num = n as u8 - b'0'; let offset = out.len();
                            if chars.peek() == Some(&':') {
                                chars.next();
                                let before_len = out.len();
                                parse_tab_stops_inner(chars, out, stops, defaults, true);
                                let def_text: String = out[before_len..].to_string();
                                defaults.insert(stop_num, def_text);
                            } else if chars.peek() == Some(&'|') {
                                chars.next(); // consume '|'
                                let mut choices = Vec::new();
                                let mut cur = String::new();
                                while let Some(&ch) = chars.peek() {
                                    chars.next();
                                    if ch == '|' { if chars.peek() == Some(&'}') { chars.next(); } choices.push(std::mem::take(&mut cur)); break; }
                                    else if ch == ',' { choices.push(std::mem::take(&mut cur)); }
                                    else { cur.push(ch); }
                                }
                                if let Some(first) = choices.first() { out.push_str(first); defaults.insert(stop_num, first.clone()); }
                            } else if chars.peek() == Some(&'/') {
                                chars.next(); // consume '/' → ${N/regex/replace/flags}
                                let mut pat = String::new();
                                while let Some(&ch) = chars.peek() { chars.next(); if ch == '/' { break; } pat.push(ch); }
                                let mut rep = String::new();
                                while let Some(&ch) = chars.peek() { chars.next(); if ch == '/' { break; } rep.push(ch); }
                                let mut flags = String::new();
                                while let Some(&ch) = chars.peek() { chars.next(); if ch == '}' { break; } flags.push(ch); }
                                // Store transform as default text with placeholder for runtime application.
                                if let Some(src) = defaults.get(&stop_num) {
                                    if let Ok(re) = regex::Regex::new(&pat) {
                                        let transformed = if flags.contains('g') { re.replace_all(src, rep.as_str()).to_string() } else { re.replace(src, rep.as_str()).to_string() };
                                        out.push_str(&transformed);
                                    }
                                }
                            } else if chars.peek() == Some(&'}') {
                                chars.next();
                                if let Some(def) = defaults.get(&stop_num) { out.push_str(def); }
                            }
                            stops.push((stop_num, offset)); continue;
                        }
                    }
                }
            }
        }
        out.push(c);
    }
}
